use std::collections::HashMap;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let height_map: HashMap<(i32, i32), u32> = input
        .lines()
        .zip(0i32..)
        .flat_map(|(line, y)| {
            line.chars()
                .zip(0i32..)
                .filter_map(move |(c, x)| c.to_digit(10).map(|d| ((x, y), d)))
        })
        .collect();

    let result: u32 = height_map
        .iter()
        .filter(|&(_, &height)| (height == 0))
        .map(|(pos, _)| rating(*pos, &height_map))
        .sum();

    Some(result.to_string())
}

fn rating(
    start_position: (i32, i32),
    height_map: &HashMap<(i32, i32), u32>,
) -> u32 {
    let neighbors = |(x, y)| {
        let height = height_map[&(x, y)];
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter_map(|pos| height_map.get(&pos).map(|&height| (pos, height)))
            .filter(move |&(_, neighbor_height)| {
                neighbor_height.checked_sub(1) == Some(height)
            })
    };

    let mut trails = 0;
    let mut queue = vec![start_position];
    while let Some(next) = queue.pop() {
        if height_map[&next] == 9 {
            trails += 1;
        }

        for (pos, _) in neighbors(next) {
            queue.push(pos);
        }
    }

    trails
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "81");
    }
}
