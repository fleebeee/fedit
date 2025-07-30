use crate::editor::core::Editor;
use crate::types::{Line, Point};

impl Editor {
    pub fn handle_selection(&mut self, point_old: Point, point_new: Point) {
        let selection = match &self.selection {
            None => [point_old, point_new],
            Some([a, _]) => [*a, point_new],
        };

        self.selection = Some(selection);
    }

    pub fn copy(&mut self) {
        if let Some(mut selection) = self.selection {
            selection.sort_unstable();
            let [a, b] = selection;
            let mut lines = vec![];

            for (i, line) in self
                .content
                .iter()
                .skip(a.y)
                .take(b.y + 1 - a.y)
                .enumerate()
            {
                if a.y == b.y {
                    // Single line
                    lines.push(Line {
                        graphemes: line.graphemes[a.x..b.x].to_vec(),
                    });
                } else if i == 0 {
                    // First line in multiline
                    lines.push(Line {
                        graphemes: line.graphemes[a.x..].to_vec(),
                    });
                } else if i == b.y - a.y {
                    // Last line in multiline
                    lines.push(Line {
                        graphemes: line.graphemes[..b.x].to_vec(),
                    });
                } else {
                    // Middle line
                    lines.push(Line {
                        graphemes: line.graphemes.to_vec(),
                    });
                }
            }

            self.clipboard = Some(lines);
        }
    }
}
