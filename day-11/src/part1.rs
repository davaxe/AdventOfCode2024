#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut stones = input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        let mut new = Vec::with_capacity(stones.len());
        for stone in stones {
            match format!("{stone}").as_str() {
                "0" => new.push(1),
                n if n.len() % 2 == 0 => {
                    let first_halp = n[..n.len() / 2].parse::<u64>().ok()?;
                    let second_half = n[n.len() / 2..].parse::<u64>().ok()?;
                    new.push(first_halp);
                    new.push(second_half);
                }
                n => new.push(n.parse::<u64>().ok()? * 2024),
            }
        }
        stones = new;
    }
    Some(stones.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "55312");
    }
}
