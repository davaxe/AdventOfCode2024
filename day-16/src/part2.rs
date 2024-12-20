use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(self, clockwise: bool) -> Self {
        match self {
            Self::North if clockwise => Self::East,
            Self::North => Self::West,
            Self::East if clockwise => Self::South,
            Self::East => Self::North,
            Self::South if clockwise => Self::West,
            Self::South => Self::East,
            Self::West if clockwise => Self::North,
            Self::West => Self::South,
        }
    }

    fn offset(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct State {
    pos: (i32, i32),
    direction: Direction,
}

impl State {
    fn neighboring_states(&self) -> impl Iterator<Item = (Self, i32)> + '_ {
        let rotation = [true, false].into_iter().map(move |clockwise| {
            (
                Self {
                    pos: self.pos,
                    direction: self.direction.rotate(clockwise),
                },
                1000,
            )
        });

        let (dx, dy) = self.direction.offset();
        let next_pos = (self.pos.0 + dx, self.pos.1 + dy);
        rotation.chain(std::iter::once((
            Self {
                pos: next_pos,
                direction: self.direction,
            },
            1,
        )))
    }
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (walls, start, end) = input.lines().zip(0i32..).fold(
        (HashSet::new(), (0, 0), (0, 0)),
        |(mut map, mut start, mut end), (line, y)| {
            for (c, x) in line.chars().zip(0i32..) {
                match c {
                    '#' => {
                        map.insert((x, y));
                    }
                    'E' => {
                        end = (x, y);
                    }
                    'S' => {
                        start = (x, y);
                    }
                    _ => {}
                }
            }
            (map, start, end)
        },
    );

    let mut heap = BinaryHeap::new();
    heap.push((
        Reverse(0),
        State {
            pos: start,
            direction: Direction::East,
        },
        vec![start],
    ));
    let mut visited: HashMap<State, i32> = HashMap::new();
    let mut tiles = HashSet::new();
    let mut lowest_score = None;

    while let Some((Reverse(cost), state, path)) = heap.pop() {
        if let Some(lowest_cost) = lowest_score {
            if cost > lowest_cost {
                break;
            }
        }

        if state.pos == end {
            lowest_score = Some(cost);
            tiles.extend(path);
            continue;
        }

        if !can_visit(state, cost, &mut visited) {
            continue;
        }

        for (next_state, step_cost) in state.neighboring_states() {
            if walls.contains(&next_state.pos)
                || !can_visit(next_state, cost + step_cost, &mut visited)
            {
                continue;
            }

            let mut next_path = path.clone();
            next_path.push(next_state.pos);
            heap.push((Reverse(cost + step_cost), next_state, next_path));
        }
    }

    Some(tiles.len().to_string())
}

fn can_visit(
    state: State,
    cost: i32,
    visited: &mut HashMap<State, i32>,
) -> bool {
    if let Some(&prev_cost) = visited.get(&state) {
        if prev_cost < cost {
            return false;
        }
    }
    visited.insert(state, cost);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "45");
    }
}
