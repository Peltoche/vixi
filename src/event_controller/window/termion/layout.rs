use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

use super::window::TermionWindow;
use crate::event_controller::window::{Layout, Window, WindowPosition, WindowSize};

use termion::clear;
use termion::color::DetectColors;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

const STATUS_HEIGHT: u32 = 1;

pub struct TermionLayout {
    height: u32,
    width: u32,
    writer: Rc<RefCell<Box<dyn Write>>>,
}

impl TermionLayout {
    pub fn new() -> Self {
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        debug!("{} colors available", stdout.available_colors().unwrap());

        write!(stdout, "{}", clear::All).unwrap();

        let (width, height) = termion::terminal_size().unwrap();

        Self {
            writer: Rc::new(RefCell::new(Box::new(stdout))),
            height: u32::from(height),
            width: u32::from(width),
        }
    }
}

impl Layout for TermionLayout {
    fn create_view_window(&self) -> Box<dyn Window> {
        let window = TermionWindow::new(
            self.writer.clone(),
            WindowPosition { y: 0, x: 0 },
            WindowSize {
                height: self.height - STATUS_HEIGHT,
                width: self.width,
            },
        );

        Box::new(window)
    }

    fn create_new_status_bar_window(&self) -> Box<dyn Window> {
        let window = TermionWindow::new(
            self.writer.clone(),
            WindowPosition {
                y: self.height - STATUS_HEIGHT,
                x: 0,
            },
            WindowSize {
                height: STATUS_HEIGHT,
                width: self.width,
            },
        );

        Box::new(window)
    }
}
