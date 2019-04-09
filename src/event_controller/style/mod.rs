pub mod termion;

pub use self::termion::TermionStyles;

pub type StyleID = i16;

pub const STYLE_LEN: usize = 20;

/// The style id 0 is reserved for the selection style id.
///
/// This id is different than the pair id.
pub const SELECTION_STYLE_ID: StyleID = 0;

pub const LINE_SECTION_STYLE_ID: StyleID = 9999;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub italic: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StyleRange {
    start: u32,
    end: u32,
    style: Style,
}

pub trait Styles {
    fn append_with_style(&self, to_append: &str, style_id: StyleID, dest: &mut String);
    fn apply_to(&self, inputs: Vec<i16>, input: &str) -> String;
    fn save(
        &mut self,
        style_id: StyleID,
        fg_color: Option<RGBColor>,
        bg_color: Option<RGBColor>,
        italic: bool,
    );
}
