use std::collections::HashSet;

use itertools::Itertools;

use crate::part1::Direction;

struct Map {
    pub obstacles: HashSet<(usize, usize)>,
    pub width: usize,
    pub height: usize,
    pub start_pos: (usize, usize),
    pub start_dir: Direction,
}

#[must_use]
pub fn task(input: &str, width: usize, height: usize) -> Option<String> {
    let mut start_pos = (0, 0);
    let mut start_dir = Direction::Down;
    let mut obstacles: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                'v' => {
                    start_pos = (x, y);
                    start_dir = Direction::Down;
                    None
                }
                '^' => {
                    start_pos = (x, y);
                    start_dir = Direction::Up;
                    None
                }
                '<' => {
                    start_pos = (x, y);
                    start_dir = Direction::Left;
                    None
                }
                '>' => {
                    start_pos = (x, y);
                    start_dir = Direction::Right;
                    None
                }
                _ => None,
            })
        })
        .collect();

    let mut map = Map {
        obstacles,
        width,
        height,
        start_pos,
        start_dir,
    };

    let result = (0..width)
        .cartesian_product(0..height)
        .filter(|pos| {
            map.obstacles.insert(*pos);
            let infinite_loop = check_infinite_loop(&map, *pos).is_some();
            map.obstacles.remove(pos);
            infinite_loop
        })
        .count();

    Some(result.to_string())
}

fn check_infinite_loop(map: &Map, obstacle: (usize, usize)) -> Option<()> {
    [
        ((obstacle.0, obstacle.1 + 1), Direction::Up),
        ((obstacle.0, obstacle.1.checked_sub(1)?), Direction::Down),
        ((obstacle.0.checked_sub(1)?, obstacle.1), Direction::Right),
        ((obstacle.0 + 1, obstacle.1), Direction::Left),
    ]
    .into_iter()
    .any(|(start, direction)| backtrack(map, start, direction).is_some())
    .then(|| ())
}

fn backtrack(
    map: &Map,
    start: (usize, usize),
    direction: Direction,
) -> Option<()> {
    let mut pos = start;
    let mut visited = HashSet::new();
    loop {
        pos = direction.reverse(&pos)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input, 10, 10);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "6");
    }
}
