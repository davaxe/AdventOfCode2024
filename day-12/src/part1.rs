use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let plot: HashMap<(i32, i32), char> = input
        .lines()
        .zip(0i32..)
        .flat_map(|(line, y)| {
            line.chars().zip(0i32..).map(move |(c, x)| ((x, y), c))
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::with_capacity(plot.len());
    let mut regions: Vec<Region> = Vec::new();
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push((0, 0));

    while let Some((x, y)) = queue.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let search_result = search_region(&plot, x, y, *plot.get(&(x, y))?);
        regions.push(search_result.region);
        visited.extend(search_result.visited);
        queue.extend(search_result.next_regions);
    }

    let result: u32 = regions.into_iter().map(|r| r.area * r.perimeter).sum();

    Some(result.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Region {
    area: u32,
    perimeter: u32,
    id: char,
}

#[derive(Debug)]
struct SearchResult {
    // The region found
    region: Region,
    // The next regions to search, i.e. connected regions with different
    // characters
    next_regions: HashSet<(i32, i32)>,
    // The regions visited during the search
    visited: HashSet<(i32, i32)>,
}

fn search_region(
    plot: &HashMap<(i32, i32), char>,
    x: i32,
    y: i32,
    id: char,
) -> SearchResult {
    let neighbors = |(x, y)| {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .map(|(x, y)| (x, y, plot.get(&(x, y))))
    };

    let mut next_regions: HashSet<(i32, i32)> = HashSet::new();
    let mut queue = vec![(x, y)];
    let mut area = 0;
    let mut perimeter = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while let Some(next) = queue.pop() {
        if visited.contains(&next) {
            continue;
        }

        visited.insert(next);
        area += 1;

        for (x_n, y_n, c) in neighbors(next) {
            let Some(&c) = c else {
                perimeter += 1;
                continue;
            };

            if c == id {
                queue.push((x_n, y_n));
            } else {
                perimeter += 1;
                next_regions.insert((x_n, y_n));
            }
        }
    }

    SearchResult {
        region: Region {
            area,
            perimeter,
            id,
        },
        next_regions,
        visited,
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
        assert_eq!(result.unwrap(), "1930");
    }
}
