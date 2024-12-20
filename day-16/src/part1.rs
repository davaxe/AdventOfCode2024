use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
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

    let mut queue = BinaryHeap::new();
    queue.push((
        Reverse(0),
        State {
            pos: start,
            direction: Direction::East,
        },
    ));
    let mut visited = HashSet::new();

    while let Some((Reverse(cost), next)) = queue.pop() {
        if visited.contains(&next) {
            continue;
        }

        if next.pos == end {
            return Some(cost.to_string());
        }

        for (neighbor_state, extra_cost) in next
            .neighboring_states()
            .filter(|(state, _)| !walls.contains(&state.pos))
        {
            queue.push((Reverse(cost + extra_cost), neighbor_state));
        }
        visited.insert(next);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "7036");
    }
}
