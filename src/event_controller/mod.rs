pub mod view;

use self::view::View;
use crate::devices::terminal::{RGBColor, RedrawBehavior, StyleID, Terminal};

use serde_json::Value;
use xi_rpc::{RemoteError, RpcCall, RpcCtx};

#[derive(Deserialize, Debug)]
pub struct LineDescription {
    cursor: Option<Vec<i32>>,
    ln: usize,
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

#[derive(Default, Clone)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub nb_invalid_lines: usize,
}

impl Buffer {
    pub fn total_len(&self) -> usize {
        self.lines.len() + self.nb_invalid_lines
    }

    pub fn lines_availables_after(&self, start: usize) -> usize {
        self.lines.len() - start
    }
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
    view: View,
}

impl xi_rpc::Handler for EventController {
    type Notification = RpcCall;
    type Request = RpcCall;

    fn handle_notification(&mut self, ctx: &RpcCtx, rpc: Self::Notification) {
        match rpc.method.as_str() {
            //"add_status_item" => self.handle_new_status_item(&rpc.params),
            //"plugin_started" => debug!("{}: -> {}", &rpc.method, &rpc.params),
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

        //self.terminal.redraw();
    }

    fn handle_request(&mut self, _ctx: &RpcCtx, rpc: Self::Request) -> Result<Value, RemoteError> {
        info!("[request] {} -> {:#?}", rpc.method, rpc.params);
        Ok(json!({}))
    }
}

impl EventController {
    pub fn new(view: View) -> Self {
        Self { view }
    }

    //fn handle_new_status_item(&mut self, body: &Value) {
    //#[derive(Deserialize, Debug)]
    //struct Event {
    ////source: String,
    //key: String,
    //value: String,
    //alignment: String,
    //}

    //let event: Event = serde_json::from_value(body.clone()).unwrap();

    //if let "change-mode" = event.key.as_str() {
    //self.terminal.update_status_bar_mode(&event.value);
    //}
    //self.terminal.move_cursor(&self.cursor);
    //}

    //fn update_status_item(&mut self, body: &Value) {
    //#[derive(Deserialize, Debug)]
    //struct Event {
    //key: String,
    //value: String,
    //}

    //let event: Event = serde_json::from_value(body.clone()).unwrap();

    //if let "change-mode" = event.key.as_str() {
    //self.terminal.update_status_bar_mode(&event.value);
    //}
    //self.terminal.move_cursor(&self.cursor);
    //}

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
            fg_color: u32,
            #[serde(default)]
            bg_color: u32,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        // Override the default colors with the `init_color` method. Once save
        // those colors will be accessible via the ids `fg_style_id` and
        // `bg_style_id`.
        //

        // fg
        let fg_rgba: [u8; 4] = event.fg_color.to_le_bytes();
        let fg_color = RGBColor {
            r: fg_rgba[0],
            g: fg_rgba[1],
            b: fg_rgba[2],
        };

        let bg_rgba: [u8; 4] = event.bg_color.to_le_bytes();
        let bg_color = RGBColor {
            r: bg_rgba[0],
            g: bg_rgba[1],
            b: bg_rgba[2],
        };

        //self.terminal
        //.save_style_set(event.id, fg_color, bg_color, event.italic);
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

        self.view.move_cursor(ctx, event.line, event.col);
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
        struct Annotation {
            #[serde(rename = "type")]
            kind: String,
            n: usize,
            payloads: Option<()>,
            ranges: Vec<[usize; 4]>,
        }

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
        self.view.update_buffer(event.update.operations);

        //let (size_y, size_x) = self.terminal.get_size();
        //if size_x != self.screen_width {
        //ctx.get_peer().send_rpc_notification(
        //"edit",
        //&json!({
        //"method": "resize",
        //"view_id": event.view_id,
        //"params": {
        //"width": size_x  ,
        //"height": size_y,
        //}
        //}),
        //);
        //}

        //self.buffer = new_buffer;
        //self.terminal
        //.redraw_view(self.screen_start, RedrawBehavior::OnlyDirty, &self.buffer);
        //self.terminal.move_cursor(&self.cursor);
    }
}
