use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) enum Tile {
    Open,
    Obstacle,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Turn the direction 90 degrees to the right
    pub fn turn(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    /// Move the position in the direction
    pub fn forward(self, pos: &(usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => Some((pos.0, pos.1.checked_sub(1)?)),
            Direction::Down => Some((pos.0, pos.1 + 1)),
            Direction::Left => Some((pos.0.checked_sub(1)?, pos.1)),
            Direction::Right => Some((pos.0 + 1, pos.1)),
        }
    }

    /// Move the position in the reverse direction.
    pub fn reverse(self, pos: &(usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => Some((pos.0, pos.1 + 1)),
            Direction::Down => Some((pos.0, pos.1.checked_sub(1)?)),
            Direction::Left => Some((pos.0 + 1, pos.1)),
            Direction::Right => Some((pos.0.checked_sub(1)?, pos.1)),
        }
    }
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut pos = (0, 0);
    let mut direction = Direction::Down;
    let map: HashMap<(usize, usize), Tile> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut map, (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    map.insert(
                        (x, y),
                        match c {
                            '.' => Tile::Open,
                            '#' => Tile::Obstacle,
                            'v' => {
                                pos = (x, y);
                                direction = Direction::Down;
                                Tile::Open
                            }
                            '^' => {
                                pos = (x, y);
                                direction = Direction::Up;
                                Tile::Open
                            }
                            '<' => {
                                pos = (x, y);
                                direction = Direction::Left;
                                Tile::Open
                            }
                            '>' => {
                                pos = (x, y);
                                direction = Direction::Right;
                                Tile::Open
                            }
                            _ => unreachable!(),
                        },
                    );
                });
                map
            });

    let mut visited = HashSet::new();
    let mut unique_positions = 0;
    while let Some(tile) = map.get(&pos) {
        match tile {
            Tile::Open => {
                if !visited.contains(&pos) {
                    unique_positions += 1;
                }
                visited.insert(pos);
                pos = direction.forward(&pos)?;
            }
            Tile::Obstacle => {
                pos = direction.reverse(&pos)?;
                direction = direction.turn();
                pos = direction.forward(&pos)?;
            }
        }
    }

    Some(unique_positions.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "41");
    }
}
