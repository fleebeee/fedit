use crate::action::{Action, ActionType};
use crate::editor::core::Editor;
use crate::types::{Line, Point};
use unicode_segmentation::UnicodeSegmentation;

impl Editor {
    pub fn apply_action(&mut self, action: &Action) {
        let start = action.start;

        match action.kind {
            ActionType::Insert => {
                let empty_vec = vec![];
                // If inserting in the middle of a line
                // left | insertion | right \n
                let (left, right) = if start.x >= self.content[start.y].len() {
                    (
                        self.content[start.y].graphemes.as_slice(),
                        empty_vec.as_slice(),
                    )
                } else {
                    self.content[start.y].graphemes.split_at(start.x)
                };

                let payload = action.payload.clone().unwrap();

                match payload.len() {
                    0 => unreachable!(),
                    1 => {
                        let mut new_line = Line {
                            graphemes: left.to_vec(),
                        };
                        new_line
                            .graphemes
                            .extend(payload[0].graphemes.iter().cloned());
                        let x_new = new_line.len();
                        new_line.graphemes.extend(right.to_vec());
                        self.content[start.y] = new_line;
                        self.move_cursor(Point::new(x_new, self.cursor.y));
                    }
                    _ => {
                        let mut new_lines = vec![];
                        let mut new_first_line = Line {
                            graphemes: left.to_vec(),
                        };
                        new_first_line
                            .graphemes
                            .extend(payload[0].graphemes.iter().cloned());
                        new_lines.push(new_first_line);

                        for middle_line in payload.iter().skip(1).take(payload.len() - 2) {
                            new_lines.push(middle_line.clone());
                        }

                        let mut new_last_line = payload.last().unwrap().clone();
                        let x_new = new_last_line.len();

                        new_last_line.graphemes.extend(right.to_vec());
                        new_lines.push(new_last_line);
                        let y_new = self.cursor.y + new_lines.len() - 1;

                        self.content.remove(start.y);
                        self.content.splice(start.y..start.y, new_lines);

                        self.move_cursor(Point::new(x_new, y_new));
                    }
                }
            }
            ActionType::Remove => {
                let end = action.end.unwrap();

                match end.y - start.y {
                    0 => {
                        // Removing within a single line
                        self.content[start.y].graphemes.drain(start.x..end.x);
                        self.move_cursor(start);
                    }
                    _ => {
                        // Removing across multiple lines
                        let left = &self.content[start.y].graphemes[..start.x];
                        let right = &self.content[end.y].graphemes[end.x..];

                        // Create new combined line
                        let mut new_line = Line {
                            graphemes: left.to_vec(),
                        };
                        new_line.graphemes.extend(right.to_vec());

                        // Replace the range of lines with the single combined line
                        self.content.splice(start.y..=end.y, [new_line]);
                        self.move_cursor(start);
                    }
                }
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.content.get_mut(self.cursor.y) {
            let grapheme = c.to_string();

            // Check if this char should combine with the previous character
            // (e.g. skin tone modifiers, combining diacritics)
            if self.cursor.x > 0 {
                if let Some(prev_char) = line.graphemes.get_mut(self.cursor.x - 1) {
                    let combined = format!("{}{}", prev_char, grapheme);
                    // Check if they form a single grapheme cluster
                    if combined.graphemes(true).count() == 1 {
                        // They combine into one grapheme cluster
                        *prev_char = combined.clone();
                        return;
                    }
                }
            }

            let redo = Action {
                start: Point::new(self.cursor.x, self.cursor.y),
                end: None,
                payload: Some(vec![Line {
                    graphemes: vec![grapheme],
                }]),
                kind: ActionType::Insert,
            };

            let undo = Action {
                start: Point::new(self.cursor.x, self.cursor.y),
                end: Some(Point::new(self.cursor.x + 1, self.cursor.y)),
                payload: None,
                kind: ActionType::Remove,
            };

            self.apply_action(&redo);
            self.undo_stack.add(redo, undo);
        }
    }

    pub fn insert_newline(&mut self) {
        let start = self.cursor;

        let redo = Action {
            start: Point::new(start.x, start.y),
            end: None,
            payload: Some(vec![Line::new(), Line::new()]),
            kind: ActionType::Insert,
        };

        self.apply_action(&redo);

        let undo = Action {
            start: Point::new(start.x, start.y),
            end: Some(Point::new(self.cursor.x, self.cursor.y)),
            payload: None,
            kind: ActionType::Remove,
        };

        self.undo_stack.add(redo, undo);
    }

    pub fn get_char_at(&self, point: Point) -> Option<String> {
        self.content.get(point.y)?.graphemes.get(point.x).cloned()
    }

    pub fn remove_char(&mut self) {
        if let Some(start) = self.get_previous_point() {
            let redo = Action {
                start,
                end: Some(self.cursor),
                payload: None,
                kind: ActionType::Remove,
            };

            let undo = Action {
                start,
                end: None,
                payload: Some(vec![Line {
                    graphemes: vec![self.get_char_at(start).unwrap()],
                }]),
                kind: ActionType::Insert,
            };

            self.apply_action(&redo);
            self.undo_stack.add(redo, undo);
        }
    }

    pub fn paste(&mut self) {
        if let Some(clipboard) = &self.clipboard {
            let start = self.cursor;

            let redo = Action {
                start,
                end: None,
                payload: Some(clipboard.clone()),
                kind: ActionType::Insert,
            };

            self.apply_action(&redo);

            let undo = Action {
                start,
                end: Some(self.cursor),
                payload: None,
                kind: ActionType::Remove,
            };

            self.undo_stack.add(redo, undo);
        }
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.undo() {
            self.apply_action(&action);
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.undo_stack.redo() {
            self.apply_action(&action);
        }
    }
}
