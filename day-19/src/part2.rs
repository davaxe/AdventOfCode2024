#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let pieces = lines
        .next()?
        .split(',')
        .map(str::trim)
        .collect::<Vec<&str>>();

    let count = lines
        .skip(1)
        .filter_map(|target| combinations(target, &pieces))
        .sum::<u64>();

    Some(count.to_string())
}

fn combinations(target: &str, pieces: &[&str]) -> Option<u64> {
    let n = target.len();
    let mut dp = vec![0u64; n + 1];
    dp[n] = 1;

    for i in (0..n).rev() {
        let mut ways: u64 = 0;
        for &piece in pieces {
            let p_len = piece.len();
            if i + p_len <= n && &target[i..i + p_len] == piece {
                ways = ways.saturating_add(dp[i + p_len]);
            }
        }
        dp[i] = ways;
    }

    // If dp[0] is zero, it means “impossible to build” ⇒ return None
    (dp[0] > 0).then(|| dp[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "16");
    }
}
