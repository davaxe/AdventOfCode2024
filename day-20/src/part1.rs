use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
}

const DIR: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[must_use]
pub fn task(input: &str, save_more_than: i64) -> Option<String> {
    let mut start = None;
    let mut end = None;
    let mut map = HashMap::new();

    for (y_idx, line) in input.lines().enumerate() {
        let y: i64 = y_idx.try_into().ok()?;
        for (x_idx, c) in line.char_indices() {
            let x: i64 = x_idx.try_into().ok()?;
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                'S' => {
                    start = Some((x, y));
                    Tile::Open
                }
                'E' => {
                    end = Some((x, y));
                    Tile::Open
                }
                _ => unreachable!(),
            };
            map.insert((x, y), tile);
        }
    }

    let start = start?;
    let end = end?;

    let to_start = bfs(start, &map);
    let to_end = bfs(end, &map);

    let baseline = *to_start.get(&end)?;

    let mut unique = HashSet::new();
    let mut count = 0;

    for (&entry, _) in map.iter().filter(|(_, &tile)| tile == Tile::Open) {
        let Some(&d_start) = to_start.get(&entry) else {
            continue;
        };

        for (dx1, dy1) in DIR {
            let mid = (entry.0 + dx1, entry.1 + dy1);
            if !map.contains_key(&mid) {
                continue;
            }

            for (dx2, dy2) in DIR {
                let exit = (mid.0 + dx2, mid.1 + dy2);
                if !matches!(map.get(&exit), Some(Tile::Open)) {
                    continue;
                }

                if let Some(&d_end) = to_end.get(&exit) {
                    let total = d_start + 2 + d_end;
                    if total < baseline {
                        let saved = baseline - total;
                        if saved >= save_more_than
                            && unique.insert((entry, exit))
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    Some(count.to_string())
}

fn bfs(
    start: (i64, i64),
    map: &HashMap<(i64, i64), Tile>,
) -> HashMap<(i64, i64), i64> {
    let mut d = HashMap::new();

    d.insert(start, 0);

    let mut queue = VecDeque::from([start]);

    while let Some((x, y)) = queue.pop_front() {
        let nd = d[&(x, y)] + 1;

        for (dx, dy) in DIR {
            let next = (x + dx, y + dy);
            if matches!(map.get(&next), Some(Tile::Open))
                && !d.contains_key(&next)
            {
                d.insert(next, nd);
                queue.push_back(next);
            }
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input, 36);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "4");
    }
}
