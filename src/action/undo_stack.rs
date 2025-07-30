use super::Action;

pub struct UndoNode {
    pub redo: Action,
    pub undo: Action,
}

pub struct UndoStack {
    pub nodes: Vec<UndoNode>,
    // 1-based index
    pub index: usize,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            index: 0,
        }
    }

    pub fn add(&mut self, redo: Action, undo: Action) {
        // Remove all nodes past current index if they exist
        if self.index < self.nodes.len() {
            self.nodes.truncate(self.index);
        }
        self.nodes.push(UndoNode { redo, undo });
        self.index += 1;
    }

    pub fn undo(&mut self) -> Option<Action> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;

        Some(self.nodes[self.index].undo.clone())
    }

    pub fn redo(&mut self) -> Option<Action> {
        if self.index == self.nodes.len() {
            return None;
        }
        self.index += 1;

        Some(self.nodes[self.index - 1].redo.clone())
    }
}
