use std::collections::HashMap;

use super::{
    RGBColor, Style, StyleID, StyleRange, Styles, LINE_SECTION_STYLE_ID, SELECTION_STYLE_ID,
    STYLE_LEN,
};

lazy_static! {
    static ref BG_RESET: String = format!("{}", color::bg_reset());
    static ref FG_RESET: String = format!("{}", color::fg_reset());
    static ref EMPTY_STYLE: Style = Style {
        background: None,
        foreground: None,
        italic: false,
    };
}

pub struct TermionStyles {
    styles: HashMap<StyleID, Style>,
}

impl TermionStyles {
    pub fn new() -> Self {
        let mut client = Self {
            styles: HashMap::new(),
        };

        client.save(
            SELECTION_STYLE_ID,
            None,
            Some(RGBColor {
                r: 70,
                g: 70,
                b: 70,
            }),
            false,
        );

        client.save(
            LINE_SECTION_STYLE_ID,
            Some(RGBColor { r: 255, g: 0, b: 0 }),
            None,
            false,
        );

        client
    }
}

impl Styles for TermionStyles {
    fn append_with_style(&self, to_append: &str, style_id: &StyleID, dest: &mut String) {
        use std::fmt::Write;

        match self.styles.get(style_id) {
            Some(res) => {
                let (bg_color, bg_reset): (&str, &str) = match &res.background {
                    Some(color) => (color, &BG_RESET),
                    None => ("", ""),
                };

                let (fg_color, fg_reset): (&str, &str) = match &res.foreground {
                    Some(color) => (color, &FG_RESET),
                    None => ("", ""),
                };

                write!(
                    dest,
                    "{}{}{}{}{}",
                    bg_color, fg_color, to_append, bg_reset, fg_reset
                )
                .unwrap();
            }
            None => {
                error!("failed to retrieve the style {}", style_id);
                dest.push_str(to_append);
                //self.append_with_default_style(to_append, dest);
            }
        }
    }

    fn save(
        &mut self,
        style_id: StyleID,
        fg_color: Option<RGBColor>,
        bg_color: Option<RGBColor>,
        italic: bool,
    ) {
        // Save the other metas into a map.
        self.styles.insert(
            style_id,
            Style {
                background: bg_color.map(|bg| color::bg(bg)),
                foreground: fg_color.map(|fg| color::fg(fg)),
                italic,
            },
        );
    }

    fn apply_to(&self, raw_styles: Vec<i16>, input: &str) -> String {
        let mut styles = self.convert_to_style_range(raw_styles);

        self.serialize_style_ranges(&mut styles);

        let mut res = String::with_capacity(input.len() + STYLE_LEN * styles.len());
        for style in styles {
            res.push_str(&format!(
                "{}{}{}{}{}",
                style.style.background.unwrap_or_else(|| String::from("")),
                style.style.foreground.unwrap_or_else(|| String::from("")),
                unsafe { input.get_unchecked(style.start as usize..style.end as usize) },
                BG_RESET.as_str(),
                FG_RESET.as_str(),
            ));
        }

        res
    }
}

impl TermionStyles {
    fn convert_to_style_range(&self, inputs: Vec<i16>) -> Vec<StyleRange> {
        let mut styles = Vec::with_capacity(inputs.len() / 3);

        let mut input_iter = inputs.iter();
        let mut idx = 0;
        for _ in 0..inputs.len() / 3 {
            let style_start = (*input_iter.next().unwrap()) as i32;
            let style_length = *input_iter.next().unwrap() as i32;
            let style_id = *input_iter.next().unwrap() as StyleID;

            let style = match self.styles.get(&style_id) {
                Some(style) => style.clone(),
                None => {
                    error!("style id {} not found", style_id);
                    Style {
                        background: None,
                        foreground: None,
                        italic: false,
                    }
                }
            };

            styles.push(StyleRange {
                start: (idx + style_start) as u32,
                end: (idx + style_start + style_length) as u32,
                style,
            });

            idx += style_start + style_length;
        }

        styles.sort_unstable_by(|a, b| {
            if a.start == b.start {
                a.end.cmp(&b.end)
            } else {
                a.start.cmp(&b.start)
            }
        });

        styles
    }

    fn serialize_style_ranges(&self, styles: &mut Vec<StyleRange>) {
        let mut idx = 0;
        // The style_len() need to be done outside the while loop because it
        // need to be dynamic due to the fact that we can insert some new
        // RangeStyle.
        if styles.len() == 0 {
            return;
        }
        let mut style_len = styles.len() - 1;
        while idx < style_len {
            if styles[idx].end <= styles[idx + 1].start {
                idx += 1;
                continue;
            }

            if styles[idx].start == styles[idx + 1].start {
                // This condition handle the case where two style have the same
                // start, like:
                //
                // 0  id1  4    idx + 1      10
                // |-------|-----------------|
                //         4   idx   6
                //         |---------|
                //
                // The idea to merge the styles from styles[idx] and styles[idx+1]
                // and put it into styles[idx]. Then it move the start of
                // styles[idx+1]. This solution have for advantage to avoid an
                // StyleRange insert and so a copy.
                //
                // This should give something like:
                //
                // 0  id1  4   idx   6 idx + 1  10
                // |-------|    +    |----------|
                //         4 idx + 1 6
                //         |---------|
                styles[idx + 1].start = styles[idx].end;

                let style_before = &styles[idx];
                let style_after = &styles[idx + 1];

                let color_after = style_after.style.clone();
                styles[idx].style = Style {
                    background: color_after
                        .background
                        .or(style_before.style.background.clone()),
                    foreground: color_after
                        .foreground
                        .or(style_before.style.foreground.clone()),
                    italic: style_after.style.italic || style_before.style.italic,
                };
            } else if styles[idx].end > styles[idx + 1].end {
                // This condition handle the case a style is completely overlapping
                // an another, like:
                //
                // 0     id1     8
                // |-------------|
                //     2 id2 6
                //     |-----|
                //
                // The idea use the styles[idx+1] as the ovelapping style and
                // insert a copy of id1 after it.
                //
                // This should give something like:
                //
                // 0 id1 2
                // |-----|
                //       2 id2 6
                //       |-----|
                //             6 id1 8
                //             |-----|
                //

                let style_before = &styles[idx];
                let mut style_after = style_before.clone();
                let overlapping_style = &styles[idx + 1];

                style_after.start = overlapping_style.end;
                styles[idx].end = overlapping_style.start;

                if styles[idx + 1].style.foreground.is_none() {
                    styles[idx + 1].style.foreground = style_after.style.foreground.clone();
                }

                if styles[idx + 1].style.background.is_none() {
                    styles[idx + 1].style.background = style_after.style.background.clone();
                }

                styles.insert(idx + 2, style_after);
                style_len += 1;
            } else {
                // This condition handle the case where two styles are overlaping
                // without the same start, like:
                //
                // 0   id1   6
                // |---------|
                //       2   id2    8
                //       |----------|
                //
                // The idea is to insert a new StyleRange with which is a merge
                // of the two style an inject it between the two.
                //
                // This should give somethig like:
                //
                // 0 id1 2
                // |-----|
                //       2 id3 6
                //       |-----|
                //             6 id2 8
                //             |-----|
                //
                let style_before = &styles[idx];
                let style_after = &styles[idx + 1];

                let color_after = style_after.style.clone();
                let overlap_style = StyleRange {
                    start: style_after.start,
                    end: style_before.end,
                    style: Style {
                        background: color_after
                            .background
                            .or(style_before.style.background.clone()),
                        foreground: color_after
                            .foreground
                            .or(style_before.style.foreground.clone()),
                        italic: style_after.style.italic || style_before.style.italic,
                    },
                };

                styles[idx].end = overlap_style.start;
                styles[idx + 1].start = overlap_style.end;
                styles.insert(idx + 1, overlap_style);
                style_len += 1;
            }

            styles.sort_unstable_by(|a, b| {
                if a.start == b.start {
                    a.end.cmp(&b.end)
                } else {
                    a.start.cmp(&b.start)
                }
            });

            idx = 0;
        }
    }
}

#[cfg(not(test))]
mod color {
    use crate::event_controller::style::RGBColor;

    use termion::color::{Bg, Fg, Reset, Rgb};

    pub fn bg(bg: RGBColor) -> String {
        format!("{}", Bg(Rgb(bg.r, bg.g, bg.b)))
    }

    pub fn bg_reset() -> String {
        format!("{}", Bg(Reset))
    }

    pub fn fg(fg: RGBColor) -> String {
        format!("{}", Fg(Rgb(fg.r, fg.g, fg.b)))
    }

    pub fn fg_reset() -> String {
        format!("{}", Fg(Reset))
    }
}

#[cfg(test)]
mod color {
    use crate::event_controller::style::RGBColor;

    pub fn bg(bg: RGBColor) -> String {
        format!("{}", format!("bg({}/{}/{})", bg.r, bg.g, bg.b))
    }

    pub fn bg_reset() -> String {
        String::from("bg(reset)")
    }

    pub fn fg(fg: RGBColor) -> String {
        format!("{}", format!("fg({}/{}/{})", fg.r, fg.g, fg.b))
    }

    pub fn fg_reset() -> String {
        String::from("fg(reset)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_line_descriptor_with_no_overlapping_styles() {
        // 0  id1  3  id2  7    11  id3   13
        // |-------|-------|-----|---------|
        let styles = vec![0, 3, 1, 0, 4, 2, 4, 2, 3];

        let mut style_registry = TermionStyles::new();
        style_registry.save(1, Some(RGBColor { r: 255, g: 0, b: 0 }), None, false);
        style_registry.save(2, Some(RGBColor { r: 0, g: 255, b: 0 }), None, false);
        style_registry.save(3, Some(RGBColor { r: 0, g: 0, b: 255 }), None, false);

        let res = style_registry.new_line_descriptor(styles);

        assert_eq!(3, res.0.len());
        assert_eq!(
            res.0[0],
            StyleRange {
                start: 0,
                end: 3,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: None,
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[1],
            StyleRange {
                start: 3,
                end: 7,
                style: Style {
                    foreground: Some(String::from("fg(0/255/0)")),
                    background: None,
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[2],
            StyleRange {
                start: 11,
                end: 13,
                style: Style {
                    foreground: Some(String::from("fg(0/0/255)")),
                    background: None,
                    italic: false,
                }
            }
        );
    }

    #[test]
    fn new_line_descriptor_with_a_simple_overlapping_style() {
        // 0  id1  4
        // |-------|
        //     2  id2   6
        //     |--------|
        let styles = vec![0, 4, 1, -2, 4, 2];

        let mut style_registry = TermionStyles::new();
        style_registry.save(1, Some(RGBColor { r: 255, g: 0, b: 0 }), None, false);
        style_registry.save(2, None, Some(RGBColor { r: 0, g: 255, b: 0 }), false);

        let res = style_registry.new_line_descriptor(styles);

        assert_eq!(3, res.0.len());
        assert_eq!(
            res.0[0],
            StyleRange {
                start: 0,
                end: 2,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: None,
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[1],
            StyleRange {
                start: 2,
                end: 4,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: Some(String::from("bg(0/255/0)")),
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[2],
            StyleRange {
                start: 4,
                end: 6,
                style: Style {
                    foreground: None,
                    background: Some(String::from("bg(0/255/0)")),
                    italic: false,
                }
            }
        );
    }

    #[test]
    fn new_line_descriptor_with_a_completely_overlapping_style() {
        // 0     id1       8
        // |---------------|
        //    2  id2   6
        //    |--------|
        let styles = vec![0, 8, 1, -6, 4, 2];

        let mut style_registry = TermionStyles::new();
        style_registry.save(1, Some(RGBColor { r: 255, g: 0, b: 0 }), None, false);
        style_registry.save(2, None, Some(RGBColor { r: 0, g: 255, b: 0 }), false);

        let res = style_registry.new_line_descriptor(styles);

        assert_eq!(3, res.0.len());
        assert_eq!(
            res.0[0],
            StyleRange {
                start: 0,
                end: 2,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: None,
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[1],
            StyleRange {
                start: 2,
                end: 6,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: Some(String::from("bg(0/255/0)")),
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[2],
            StyleRange {
                start: 6,
                end: 8,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: None,
                    italic: false,
                }
            }
        );
    }

    #[test]
    fn new_line_descriptor_with_a_double_overlapping_style() {
        // 0  id1  4  id3  10
        // |-------|-------|
        //     2  id2   6
        //     |--------|
        let styles = vec![0, 4, 1, -2, 4, 2, -2, 6, 3];

        let mut style_registry = TermionStyles::new();
        style_registry.save(1, Some(RGBColor { r: 255, g: 0, b: 0 }), None, false);
        style_registry.save(2, None, Some(RGBColor { r: 0, g: 255, b: 0 }), false);
        style_registry.save(3, Some(RGBColor { r: 0, g: 0, b: 255 }), None, false);

        let res = style_registry.new_line_descriptor(styles);

        assert_eq!(4, res.0.len());
        assert_eq!(
            res.0[0],
            StyleRange {
                start: 0,
                end: 2,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: None,
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[1],
            StyleRange {
                start: 2,
                end: 4,
                style: Style {
                    foreground: Some(String::from("fg(255/0/0)")),
                    background: Some(String::from("bg(0/255/0)")),
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[2],
            StyleRange {
                start: 4,
                end: 6,
                style: Style {
                    foreground: Some(String::from("fg(0/0/255)")),
                    background: Some(String::from("bg(0/255/0)")),
                    italic: false,
                }
            }
        );

        assert_eq!(
            res.0[3],
            StyleRange {
                start: 6,
                end: 10,
                style: Style {
                    foreground: Some(String::from("fg(0/0/255)")),
                    background: None,
                    italic: false,
                }
            }
        );
    }
}
