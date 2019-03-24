use std::panic;
use std::process::exit;
use std::sync::{Once, ONCE_INIT};

use ncurses::*;

/// The style id is used to override the ncurses default colors and save the
/// style color. If this number is two hight, some color conflicts will appeares.
///
/// Check the `handle_style_change` method documentation for more informations.
const MAX_STYLE_ID: u32 = 50;

static HANDLER: Once = ONCE_INIT;

pub struct Style {
    pub color_id: u32,
    pub italic: bool,
}

/// An RGB color description.
///
/// Each value define the amount of a primary color composing it. The possible
/// values for each primary color go from `0` for the absence of color to `255`
/// for the full presence of the color.
///
/// Example:
/// ```rust
/// let black = RGBColor{r: 0, g: 0, b: 0}
/// let white = RGBColor{r: 255, g: 255, b: 255}
/// let red = RGBColor{r: 255, g: 0, b: 0}
/// ```
pub struct RGBColor {
    /// red
    pub r: u8,
    /// green
    pub g: u8,
    /// blue
    pub b: u8,
}

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        /* Setup ncurses. */
        initscr();
        raw();
        keypad(stdscr(), true); // Allow for extended keyboard (like F1).
        noecho();
        start_color();

        install_custom_panic_handler();

        Self {}
    }

    pub fn move_cursor(&self, y: u32, x: u32) {
        mv(y as i32, x as i32);
    }

    pub fn rewrite_line(&self, y: usize) -> LineReWriter {
        LineReWriter::new(y)
    }

    /// Return the screen size in term of characters.
    pub fn get_size(&self) -> (u32, u32) {
        let mut size_x = 0;
        let mut size_y = 0;
        getmaxyx(stdscr(), &mut size_y, &mut size_x);

        (size_y as u32, size_x as u32)
    }

    pub fn redraw(&mut self) {
        refresh();
    }

    /// The style saving process is done via the ncurse routines by overriding
    /// the existing colors presets via the `init_color` and `init_pair`
    /// functions. Thoses functionalities are only available for the
    /// terminals with the `truecolor` capability. In order to check if your
    /// terminal can handle it check that the output of `echo $COLORTERM` is
    /// equal to `truecolor` and the output of `echo $TERM` is equal to
    /// `xterm-256color`.
    ///
    /// As the `xterm-256color` feature is set, the terminal preset a set of
    /// 256 colors with the ids from 0 to 255. As the feature `truecolor` is set
    /// we can overrid those colors with some arbitrary RGB color. It will save
    /// the color_pairs (a set of background + foreground color) within the id
    /// range of [0..50], the foreground colors within the range [50..100] and
    /// the background colors whithin the range [100...150]. As each style
    /// correspond to a color pair, which is composed of a background and and
    /// foreground color it can save only 50 differents styles. After this number
    /// the colors_pairs saved will override the foreground colors and the
    /// foreground colors will override the background colors leading to some
    /// randome colors sets.
    pub fn save_color_set(&mut self, color_id: u32, fg_color: RGBColor, bg_color: RGBColor) {
        if color_id > MAX_STYLE_ID {
            error!(
                "the new style id is greater than {}, this will load to some randome colors.",
                MAX_STYLE_ID
            );
        }

        // Name space the foreground and background colors.
        let fg_color_id = 50 + (color_id as i16);
        let bg_color_id = 100 + (color_id as i16);

        // The `init_color` method take a color range within [0..1000] but the
        // RGBA colors received by the event are within the range [0..256]. A
        // rough conversion is done by multiplying the event values by 4.
        init_color(
            fg_color_id,
            i16::from(fg_color.r) * 4,
            i16::from(fg_color.g) * 4,
            i16::from(fg_color.b) * 4,
        );

        init_color(
            bg_color_id,
            i16::from(bg_color.r) * 4,
            i16::from(bg_color.g) * 4,
            i16::from(bg_color.b) * 4,
        );

        // Save the new pair of background/foreground color with the `init_pair`
        // method. The pair_id must be the same id than the style id in order
        // to avoid translation during the rendering (cf: the
        // `print_stylized_line` method).
        init_pair(color_id as i16, fg_color_id, bg_color_id);
    }

    pub fn set_style(&self, style: &Style) {
        let attr = COLOR_PAIR(style.color_id as i16);
        attron(attr);

        if style.italic {
            attron(A_ITALIC());
        }
    }

    pub fn unset_style(&self, style: &Style) {
        let attr = COLOR_PAIR(style.color_id as i16);
        attroff(attr);

        if style.italic {
            attroff(A_ITALIC());
        }
    }
}

pub struct LineReWriter {}

impl LineReWriter {
    pub fn new(line: usize) -> Self {
        mv(line as i32, 0);
        clrtoeol();

        Self {}
    }

    pub fn push_str(&mut self, s: &str) {
        addstr(s);
    }
}

fn install_custom_panic_handler() {
    HANDLER.call_once(|| {
        let default_handler = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            // Clean the terminal.
            endwin();

            // Run the default panic handler.
            default_handler(info);

            // Exit with the status '1'.
            exit(1);
        }));
    })
}
