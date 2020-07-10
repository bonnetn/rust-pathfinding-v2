use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};
use std::f64::INFINITY;
use std::slice;

use crate::math::distance::distance;
use crate::types::{Position, Segment};
use crate::types::heap_element::HeapElement;
use crate::util::reachable_positions::reachable_positions;

mod util;
mod math;
mod types;


#[no_mangle]
pub unsafe extern "C" fn find_path_ffi(output_arr: *mut Position, output_arr_size: i32, obstacles: *const Segment, obstacles_size: i32, start: *const Position, end: *const Position) -> i32 {
    // INPUT:
    let start = *start.as_ref().expect("null start position");
    let end = *end.as_ref().expect("null end position");

    assert!(!obstacles.is_null(), "null obstacle pointer");
    assert!(obstacles_size > 0, "negative obstacle_size");
    let obstacles: &[Segment] = slice::from_raw_parts(obstacles, obstacles_size as usize);

    // OUTPUT:
    assert!(!output_arr.is_null(), "null output_arr pointer");
    assert!(output_arr_size > 0, "negative output_arr_size");
    let arr: &mut [Position] = slice::from_raw_parts_mut(output_arr, output_arr as usize);

    let result = find_path(start, obstacles, end);
    if let Some(result) = result {
        let output_size = min(output_arr as usize, result.len());
        for i in 0..output_size {
            arr[i] = result[i];
        }
        return output_size as i32;
    }
    return 0;
}

/// find_path is a simple implementation of Dijkstra algorithm over a visibility graph defined by
/// the obstacles.
pub fn find_path<'a>(
    start: Position,
    obstacles: &[Segment],
    destination: Position,
) -> Option<Vec<Position>> {
    let mut total_distance: HashMap<Position, f64> = HashMap::new();
    total_distance.insert(start, 0.);

    let mut q: BinaryHeap<HeapElement> = BinaryHeap::new();
    q.push(HeapElement {
        pos: start,
        dist: 0.0,
    });

    let mut prev: HashMap<Position, Position> = HashMap::new();
    // Start is unset on purpose so that get_path can detect when to stop rewinding the path.

    while let Some(HeapElement { pos, dist }) = q.pop() {
        let current_distance = *total_distance.get(&pos)
            .expect("should always have a distance set if in the queue");

        if dist > current_distance {
            continue; // Skip it, it was already processed with a lower distance.
        }

        for n in reachable_positions(pos, obstacles, destination) {
            let alt = current_distance + distance(pos, n);
            let neighbor_dist = match total_distance.get(&n) {
                None => INFINITY,
                Some(v) => *v,
            };
            if neighbor_dist > alt {
                total_distance.insert(n, alt);
                prev.insert(n, pos);
                if n == destination {
                    return Some(get_path(prev, destination));
                }

                q.push(HeapElement {
                    pos: n,
                    dist: alt,
                });
            }
        }
    }
    return None;
}

fn get_path(prev: HashMap<Position, Position>, destination: Position) -> Vec<Position> {
    let mut v: Vec<Position> = Vec::new();
    let mut cur = destination;
    v.push(cur);
    while let Some(n) = prev.get(&cur) {
        v.push(*n);
        cur = *n;
    }
    v.reverse();

    return v;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_obstacles() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 10, y: 10 };
        let segments = [];
        let got = find_path(start, &segments, end);
        assert_eq!(got, Some(vec![
            Position { x: 0, y: 0 },
            Position { x: 10, y: 10 },
        ]))
    }

    #[test]
    fn direct_path_with_obstacles() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 10, y: 10 };
        let segments = [
            Segment {
                start: Position { x: 5, y: -1 },
                end: Position { x: 5, y: 4 },
            }, Segment {
                start: Position { x: 6, y: 5 },
                end: Position { x: 6, y: 0 },
            },
        ];
        let got = find_path(start, &segments, end);
        assert_eq!(got, Some(vec![
            Position { x: 0, y: 0 },
            Position { x: 10, y: 10 },
        ]))
    }

    #[test]
    fn indirect_path_with_obstacles() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 10, y: 10 };
        let segments = [
            Segment {
                start: Position { x: 5, y: -1000 },
                end: Position { x: 5, y: 10 },
            }, Segment {
                start: Position { x: 6, y: 1000 },
                end: Position { x: 6, y: 0 },
            },
        ];
        let got = find_path(start, &segments, end);
        assert_eq!(got, Some(vec![
            Position { x: 0, y: 0 },
            Position { x: 5, y: 10 },
            Position { x: 6, y: 0 },
            Position { x: 10, y: 10 },
        ]))
    }

    fn maze() -> Vec<Segment> {
        vec![
            Segment {
                start: Position { x: -100, y: -1 },
                end: Position { x: 100, y: -1 },
            },
            Segment {
                start: Position { x: 11, y: 100 },
                end: Position { x: 11, y: -100 },
            },
            Segment {
                start: Position { x: 100, y: 11 },
                end: Position { x: -100, y: 11 },
            },
            Segment {
                start: Position { x: -1, y: 100 },
                end: Position { x: -1, y: -100 },
            },
            Segment {
                start: Position { x: 1, y: 9 },
                end: Position { x: 1, y: -100 },
            },
            Segment {
                start: Position { x: 3, y: 9 },
                end: Position { x: 3, y: -100 },
            },
            Segment {
                start: Position { x: 5, y: 9 },
                end: Position { x: 5, y: -100 },
            },
            Segment {
                start: Position { x: 7, y: 9 },
                end: Position { x: 7, y: -100 },
            },
            Segment {
                start: Position { x: 9, y: 9 },
                end: Position { x: 9, y: -100 },
            },
            Segment {
                start: Position { x: 2, y: 100 },
                end: Position { x: 2, y: 1 },
            },
            Segment {
                start: Position { x: 4, y: 100 },
                end: Position { x: 4, y: 1 },
            },
            Segment {
                start: Position { x: 6, y: 100 },
                end: Position { x: 6, y: 1 },
            },
            Segment {
                start: Position { x: 8, y: 100 },
                end: Position { x: 8, y: 1 },
            },
        ]
    }

    #[test]
    fn test_maze() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 10, y: 10 };
        let segments = maze();
        let got = find_path(start, &segments, end);
        assert_eq!(got, Some(vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 9 },
            Position { x: 2, y: 1 },
            Position { x: 3, y: 9 },
            Position { x: 4, y: 1 },
            Position { x: 5, y: 9 },
            Position { x: 6, y: 1 },
            Position { x: 7, y: 9 },
            Position { x: 8, y: 1 },
            Position { x: 9, y: 9 },
            Position { x: 10, y: 10 },
        ]))
    }
}
