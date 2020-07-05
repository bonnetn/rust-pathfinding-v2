use crate::types::{Position, Segment};

pub fn line_of_sight(start: Position, obstacles: &[Segment], destination: Position) -> bool {
    return obstacles.iter()
        .all(|obstacle| line_of_sight_single(start, obstacle, destination));
}

/// Returns false of obstacle is preventing line of sight between start and destination.
/// Return true by convention if start == destination.
/// If destination is == to one of the edge of the obstacle, line of sight will return true.
fn line_of_sight_single(start: Position, obstacle: &Segment, destination: Position) -> bool {
    // Pos1 + Dir1 * x == Pos2 + Dir2 * y
    // (Pos1 + Dir1 * x) ^ Dir2 == Pos2 ^ Dir2
    //  Dir1 ^ Dir2 * x == Pos2 ^ Dir2 - Pos1 ^ Dir2
    // If X != 0:
    //      x == (Pos2 ^ Dir2 - Pos1 ^ Dir2) / (Dir1 ^ Dir2)
    // For Y:
    //      Dir2 * y == Pos1 + Dir1 * x - Pos2
    //      ||Dir2||^2 * y == (Pos1 + Dir1 * x - Pos2) . Dir
    //      y == (Pos1 + Dir1 * x - Pos2) . Dir2 / ||Dir2||^2

    if destination == obstacle.start || destination == obstacle.end {
        return true
    }

    let pos1 = (obstacle.start.x, obstacle.start.y);
    let dir1 = (
        obstacle.end.x - obstacle.start.x,
        obstacle.end.y - obstacle.start.y,
    );

    let pos2 = (start.x, start.y);
    let dir2 = (destination.x - start.x, destination.y - start.y);

    let denominator = cross_product!(dir1, dir2);
    if denominator == 0 {
        return true;
    }

    let numerator = cross_product!(pos2, dir2) - cross_product!(pos1, dir2);
    let x = (numerator as f64) / (denominator as f64);
    if x < 0. || x > 1. {
        return true; // Intersect with the obstacle, but outside its bounds.
    }

    let denominator = dot_product!(
        (dir2.0, dir2.1),
        (dir2.0, dir2.1)
    );
    if denominator == 0 {
        // By convention, if dir2.dir2 == 0 <=> ||dir2|| == 0 <=> start == end
        // We consider there is a line of sight in that case.
        return true;
    }
    let numerator = dot_product!(
        (
            (pos1.0 - pos2.0) as f64 + dir1.0 as f64 * x,
            (pos1.1 - pos2.1) as f64 + dir1.1 as f64 * x
        ),
        (dir2.0 as f64, dir2.1 as f64)
    );
    let y = numerator / (denominator as f64);
    if y <= 0. || y >= 1. {
        return true; // Intersect with an obstacle, but not between start and end.
    }

    return false;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_no_line_of_sight() {
        let got = line_of_sight(
            Position {
                x: 0,
                y: 0,
            },
            &[Segment {
                start: Position { x: 5, y: 0 },
                end: Position { x: 5, y: 10 },
            }],
            Position {
                x: 10,
                y: 10,
            },
        );
        assert_eq!(got, false)
    }

    #[test]
    fn has_line_of_sight1() {
        let got = line_of_sight(
            Position {
                x: 0,
                y: 0,
            },
            &[Segment {
                start: Position { x: 5, y: 9 },
                end: Position { x: 5, y: 8 }, // If obstacle was longer, it would obtrude the LOS.
            }],
            Position {
                x: 10,
                y: 10,
            },
        );
        assert_eq!(got, true)
    }

    #[test]
    fn has_line_of_sight2() {
        let got = line_of_sight(
            Position {
                x: 0,
                y: 0,
            },
            &[Segment {
                start: Position { x: 11, y: 20 },
                end: Position { x: 11, y: -10 },
            }],
            Position { // If destination was a bit further it wouldn't have LOS.
                x: 10,
                y: 10,
            },
        );
        assert_eq!(got, true)
    }
}
