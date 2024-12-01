#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let first = numbers.next()?.parse().ok()?;
        let second = numbers.next()?.parse().ok()?;
        left.push(first);
        right.push(second);
    }

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
