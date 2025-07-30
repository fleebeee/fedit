pub mod action;
pub mod editor;
pub mod types;

// Re-export commonly used items
pub use action::{Action, ActionType, UndoStack};
pub use editor::Editor;
pub use types::{Line, Point};
