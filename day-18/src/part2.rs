use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

#[must_use]
pub fn task(input: &str, end_pos: (u8, u8)) -> Option<String> {
    let bytes_location = input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .flat_map(str::parse::<u8>)
                .collect_tuple::<(u8, u8)>()
        })
        .collect_vec();

    let mut corrupted_memory: HashSet<(u8, u8)> =
        HashSet::with_capacity(bytes_location.len());

    for pos in bytes_location {
        corrupted_memory.insert(pos);

        if find_solution((0, 0), end_pos, &corrupted_memory).is_none() {
            return Some(format!("{},{}", pos.0, pos.1));
        }
    }

    None
}

fn find_solution(
    (sx, sy): (u8, u8),
    (ex, ey): (u8, u8),
    corrupted_memory: &HashSet<(u8, u8)>,
) -> Option<u32> {
    let mut queue: BinaryHeap<(Reverse<u32>, u8, u8)> = BinaryHeap::new();
    queue.push((Reverse(0), sx, sy));
    let mut visited = HashSet::new();

    while let Some((Reverse(steps), x, y)) = queue.pop() {
        if (x, y) == (ex, ey) {
            return Some(steps);
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        for (nx, ny) in [
            (x.checked_add(1), Some(y)),
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_add(1)),
            (Some(x), y.checked_sub(1)),
        ]
        .into_iter()
        .filter_map(|(ox, oy)| match (ox, oy) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }) {
            if nx > ex || ny > ey {
                continue;
            }

            if corrupted_memory.contains(&(nx, ny)) {
                continue;
            }
            queue.push((Reverse(steps + 1), nx, ny));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input, (6, 6));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "6,1");
    }
}
