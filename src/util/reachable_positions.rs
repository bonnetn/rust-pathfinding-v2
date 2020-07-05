use crate::math::line_of_sight::line_of_sight;
use crate::types::{Position, Segment};

pub fn reachable_positions<'a>(
    start: Position,
    obstacles: &'a [Segment],
    destination: Position,
) -> impl Iterator<Item=Position> + 'a {
    // Take all the segment start & end position and put them into an iterator.
    let obstacles_positions = obstacles
        .iter()
        .flat_map(|segment| {
            return vec![segment.start, segment.end].into_iter();
        });

    let obstacles_positions: Vec<Position> = obstacles_positions.collect();
    let obstacles_positions = obstacles_positions.into_iter();

    // All segment positions + end position.
    let candidates = std::iter::once(destination)
        .chain(obstacles_positions);

    // Remove all positions that are not reachable.
    let candidates = candidates
        .filter(move |candidate| {
            line_of_sight(start, obstacles, *candidate)
        });

    return candidates;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 10, y: 10 };
        let segments = [
            Segment {
                start: Position { x: 5, y: -1 },
                end: Position { x: 5, y: 4 },
            }, Segment { // Partly hidden behind the other segment.
                start: Position { x: 6, y: 5 },
                end: Position { x: 6, y: 0 },
            },
        ];
        let got: Vec<Position> = reachable_positions(start, &segments, end).collect();
        assert_eq!(got, [
            Position { x: 10, y: 10 }, // End
            Position { x: 5, y: -1 }, // Segment 1
            Position { x: 5, y: 4 }, // Segment 1
            Position { x: 6, y: 5 }, // Segment 2 (the other is hidden behind segment 1)
        ]);
    }

    #[test]
    fn no_reachable_pos() {
        let start = Position { x: 5, y: 5 };
        let end = Position { x: 100, y: 100 };
        let segments = [
            Segment {
                start: Position { x: -1, y: 0 },
                end: Position { x: 11, y: 0 },
            },
            Segment {
                start: Position { x: 10, y: 11 },
                end: Position { x: 10, y: -1 },
            },
            Segment {
                start: Position { x: 11, y: 10 },
                end: Position { x: -1, y: 10 },
            },
            Segment {
                start: Position { x: 0, y: 11 },
                end: Position { x: 0, y: -1 },
            },
        ];
        let got: Vec<Position> = reachable_positions(start, &segments, end).collect();
        assert_eq!(got, []);
    }
}
