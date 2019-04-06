use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

use super::{RGBColor, StyleID, Styles};

use termion::color;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub background: termion::color::Rgb,
    pub foreground: termion::color::Rgb,
    pub italic: bool,
    pub selected: bool,
}

pub struct TermionStyles {
    writer: Rc<RefCell<Box<dyn Write>>>,
    styles: HashMap<StyleID, Style>,
    default_background: color::Rgb,
    default_foreground: color::Rgb,
}

impl TermionStyles {
    pub fn new(writer: Rc<RefCell<Box<dyn Write>>>) -> Self {
        let client = Self {
            writer,
            styles: HashMap::new(),
            default_background: color::Rgb(0, 0, 0),
            default_foreground: color::Rgb(255, 255, 255),
        };

        client
    }
}

impl Styles for TermionStyles {
    fn set(&self, style_id: &StyleID) {
        match self.styles.get(style_id) {
            Some(res) => {
                write!(
                    self.writer.borrow_mut(),
                    "{}{}",
                    color::Bg(res.background),
                    color::Fg(res.foreground),
                )
                .unwrap();
            }
            None => {
                error!("failed to retrieve the style {}", style_id);
                self.set_default()
            }
        }
    }

    fn set_default(&self) {
        write!(
            self.writer.borrow_mut(),
            "{}{}",
            color::Bg(self.default_background),
            color::Fg(self.default_foreground),
        )
        .unwrap();
    }

    fn save(&mut self, style_id: StyleID, fg_color: RGBColor, bg_color: RGBColor, italic: bool) {
        // Save the other metas into a map.
        self.styles.insert(
            style_id,
            Style {
                background: color::Rgb(bg_color.r, bg_color.g, bg_color.b),
                foreground: color::Rgb(fg_color.r, fg_color.g, fg_color.b),
                italic,
                selected: false,
            },
        );
    }
}
