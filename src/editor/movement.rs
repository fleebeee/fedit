use crate::editor::core::{Direction, Editor};
use crate::types::Point;
use crossterm::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

impl Editor {
    // Make sure the cursor stays within the viewport
    pub fn adjust_offset(&mut self) {
        let Point {
            x: dims_width,
            y: dims_height,
        } = Editor::get_dimensions();

        if self.cursor.y < self.offset.y {
            self.offset.y = self.cursor.y;
        } else if self.cursor.y >= self.offset.y + dims_height {
            self.offset.y = self.cursor.y - dims_height + 1;
        }

        let target_width = self.get_current_line().width_to(self.cursor.x);
        if target_width < self.offset.x {
            self.offset.x = target_width;
        } else if target_width >= self.offset.x + dims_width {
            self.offset.x = target_width - dims_width + 1;
        }
    }

    pub fn get_previous_point(&self) -> Option<Point> {
        if self.cursor.x > 0 {
            Some(Point::new(self.cursor.x - 1, self.cursor.y))
        } else if self.cursor.y > 0 {
            let previous_line = &self.content[self.cursor.y - 1];
            Some(Point::new(previous_line.len(), self.cursor.y - 1))
        } else {
            None
        }
    }

    pub fn get_next_point(&self) -> Option<Point> {
        let current_line = self.get_current_line();

        if self.cursor.x < current_line.len() {
            Some(Point::new(self.cursor.x + 1, self.cursor.y))
        } else if self.cursor.y < self.content.len() - 1 {
            Some(Point::new(0, self.cursor.y + 1))
        } else {
            None
        }
    }

    pub fn move_cursor(&mut self, destination: Point) {
        self.cursor = destination;
        self.adjust_offset();
    }

    pub fn handle_movement_input(&mut self, direction: Direction, modifiers: KeyModifiers) {
        let current_line = self.get_current_line();
        let point_old = self.cursor;
        let Point { x: x_old, y: y_old } = point_old;

        let (x_new, y_new) = match direction {
            // Vertical motion
            Direction::Up | Direction::Down => match modifiers {
                // SUPER + UP or SUPER + DOWN
                x if x & KeyModifiers::SUPER != KeyModifiers::NONE => {
                    self.preferred_width = 0;

                    match direction {
                        Direction::Up => (0, 0),
                        Direction::Down => {
                            let y_max = self.content.len().saturating_sub(1);
                            let x_max = self.content[y_max].len();

                            (x_max, y_max)
                        }
                        _ => unreachable!(),
                    }
                }
                // UP or DOWN
                _ => {
                    let dy = match direction {
                        Direction::Up => -1,
                        Direction::Down => 1,
                        _ => unreachable!(),
                    };

                    let y_new = (y_old as isize + dy)
                        .min(self.content.len() as isize - 1)
                        .max(0) as usize;

                    if y_new == y_old {
                        (x_old, y_old)
                    } else {
                        let new_line = &self.content[y_new as usize];
                        let preferred_x = new_line.x_at_width(self.preferred_width);
                        let x_new = preferred_x.map_or(new_line.len(), |p| p);

                        (x_new, y_new)
                    }
                }
            },
            Direction::Left | Direction::Right => match modifiers {
                // SUPER+LEFT or SUPER+RIGHT
                x if x & KeyModifiers::SUPER != KeyModifiers::NONE => match direction {
                    Direction::Left => {
                        if x_old == 0 {
                            if y_old > 0 {
                                let y_new = y_old - 1;
                                let x_new = self.content[y_new].len();

                                (x_new, y_new)
                            } else {
                                (x_old, y_old)
                            }
                        } else {
                            (0, y_old)
                        }
                    }
                    Direction::Right => {
                        if x_old == current_line.len() {
                            if y_old < self.content.len().saturating_sub(1) {
                                let x_new = 0;
                                let y_new = y_old + 1;

                                (x_new, y_new)
                            } else {
                                (x_old, y_old)
                            }
                        } else {
                            (current_line.len(), y_old)
                        }
                    }
                    _ => unreachable!(),
                },
                // LEFT or RIGHT
                _ => {
                    let (x_new, y_new) = match direction {
                        Direction::Left => {
                            if let Some(Point { x: x_new, y: y_new }) = self.get_previous_point() {
                                (x_new, y_new)
                            } else {
                                (x_old, y_old)
                            }
                        }
                        Direction::Right => {
                            if let Some(Point { x: x_new, y: y_new }) = self.get_next_point() {
                                (x_new, y_new)
                            } else {
                                (x_old, y_old)
                            }
                        }
                        _ => unreachable!(),
                    };

                    self.preferred_width = self.content[y_new].width_to(x_new);

                    (x_new, y_new)
                }
            },
        };

        let point_new = Point { x: x_new, y: y_new };

        // Selection logic
        if modifiers & KeyModifiers::SHIFT != KeyModifiers::NONE {
            self.handle_selection(point_old, point_new);
        } else {
            self.selection = None;
        }

        self.move_cursor(point_new);
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> bool {
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let click_y = mouse_event.row as usize;
                let click_x = mouse_event.column as usize;
                let y_new = click_y + self.offset.y;

                // If clicking the line we're on, check x offset
                let line = &self.content[y_new];
                let offset_width = if y_new == self.cursor.y {
                    line.width_to(self.offset.x)
                } else {
                    0
                };
                let width_goal = click_x + offset_width;

                if y_new != self.cursor.y {
                    self.offset.x = 0;
                }

                self.cursor.x = line.x_at_width(width_goal).unwrap_or(line.len());
                self.cursor.y = y_new;
                self.preferred_width = width_goal;

                return true;
            }
            _ => (),
        };

        false
    }
}
