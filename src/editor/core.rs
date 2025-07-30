use crate::action::UndoStack;
use crate::types::{Line, Point, Status};
use crossterm::{
    event::{
        self, Event, KeyCode, KeyEvent, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute, terminal,
};
use std::fs;
use std::io;
use std::io::stdout;

#[derive(PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub struct Editor {
    pub content: Vec<Line>,
    pub cursor: Point,
    pub offset: Point,
    pub preferred_width: usize,
    pub selection: Option<[Point; 2]>,
    pub clipboard: Option<Vec<Line>>,
    pub undo_stack: UndoStack,
    pub filename: Option<String>,
    pub status: Option<Status>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: vec![Line::new()],
            cursor: Point::new(0, 0),
            offset: Point::new(0, 0),
            preferred_width: 0,
            selection: None,
            clipboard: None,
            undo_stack: UndoStack::new(),
            filename: None,
            status: None,
        }
    }

    pub fn load_file(&mut self, filename: &str) -> io::Result<()> {
        match fs::read_to_string(filename) {
            Ok(content) => {
                self.content = content
                    .lines()
                    .map(|line| Line::from_string(line.to_string()))
                    .collect();

                if self.content.is_empty() {
                    self.content.push(Line::new());
                }
                self.filename = Some(filename.to_string());
                Ok(())
            }
            Err(_) => {
                // File doesn't exist, start with empty content
                self.content = vec![Line::new()];
                self.filename = Some(filename.to_string());
                Ok(())
            }
        }
    }

    pub fn save_file(&mut self) {
        if let Some(filename) = &self.filename {
            let content = self
                .content
                .iter()
                .map(|line| {
                    line.graphemes
                        .iter()
                        .map(|g| g.as_str())
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n");

            if let Err(e) = fs::write(filename, content) {
                self.status = Some(Status::new(format!("Error saving file: {}", e)));
            } else {
                self.status = Some(Status::new(format!("Saved to {}", filename)));
            }
        } else {
            self.status = Some(Status::new("No filename specified".to_string()));
        }
    }

    pub fn get_current_line(&self) -> &Line {
        self.content
            .get(self.cursor.y)
            .expect("Current line not found")
    }

    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), terminal::EnterAlternateScreen)?;
        let mut stdout = stdout();
        execute!(
            stdout,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        )?;

        // Enable mouse support
        execute!(stdout, crossterm::event::EnableMouseCapture)?;

        // Initial draw
        self.draw()?;

        loop {
            match event::read()? {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => {
                    match (code, modifiers) {
                        (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                        (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.save_file(),
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.copy(),
                        (KeyCode::Char('v'), KeyModifiers::CONTROL) => self.paste(),
                        (KeyCode::Char('z'), KeyModifiers::CONTROL) => self.undo(),
                        (KeyCode::Char('y'), KeyModifiers::CONTROL) => self.redo(),
                        (KeyCode::Up, mods) => self.handle_movement_input(Direction::Up, mods),
                        (KeyCode::Down, mods) => self.handle_movement_input(Direction::Down, mods),
                        (KeyCode::Left, mods) => self.handle_movement_input(Direction::Left, mods),
                        (KeyCode::Right, mods) => {
                            self.handle_movement_input(Direction::Right, mods)
                        }
                        (KeyCode::Backspace, _) => self.remove_char(),
                        (KeyCode::Tab, _) => self.insert_char('\t'),
                        (KeyCode::Enter, _) => self.insert_newline(),
                        (KeyCode::Char(c), KeyModifiers::NONE) => {
                            self.insert_char(c);
                        }
                        (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                            self.insert_char(c);
                        }
                        _ => {}
                    }
                    self.draw()?;
                }
                Event::Mouse(mouse_event) => {
                    if self.handle_mouse_event(mouse_event) {
                        self.draw()?;
                    };
                }
                _ => {}
            }
        }

        execute!(stdout, crossterm::event::DisableMouseCapture)?;
        execute!(stdout, PopKeyboardEnhancementFlags)?;
        execute!(stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
