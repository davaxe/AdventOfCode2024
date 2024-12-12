use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemorySlot {
    Free,
    Used(usize),
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut disk_map = input
        .char_indices()
        .filter_map(|(i, c)| match c.to_digit(10) {
            Some(x) if i % 2 == 0 => {
                Some(iter::repeat(MemorySlot::Used(i / 2)).take(x as usize))
            }
            Some(x) if i % 2 != 0 => {
                Some(iter::repeat(MemorySlot::Free).take(x as usize))
            }
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut move_index = next_move_index(&disk_map, disk_map.len())?;

    for i in 0..disk_map.len() {
        if let MemorySlot::Used(_) = disk_map[i] {
            continue;
        }

        if move_index < i {
            break;
        }

        disk_map.swap(move_index, i);
        move_index = next_move_index(&disk_map, move_index)?;
    }

    let result: usize = disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if let MemorySlot::Used(id) = x {
                Some(id * i)
            } else {
                None
            }
        })
        .sum();

    Some(result.to_string())
}

fn next_move_index(
    disk_map: &[MemorySlot],
    move_index: usize,
) -> Option<usize> {
    (0..move_index)
        .rev()
        .find(|&i| matches!(disk_map[i], MemorySlot::Used(_)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "1928");
    }
}
