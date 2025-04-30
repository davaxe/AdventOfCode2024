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

    // Distance to start from any other reachable tile
    let to_start = bfs(start, &map);
    // Distance to end from any other reachable tile
    let to_end = bfs(end, &map);

    let baseline = *to_start.get(&end)?;

    let count = map
        .iter()
        .filter(|(_, &tile)| tile == Tile::Open)
        .flat_map(|(&cheat_start, _)| {
            find_cheat_path(cheat_start, &map, 20)
                .into_iter()
                .map(move |cheat_end| (cheat_start, cheat_end))
        })
        .filter_map(|(cheat_start, (ex, ey, d))| {
            let &d_start = to_start.get(&cheat_start)?;
            let &d_end = to_end.get(&(ex, ey))?;
            let distance = d_start + d_end + d;
            let saved = baseline - distance;

            if saved >= save_more_than {
                Some((cheat_start, (ex, ey)))
            } else {
                None
            }
        })
        .count();

    Some(count.to_string())
}

fn find_cheat_path(
    start: (i64, i64),
    map: &HashMap<(i64, i64), Tile>,
    max_dist: i64,
) -> HashSet<(i64, i64, i64)> {
    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);
    let mut exits = HashSet::new();

    while let Some(((x, y), d)) = queue.pop_front() {
        if d == max_dist {
            continue;
        }

        for (dx, dy) in DIR {
            let next = (x + dx, y + dy);
            if !map.contains_key(&next) || seen.contains(&next) {
                continue;
            }

            let nd = d + 1;
            if nd > max_dist {
                continue;
            }

            seen.insert(next);
            if matches!(map[&next], Tile::Open) && next != start {
                exits.insert((next.0, next.1, nd));
            }
            queue.push_back((next, nd));
        }
    }
    exits
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
        let result = task(input, 70);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "41");
    }
}
