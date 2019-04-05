mod ncurses;
pub mod termion;

pub use self::ncurses::NcursesStyles;
pub use self::termion::TermionStyles;

pub type StyleID = i16;

/// The style id 0 is reserved for the selection style id.
///
/// This id is different than the pair id.
pub const SELECTION_STYLE_ID: StyleID = 0;

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

pub trait Styles {
    fn set(&self, style_id: &StyleID);
    fn set_default(&self);
    fn save(&mut self, style_id: StyleID, fg_color: RGBColor, bg_color: RGBColor, italic: bool);
}
