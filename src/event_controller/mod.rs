mod status_bar;
pub mod style;
pub mod view;
pub mod window;

pub use self::style::Styles;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use self::status_bar::StatusBar;
use self::style::{RGBColor, StyleID};
use self::view::{View, ViewID};
use self::window::Layout;

use serde_json::Value;
use xi_rpc::{RemoteError, RpcCall, RpcCtx};

#[derive(Deserialize, Debug)]
pub struct LineDescription {
    cursor: Option<Vec<i32>>,
    ln: Option<usize>,
    styles: Vec<StyleID>,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Operation {
    #[serde(rename = "op")]
    kind: String,
    n: usize,
    ln: Option<usize>,
    lines: Option<Vec<LineDescription>>,
}

#[derive(Deserialize, Debug)]
pub struct Annotation {
    #[serde(rename = "type")]
    kind: String,
    n: usize,
    payloads: Option<()>,
    ranges: Vec<[usize; 4]>,
}

#[derive(Default, Clone)]
pub struct Cursor {
    pub y: u32,
    pub x: u32,
}

#[derive(Default, Clone)]
pub struct Line {
    pub raw: String,
    pub styles: Vec<StyleID>,
    /// The "real" line number.
    ///
    /// A line wrapped in two lines will keep the same `ln` value.
    pub ln: usize,
    /// Indicate if the line needs to be rendered during the next redraw.
    pub is_dirty: bool,
}

pub struct EventController {
    styles: Rc<RefCell<Box<dyn Styles>>>,
    views: HashMap<ViewID, View>,
    layout: Box<dyn Layout>,
    status_bar: StatusBar,
    current_view: String,
}

impl xi_rpc::Handler for EventController {
    type Notification = RpcCall;
    type Request = RpcCall;

    fn handle_notification(&mut self, ctx: &RpcCtx, rpc: Self::Notification) {
        match rpc.method.as_str() {
            "add_status_item" => self.handle_new_status_item(&rpc.params),
            "update_status_item" => self.update_status_item(&rpc.params),
            "plugin_started" => debug!("{}: -> {}", &rpc.method, &rpc.params),
            "available_languages" => debug!("{}", &rpc.method),
            "available_themes" => debug!("{}", &rpc.method),
            "available_plugins" => debug!("{}", &rpc.method),
            "config_changed" => debug!("{}", &rpc.method),
            "def_style" => self.handle_style_change(&rpc.params),
            "language_changed" => debug!("{}", &rpc.method),
            "scroll_to" => self.handle_cursor_move(&ctx, &rpc.params),
            "update" => self.handle_content_update(&ctx, &rpc.params),
            "theme_changed" => debug!("{}", &rpc.method),
            _ => debug!("unhandled notif {} -> {}", &rpc.method, &rpc.params),
        };
    }

    fn handle_request(&mut self, _ctx: &RpcCtx, rpc: Self::Request) -> Result<Value, RemoteError> {
        info!("[request] {} -> {:#?}", rpc.method, rpc.params);
        Ok(json!({}))
    }
}

impl EventController {
    pub fn new(layout: Box<dyn Layout>, styles: Rc<RefCell<Box<dyn Styles>>>) -> Self {
        let status_bar = StatusBar::new(layout.create_new_status_bar_window());

        Self {
            styles,
            layout,
            views: HashMap::new(),
            status_bar,
            current_view: String::new(),
        }
    }

    fn handle_new_status_item(&mut self, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Event {
            //source: String,
            key: String,
            value: String,
            alignment: String,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        if let "change-mode" = event.key.as_str() {
            self.status_bar.update_mode(&event.value);
        }
    }

    fn update_status_item(&mut self, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Event {
            key: String,
            value: String,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        if let "change-mode" = event.key.as_str() {
            self.status_bar.update_mode(&event.value);
        }
    }

    /// Handle the "def_style" event.
    ///
    /// This function need to create a new set of background/foreground and save
    /// it with the given id.
    fn handle_style_change(&mut self, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Event {
            id: StyleID,
            #[serde(default)]
            italic: bool,
            fg_color: Option<u32>,
            bg_color: Option<u32>,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        // fg
        let fg_color = event.fg_color.map(|fg| {
            let rgba = fg.to_le_bytes();
            RGBColor {
                r: rgba[0],
                g: rgba[1],
                b: rgba[2],
            }
        });

        let bg_color = event.bg_color.map(|bg| {
            let rgba = bg.to_le_bytes();
            RGBColor {
                r: rgba[0],
                g: rgba[1],
                b: rgba[2],
            }
        });

        self.styles
            .borrow_mut()
            .save(event.id, fg_color, bg_color, event.italic);
    }

    /// Handle the "scroll_to" event.
    ///
    /// It move the cursor into the given position. If the position is not
    /// within the screen, it will scroll all the view content by modifying
    /// the `self.screen_start` value.
    fn handle_cursor_move(&mut self, ctx: &RpcCtx, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Event {
            view_id: String,
            col: u32,
            line: u32,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        self.create_view_if_required(ctx, &event.view_id);
        self.views
            .get_mut(&event.view_id)
            .unwrap()
            .move_cursor(ctx, event.line, event.col);
    }

    /// Handle the "update" event.
    ///
    /// It create a new buffer, apply all the event directives, swith this
    /// new buffer with the old one than trigger a redraw.
    ///
    /// The event is compose of differents `Operation`. Each one indicate how
    /// to fill the new buffer. The posible operations are:
    /// - "copy" -> Copy a part of the old buffer into the new one.
    /// - "skip" -> Keep a number of line empty.
    /// - "invalidate" -> Mark some lines as not available because the core
    ///     doesn't have given their content yet.
    /// - "ins" -> Insert some new content.
    fn handle_content_update(&mut self, ctx: &RpcCtx, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Update {
            annotations: Vec<Annotation>,
            #[serde(rename = "ops")]
            operations: Vec<Operation>,
        }

        #[derive(Deserialize, Debug)]
        struct Event {
            view_id: String,
            update: Update,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        self.create_view_if_required(ctx, &event.view_id);
        let view = self.views.get_mut(&event.view_id).unwrap();

        view.update_buffer(event.update.operations);
    }

    fn create_view_if_required(&mut self, ctx: &RpcCtx, view_id: &str) {
        if self.views.contains_key(view_id) {
            return;
        }

        let window = self.layout.create_view_window();

        let new_view = View::new(ctx, &view_id, window, self.styles.clone());
        self.views.insert(view_id.to_string(), new_view);

        self.current_view = view_id.to_string();
    }
}
