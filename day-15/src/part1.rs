use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Box,
    Robot,
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let input = input.replace("\r\n", "\n");
    let mut parts = input.split("\n\n");
    let map = parts.next()?;
    let moves = parts.next()?.chars().filter_map(|c| match c {
        '^' => Some(Move::Up),
        'v' => Some(Move::Down),
        '<' => Some(Move::Left),
        '>' => Some(Move::Right),
        _ => None,
    });

    let (mut robot_pos, mut map) = map.lines().zip(0i32..).fold(
        ((0, 0), HashMap::new()),
        |(mut start_pos, mut map), (line, y)| {
            for (c, x) in line.chars().zip(0i32..) {
                let tile = match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => {
                        start_pos = (x, y);
                        Tile::Robot
                    }
                    _ => Tile::Empty,
                };
                map.insert((x, y), tile);
            }
            (start_pos, map)
        },
    );

    for next_move in moves {
        let next_pos = next_pos(robot_pos, next_move);
        let Some(&tile) = map.get(&next_pos) else {
            continue;
        };

        match tile {
            Tile::Empty => {
                map.insert(robot_pos, Tile::Empty);
                map.insert(next_pos, Tile::Robot);
                robot_pos = next_pos;
            }
            Tile::Box if shift_boxes(&mut map, next_pos, next_move) => {
                map.insert(robot_pos, Tile::Empty);
                map.insert(next_pos, Tile::Robot);
                robot_pos = next_pos;
            }
            Tile::Robot => unreachable!(),
            _ => continue,
        }
    }

    let result: i32 = map
        .into_iter()
        .filter_map(|((x, y), tile)| match tile {
            Tile::Box => Some(x + 100 * y),
            _ => None,
        })
        .sum();

    Some(result.to_string())
}

fn next_pos((x, y): (i32, i32), mov: Move) -> (i32, i32) {
    match mov {
        Move::Up => (x, y - 1),
        Move::Down => (x, y + 1),
        Move::Left => (x - 1, y),
        Move::Right => (x + 1, y),
    }
}

fn shift_boxes(
    map: &mut HashMap<(i32, i32), Tile>,
    pos: (i32, i32),
    mov: Move,
) -> bool {
    let mut boxes = 0;
    let mut pos_t = pos;
    while let Some(Tile::Box) = map.get(&pos_t) {
        pos_t = next_pos(pos_t, mov);
        boxes += 1;
    }

    if let Some(Tile::Empty) = map.get(&pos_t) {
        let mut pos_t = pos;
        for _ in 0..boxes {
            pos_t = next_pos(pos_t, mov);
            map.insert(pos_t, Tile::Box);
        }
        return true;
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
        assert_eq!(result.unwrap(), "10092");
    }

    #[test]
    fn alternative_test_task() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "2028");
    }
}
