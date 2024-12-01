/// Processes the input and returns the result as a string.
///
/// # Panics
///
/// This function will panic if the input lines do not contain at least two
/// whitespace-separated numbers.
#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut left = vec![];
    let mut right = vec![];
    input.lines().for_each(|line| {
        let mut numbers = line.split_whitespace();
        left.push(numbers.next().unwrap().parse::<i32>().unwrap());
        right.push(numbers.next().unwrap().parse::<i32>().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    Some(
        right
            .iter()
            .zip(left.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "11");
    }
}
