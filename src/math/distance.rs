use crate::types::Position;

pub fn distance2(pos1: Position, pos2: Position) -> i32 {
    let x = pos1.x - pos2.x;
    let y = pos1.y - pos2.y;
    return x * x + y * y;
}

pub fn distance(pos1: Position, pos2: Position) -> f64 {
    return (distance2(pos1, pos2) as f64).sqrt();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean_distance() {
        assert_eq!(distance(
            Position { x: 0, y: 0 },
            Position { x: -1, y: 1 },
        ), 2.0f64.sqrt())
    }

    #[test]
    fn dist2() {
        assert_eq!(distance2(
            Position { x: 0, y: 0 },
            Position { x: -2, y: -2 },
        ), 8)
    }
}
