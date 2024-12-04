use itertools::Itertools;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let width = map.len();
    let height = map[0].len();

    let result = (0..height)
        .cartesian_product(0..width)
        .filter(|(x, y)| map[*y][*x] == 'X')
        .flat_map(|(x, y)| find_words_in_directions(&map, x, y, 4))
        .filter(|word| word == "XMAS")
        .count();

    Some(result.to_string())
}

/// Find words in all directions from a given point.
///
/// Returns an iterator of matching words of maximum length `size`.
fn find_words_in_directions(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    size: usize,
) -> impl Iterator<Item = String> + '_ {
    [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .into_iter()
    .filter_map(move |(dx, dy)| word_in_direction(grid, x, y, dx, dy, size))
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
        assert_eq!(result.unwrap(), "18");
    }
}
