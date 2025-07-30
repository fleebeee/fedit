use crate::editor::core::Editor;
use crate::types::Point;
use crossterm::{
    cursor, execute,
    style::Stylize,
    terminal::{self, ClearType},
};
use std::io::{self, Write, stdout};

const STATUS_BAR_HEIGHT: usize = 1;

impl Editor {
    pub fn get_dimensions() -> Point {
        let (width, height) = terminal::size().ok().unwrap();
        // Always reserve space for status
        Point::new(width as usize, height as usize - STATUS_BAR_HEIGHT)
    }

    pub fn draw_status_line(&self) -> io::Result<()> {
        let Point {
            x: width,
            y: height,
        } = Editor::get_dimensions();

        let status = if self.status.is_some() && self.status.as_ref().unwrap().is_fresh() {
            self.status.as_ref().unwrap().text.clone()
        } else {
            format!(
                " {} • {}:{} ",
                self.filename.as_deref().unwrap_or("[No Name]"),
                self.cursor.y + 1,
                self.cursor.x + 1,
            )
        };

        // Truncate if too long
        let truncated_status = if status.len() > width as usize {
            format!("{}...", &status[..width as usize - 3])
        } else {
            status
        };

        // Move to status line and draw
        execute!(stdout(), cursor::MoveTo(0, height as u16))?;
        print!("{}", truncated_status.on_dark_grey());

        stdout().flush()?;
        Ok(())
    }

    pub fn draw(&mut self) -> io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        let Point {
            x: width,
            y: height,
        } = Editor::get_dimensions();

        let selection = self.selection.map_or(None, |selection| {
            let [a, b] = selection;
            if b.y > a.y || b.y == a.y && b.x > a.x {
                Some([a, b])
            } else {
                Some([b, a])
            }
        });

        // Draw content
        for (i, line) in self
            .content
            .iter()
            .enumerate()
            .skip(self.offset.y)
            .take(height)
        {
            execute!(stdout(), cursor::MoveTo(0, i as u16 - self.offset.y as u16))?;
            execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;

            let active = self.cursor.y == i;
            let offset = if active { Some(self.offset.x) } else { None };

            // Map selection [Point, Point] to [usize, usize] corresponding to x-indices
            // on this line
            let highlight = selection.map_or(None, |selection| match selection {
                // Single line selection
                [Point { x: x1, y: y1 }, Point { x: x2, y: y2 }] if y1 == i && y2 == i => {
                    Some([x1, x2])
                }
                // Selection starts on this row and continues past line break
                [Point { x: x1, y: y1 }, Point { x: _, y: y2 }] if y1 == i && y2 != i => {
                    Some([x1, line.len()])
                }
                // Selection started earlier and ends on this line
                [Point { x: _, y: y1 }, Point { x: x2, y: y2 }] if y1 != i && y2 == i => {
                    Some([0, x2])
                }
                // Selection wholly encompasses this line
                [Point { x: _, y: y1 }, Point { x: _, y: y2 }] if y1 < i && y2 > i => {
                    Some([0, line.len()])
                }
                _ => None,
            });

            line.print(offset, highlight);
        }

        // Draw status line
        self.draw_status_line()?;

        // Position cursor correctly
        let screen_y = self.cursor.y.saturating_sub(self.offset.y);
        let line = self.get_current_line();
        let mut display_x: usize = line
            .width_to(self.cursor.x)
            .saturating_sub(line.width_to(self.offset.x));

        if line.len() > self.cursor.x {
            display_x = display_x.min(width - 1);
        }

        // Only position cursor if it's within the editor area
        if screen_y < height {
            execute!(stdout(), cursor::MoveTo(display_x as u16, screen_y as u16))?;
        }

        stdout().flush()?;
        Ok(())
    }
}
