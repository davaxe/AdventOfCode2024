use std::collections::HashMap;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut left: Vec<usize> = vec![];
    let mut right: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let first = numbers.next()?.parse().ok()?;
        let second = numbers.next()?.parse().ok()?;
        left.push(first);
        right.entry(second).and_modify(|n| *n += 1).or_insert(1);
    }

    Some(
        left.iter()
            .map(|n| right.get(n).unwrap_or(&0) * n)
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
