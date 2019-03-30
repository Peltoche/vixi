use std::collections::HashMap;
use std::panic;
use std::process::exit;
use std::sync::{Once, ONCE_INIT};

use crate::event_controller::{Buffer, Cursor, Line};

use ncurses::*;

#[derive(Eq, PartialEq, Debug)]
pub enum RedrawBehavior {
    OnlyDirty,
    Everything,
}

const SPACES_IN_LINE_SECTION: u32 = 2;

/// The style id is used to override the ncurses default colors and save the
/// style color. If this number is two hight, some color conflicts will appeares.
///
/// Check the `handle_style_change` method documentation for more informations.
const MAX_STYLE_ID: u32 = 50;

/// The color id for the default background.
const BG_STYLE_ID: u32 = 253;
/// The pair id for the default background/foreground.
const DEFAULT_COLOR_PAIR_ID: i16 = 254;

const SELECTION_STYLE_ID: u32 = 0;

const SELECTION_COLOR_NAMESPACE: u32 = 150;

static HANDLER: Once = ONCE_INIT;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub style_id: u32,
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
#[derive(Debug, Clone, Copy)]
pub struct RGBColor {
    /// red
    pub r: u8,
    /// green
    pub g: u8,
    /// blue
    pub b: u8,
}

#[derive(Clone, Debug, Default)]
pub struct Terminal {
    styles: HashMap<u32, Style>,
    size_line_section: u32,
}

impl Terminal {
    pub fn new() -> Self {
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");

        /* Setup ncurses. */
        initscr();
        raw();
        keypad(stdscr(), true); // Allow for extended keyboard (like F1).
        noecho();
        start_color();
        //nodelay(stdscr(), true);
        set_escdelay(0);
        halfdelay(1);

        install_custom_panic_handler();

        let terminal = Self::default();

        // Save the background color for the selection.
        //
        // TODO: make the background color configurable.
        terminal.save_color(
            SELECTION_STYLE_ID,
            RGBColor {
                r: 70,
                g: 70,
                b: 70,
            },
        );

        // Paint all the screen with the black color in order to set an uniform
        // background color.
        //
        // TODO: make the background color configurable.
        terminal.set_background_color(RGBColor { r: 0, g: 0, b: 0 });

        terminal
    }

    /// Move the cursor into the text space.
    ///
    /// This translate a position into the text area into a screen position.
    pub fn move_cursor(&self, cursor: &Cursor) {
        mv(cursor.y as i32, (cursor.x + self.size_line_section) as i32);
    }

    /// Return the screen size in term of characters.
    pub fn get_size(&self) -> (usize, usize) {
        let mut size_x = 0;
        let mut size_y = 0;
        getmaxyx(stdscr(), &mut size_y, &mut size_x);

        // Remove two lines for the status and the command bars.
        ((size_y - 2) as usize, size_x as usize)
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
    pub fn save_style_set(
        &mut self,
        style_id: u32,
        fg_color: RGBColor,
        bg_color: RGBColor,
        italic: bool,
    ) {
        if style_id > MAX_STYLE_ID {
            error!(
                "the new style id is greater than {}, this will load to some randome colors.",
                MAX_STYLE_ID
            );
        }

        // Name space the foreground and background colors.
        let fg_style_id = 50 + style_id;
        let bg_style_id = 100 + style_id;
        let selected_style_id = SELECTION_COLOR_NAMESPACE + style_id;

        self.save_color(fg_style_id, fg_color);
        self.save_color(bg_style_id, bg_color);

        // Save the new pair of background/foreground color with the `init_pair`
        // method. The pair_id must be the same id than the style id in order
        // to avoid translation during the rendering (cf: the
        // `print_stylized_line` method).
        init_pair(style_id as i16, fg_style_id as i16, bg_style_id as i16);
        init_pair(
            selected_style_id as i16,
            fg_style_id as i16,
            SELECTION_STYLE_ID as i16,
        );

        // Save the other metas into a map.
        self.styles.insert(style_id, Style { style_id, italic });
    }

    pub fn set_background_color(&self, color: RGBColor) {
        // Create a new pair with the background color and white as foreground
        // color.
        self.save_color(BG_STYLE_ID, color);
        init_pair(
            DEFAULT_COLOR_PAIR_ID as i16,
            COLOR_WHITE,
            BG_STYLE_ID as i16,
        );

        // Apply this color everywhere in the terminal by setting some ` ` char
        // everywhere.
        bkgd(' ' as chtype | COLOR_PAIR(DEFAULT_COLOR_PAIR_ID) as chtype);
    }

    /// Redraw the screen content.
    ///
    /// It take the Line corresponding to `this.buffer[this.screen_start]` and
    /// render it as the top line and fill the screen with all the following
    /// lines.
    pub fn redraw_view(&mut self, buffer_start: usize, behavior: RedrawBehavior, buffer: &Buffer) {
        // Caculate the size of the line section.
        //
        // This size change in function of the number of line du to the size of
        // the number to render. Count the number of spaces set around the section.
        let new_size_line_section =
            (buffer.total_len().to_string().len()) as u32 + SPACES_IN_LINE_SECTION;
        self.size_line_section = new_size_line_section;

        let (screen_size_y, _) = self.get_size();

        let buffer_len = if buffer.lines_availables_after(buffer_start) < screen_size_y as usize {
            // The number of lines inside the buffer is less than the available lines on the screen so
            // it print all the remaining of the buffer.
            buffer.lines_availables_after(buffer_start)
        } else {
            // The number of lines inside the buffer is greater than the available lines on the screen so
            // it print only what the screen is able to show.
            screen_size_y as usize
        };

        let buffer_iter = buffer
            .lines
            .iter()
            .skip(buffer_start as usize)
            .take(buffer_len);
        for (screen_line, line) in buffer_iter.enumerate() {
            if behavior == RedrawBehavior::Everything || line.is_dirty {
                self.rewrite_line(screen_line, &line);
            }
        }
    }

    fn rewrite_line(&mut self, line_number: usize, line: &Line) {
        #[derive(Clone, Debug)]
        struct CharStyle {
            style_id: u32,
            selected: bool,
            italic: bool,
        }

        mv(line_number as i32, 0);
        clrtoeol();

        // Print the line number.
        addstr(
            format!(
                " {:width$} ",
                line.ln,
                width = (self.size_line_section - SPACES_IN_LINE_SECTION) as usize
            )
            .as_str(),
        );

        let mut style_map: Vec<CharStyle> = Vec::with_capacity(line.raw.len());
        style_map.resize(
            line.raw.len(),
            CharStyle {
                style_id: BG_STYLE_ID,
                selected: false,
                italic: false,
            },
        );

        let mut idx = 0;
        let mut style_iter = line.styles.iter();
        for _ in 0..line.styles.len() / 3 {
            let style_start = (*style_iter.next().unwrap()) as i32;
            let style_length = (*style_iter.next().unwrap()) as i32;
            let style_id = (*style_iter.next().unwrap()) as u32;

            let style = self.styles.get(&style_id);

            for i in idx + style_start..idx + style_start + style_length {
                let char_style = &mut style_map[i as usize];

                if style_id == SELECTION_STYLE_ID {
                    char_style.selected = true;
                } else {
                    char_style.style_id = style_id;

                    if style.unwrap().italic {
                        char_style.italic = true;
                    }
                }
            }
            idx += style_start + style_length;
        }

        let mut content_iter = line.raw.chars();
        for style in style_map.iter() {
            let attrs = if style.italic { A_ITALIC() } else { A_NORMAL() };

            let style_id = if style.selected {
                SELECTION_COLOR_NAMESPACE + style.style_id
            } else {
                style.style_id
            };

            addch(content_iter.next().unwrap() as chtype | attrs | COLOR_PAIR(style_id as i16));
        }
    }

    pub fn update_status_bar_mode(&mut self, mode: &str) {
        info!("update status: {}", mode);
        let size_y = getmaxy(stdscr());

        // Remove 1 for the command line.
        mv(size_y - 1, 0);

        addstr(&mode.to_uppercase());
    }

    /// The `init_color` method take a color range within [0..1000] but the
    /// RGBA colors received by the event are within the range [0..256]. A
    /// rough conversion is done by multiplying the event values by 4.
    fn save_color(&self, id: u32, color: RGBColor) {
        init_color(
            id as i16,
            i16::from(color.r) * 4,
            i16::from(color.g) * 4,
            i16::from(color.b) * 4,
        );
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
