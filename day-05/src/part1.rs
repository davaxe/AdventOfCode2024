use std::collections::{HashMap, HashSet};

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut parts = input.split("\r\n\r\n");
    let rules_it = parts.next()?;
    let page_orders: Vec<Vec<i32>> = parts
        .next()?
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.trim().parse().ok())
                .collect()
        })
        .collect();

    // all items in rules[key] are greater than key
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    rules_it
        .lines()
        .filter_map(|rule| {
            match rule
                .split('|')
                .filter_map(|num| num.trim().parse().ok())
                .collect::<Vec<i32>>()
                .as_slice()
            {
                [first, second, ..] => Some((*first, *second)),
                _ => None,
            }
        })
        .for_each(|(before, after)| {
            rules.entry(before).or_default().insert(after);
        });

    let result: i32 = page_orders
        .into_iter()
        .filter(|pages| correctly_ordered_pages(&rules, pages))
        .map(|pages| pages[pages.len() / 2])
        .sum();

    Some(result.to_string())
}

fn correctly_ordered_pages(
    rules: &HashMap<i32, HashSet<i32>>,
    pages: &[i32],
) -> bool {
    pages.is_sorted_by(|a, b| match compare(rules, *a, *b) {
        std::cmp::Ordering::Equal | std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Greater => false,
    })
}

fn compare(
    rules: &HashMap<i32, HashSet<i32>>,
    a: i32,
    b: i32,
) -> std::cmp::Ordering {
    match rules.get(&a) {
        Some(set) if set.contains(&b) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Greater,
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
        assert_eq!(result.unwrap(), "143");
    }
}
