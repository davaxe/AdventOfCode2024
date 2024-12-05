use std::collections::HashMap;

use itertools::Itertools;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let width = map.len();
    let height = map[0].len();

    let mut count: HashMap<(i128, i128), u32> = HashMap::new();

    (0..height)
        .cartesian_product(0..width)
        .filter(|(x, y)| map[*y][*x] == 'M')
        .flat_map(|(x, y)| {
            find_matching_words_in_directions(&map, x, y, "MAS")
                .map(move |(dx, dy, _)| (x as i128 + dx, y as i128 + dy))
        })
        .for_each(|(x, y)| {
            *count.entry((x, y)).or_insert(0) += 1;
        });

    let overlap = count.values().filter(|&&v| v > 1).count();
    Some(overlap.to_string())
}

/// Find matching words in all diagonal directions from a given point.
///
/// Returns an iterator of tuples containing the direction and the matching
/// word.
fn find_matching_words_in_directions<'a>(
    grid: &'a [Vec<char>],
    x: usize,
    y: usize,
    matches: &'a str,
) -> impl Iterator<Item = (i128, i128, String)> + 'a {
    let size = matches.len();
    [(1, 1), (-1, 1), (-1, -1), (1, -1)].into_iter().filter_map(
        move |(dx, dy)| match word_in_direction(grid, x, y, dx, dy, size) {
            Some(word) if word == matches => Some((dx, dy, word)),
            _ => None,
        },
    )
}

/// Find a word of give `size` in a given direction from a point.
///
/// Returns the word if a word of length `size` is found, otherwise `None`.
fn word_in_direction(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    dx: i128,
    dy: i128,
    size: usize,
) -> Option<String> {
    let mut word = String::new();
    let mut x = x as i128;
    let mut y = y as i128;
    loop {
        let x_idx: usize = x.try_into().ok()?;
        let y_idx: usize = y.try_into().ok()?;
        let char = *grid.get(y_idx)?.get(x_idx)?;
        word.push(char);
        if word.len() == size {
            return Some(word);
        }
        x += dx;
        y += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "9");
    }
}
