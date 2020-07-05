use crate::types::Position;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct HeapElement {
    pub pos: Position,
    pub dist: f64,
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.dist.partial_cmp(&other.dist) {
            None => Ordering::Equal,
            Some(o) => o.reverse(),
        }
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        self.pos.eq(&other.pos)
    }
}
