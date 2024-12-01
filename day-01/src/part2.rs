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
        left.push(numbers.next().unwrap().parse::<usize>().unwrap());
        right.push(numbers.next().unwrap().parse::<usize>().unwrap());
    });

    Some(
        left.iter()
            .map(|n| right.iter().filter(|&m| m == n).count() * n)
            .sum::<usize>()
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
        assert_eq!(result.unwrap(), "31");
    }
}
