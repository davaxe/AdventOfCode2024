use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    input.lines().zip(0i32..).for_each(|(line, y)| {
        line.chars().zip(0i32..).for_each(|(c, x)| {
            if c != '.' {
                antennas.entry(c).or_default().push((x, y));
            }
            width = width.max(x);
        });
        height = height.max(y);
    });

    let positions = antennas
        .values()
        .flat_map(|positions| antinodes(positions, width, height))
        .collect::<HashSet<_>>();

    Some(positions.len().to_string())
}

fn antinodes(
    positions: &[(i32, i32)],
    width: i32,
    height: i32,
) -> impl Iterator<Item = (i32, i32)> + '_ {
    positions
        .iter()
        .combinations(2)
        .map(|pair| match pair.as_slice() {
            [(x1, y1), (x2, y2)] => ((*x1, *y1), (*x2, *y2)),
            _ => unreachable!(),
        })
        .flat_map(move |(a, b)| {
            antinode(a, b, width, height).chain(antinode(b, a, width, height))
        })
}

fn antinode(
    (x1, y1): (i32, i32),
    (x2, y2): (i32, i32),
    width: i32,
    height: i32,
) -> impl Iterator<Item = (i32, i32)> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (0..)
        .map(move |n| (x1 - dx * n, y1 - dy * n))
        .take_while(move |(x, y)| {
            *x >= 0 && *x <= width && *y >= 0 && *y <= height
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "34");
    }

    #[test]
    fn alternative_test_task() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "9");
    }
}
