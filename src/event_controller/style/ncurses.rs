use std::collections::HashMap;

use super::{RGBColor, Style, StyleID, Styles};

use ncurses::*;

/// The pair id for the default background/foreground.
///
/// The pair_id 0 is the one used by default by the ncurse.
pub const DEFAULT_COLOR_PAIR_ID: i16 = 0;

/// The style id is used to override the ncurses default colors and save the
/// style color. If this number is two hight, some color conflicts will appeares.
///
/// Check the `handle_style_change` method documentation for more informations.
const MAX_STYLE_ID: StyleID = 50;

/// Split the 255 available values into namespaces in which the foreground,
/// background and selection colors are separated.
const PAIR_NAMESPACE: i16 = MAX_STYLE_ID * 0;
const FG_COLOR_NAMESPACE: i16 = MAX_STYLE_ID * 1;
const BG_COLOR_NAMESPACE: i16 = MAX_STYLE_ID * 2;
const SELECTION_COLOR_NAMESPACE: i16 = MAX_STYLE_ID * 3;
const CUSTOM_COLOR_NAMESPACE: i16 = MAX_STYLE_ID * 4;

/// ID for the background color id used for the selections.
const SELECTION_BACKGROUND_COLOR_ID: i16 = CUSTOM_COLOR_NAMESPACE + 0;

#[derive(Debug)]
pub struct Ncurses {
    styles: HashMap<StyleID, Style>,
}

impl Ncurses {
    pub fn new() -> Self {
        let client = Self {
            styles: HashMap::new(),
        };

        // Save the background color for the selection.
        //
        // TODO: make the background color configurable.
        client.save_color(
            SELECTION_BACKGROUND_COLOR_ID,
            RGBColor {
                r: 90,
                g: 90,
                b: 90,
            },
        );

        client
    }

    /// The `init_color` method take a color range within [0..1000] but the
    /// RGBA colors received by the event are within the range [0..256]. A
    /// rough conversion is done by multiplying the event values by 4.
    fn save_color(&self, id: StyleID, color: RGBColor) {
        init_color(
            id,
            i16::from(color.r) * 4,
            i16::from(color.g) * 4,
            i16::from(color.b) * 4,
        );
    }
}

impl Styles for Ncurses {
    fn get(&self, style_id: &StyleID) -> Style {
        match self.styles.get(style_id) {
            Some(res) => *res,
            None => {
                error!("failed to retrieve the style {}", style_id);
                self.get_default()
            }
        }
    }

    fn get_default(&self) -> Style {
        Style {
            style_id: DEFAULT_COLOR_PAIR_ID,
            italic: false,
            selected: false,
        }
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
    fn save(&mut self, style_id: StyleID, fg_color: RGBColor, bg_color: RGBColor, italic: bool) {
        if style_id > MAX_STYLE_ID {
            error!(
                "the new style id is greater than {}, this will load to some randome colors.",
                MAX_STYLE_ID
            );
        }

        // Name space the foreground and background colors.
        let pair_id = PAIR_NAMESPACE + style_id;
        let fg_style_id = FG_COLOR_NAMESPACE + style_id;
        let bg_style_id = BG_COLOR_NAMESPACE + style_id;
        let selected_style_id = SELECTION_COLOR_NAMESPACE + style_id;

        self.save_color(fg_style_id, fg_color);
        self.save_color(bg_style_id, bg_color);

        // Save the new pair of background/foreground color with the `init_pair`
        // method. The pair_id must be the same id than the style id in order
        // to avoid translation during the rendering (cf: the
        // `print_stylized_line` method).
        init_pair(pair_id, fg_style_id, bg_style_id);
        init_pair(
            selected_style_id,
            fg_style_id,
            SELECTION_BACKGROUND_COLOR_ID,
        );

        // Save the other metas into a map.
        self.styles.insert(
            style_id,
            Style {
                style_id,
                italic,
                selected: false,
            },
        );
    }
}
