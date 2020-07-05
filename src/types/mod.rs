pub(crate) mod heap_element;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Segment {
    pub start: Position,
    pub end: Position,
}

