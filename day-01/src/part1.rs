#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let first = numbers.next()?.parse().ok()?;
        let second = numbers.next()?.parse().ok()?;
        left.push(first);
        right.push(second);
    }

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
