use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
use crossterm::style::Stylize;

const TAB_WIDTH: usize = 4;

#[derive(Clone)]
pub struct Line {
    pub graphemes: Vec<String>,
}

impl Line {
    pub fn new() -> Self {
        Self { graphemes: vec![] }
    }

    pub fn from_string(s: String) -> Self {
        Self {
            graphemes: s.graphemes(true).map(|g| g.to_string()).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

    pub fn width_to(&self, index: usize) -> usize {
        let mut width = 0;
        for character in self.graphemes.iter().take(index) {
            if character == "\t" {
                // Calculate tab width based on current position
                // Not sure how this works in tandem with emojis etc.
                width += TAB_WIDTH - (width % TAB_WIDTH);
            } else {
                width += character.width();
            }
        }
        width
    }

    pub fn x_at_width(&self, width_goal: usize) -> Option<usize> {
        let mut width = 0;
        let mut i = 0;
        for character in &self.graphemes {
            if character == "\t" {
                width += TAB_WIDTH - (width % TAB_WIDTH);
            } else {
                width += character.width();
            }

            if width > width_goal {
                return Some(i);
            }

            i += 1;
        }

        None
    }

    pub fn print(&self, offset: Option<usize>, highlight: Option<[usize; 2]>) {
        let offset = offset.unwrap_or(0);
        for (i, grapheme) in self.graphemes.iter().enumerate().skip(offset) {
            let string = if grapheme == "\t" {
                &" ".repeat(TAB_WIDTH)
            } else {
                grapheme
            };

            if let Some([a, b]) = highlight {
                if i >= a && i <= b {
                    print!("{}", string.clone().on_blue())
                } else {
                    print!("{}", string)
                }
            } else {
                print!("{}", string)
            };
        }
    }
}
