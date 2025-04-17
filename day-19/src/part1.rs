use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

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
        .filter(|target| check(target, &pieces))
        .count();

    Some(count.to_string())
}

fn check(target: &str, pieces: &[&str]) -> bool {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(target.len()), target.to_string()));

    let mut visited = HashSet::new();

    while let Some((_, current)) = queue.pop() {
        if current.is_empty() {
            return true;
        }

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        for piece in pieces {
            if let Some(new_string) = current.strip_prefix(piece) {
                queue.push((Reverse(new_string.len()), new_string.to_string()));
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "6");
    }
}
