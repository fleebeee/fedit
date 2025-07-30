use crate::types::{Line, Point};

#[derive(Clone, Copy)]
pub enum ActionType {
    Insert,
    Remove,
}

#[derive(Clone)]
pub struct Action {
    pub start: Point,
    pub end: Option<Point>,
    pub payload: Option<Vec<Line>>,
    pub kind: ActionType,
}
