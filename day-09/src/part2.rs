use std::collections::HashSet;
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemorySlot {
    Free,
    Used(usize),
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut free_slots: Vec<(usize, usize)> = Vec::new();
    let mut sizes: Vec<usize> = Vec::new();
    let mut index = 0;
    let mut disk_map = input
        .char_indices()
        .filter_map(|(i, c)| match c.to_digit(10) {
            Some(x) if i % 2 == 0 => {
                sizes.push(x as usize);
                index += x as usize;
                Some(iter::repeat_n(MemorySlot::Used(i / 2), x as usize))
            }
            Some(x) if i % 2 != 0 => {
                free_slots.push((index, x as usize));
                index += x as usize;
                Some(iter::repeat_n(MemorySlot::Free, x as usize))
            }
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>();

    defrag(&mut disk_map, &sizes, &mut free_slots);

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

fn defrag(
    disk_map: &mut [MemorySlot],
    sizes: &[usize],
    free_slots: &mut Vec<(usize, usize)>,
) {
    let mut i = disk_map.len() - 1;
    let mut already_moved = HashSet::new();
    while i != 0 {
        if let MemorySlot::Used(id) = disk_map[i] {
            let size = sizes[id];
            if already_moved.contains(&id) {
                i = i.saturating_sub(size);
                continue;
            }

            let Some(slot_idx) =
                free_slots.iter().position(|&(start_index, free_size)| {
                    free_size >= size && start_index <= i
                })
            else {
                i = i.saturating_sub(1);
                continue;
            };

            let (start, free_size) = free_slots[slot_idx];
            if free_size == size {
                free_slots.remove(slot_idx);
            } else {
                free_slots[slot_idx] = (start + size, free_size - size);
            }

            (start..start + size)
                .zip((i - size + 1..=i).rev())
                .for_each(|(from, to)| {
                    disk_map.swap(from, to);
                });

            i = i.saturating_sub(size);
            already_moved.insert(id);
        } else {
            i -= 1;
        }
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
        assert_eq!(result.unwrap(), "2858");
    }

    #[test]
    fn alternative_test_task() {
        let input = "12345";
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "132");
    }
}
