use itertools::Itertools;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let result = input
        .lines()
        .filter_map(|report| {
            let numbers: Vec<i32> = report
                .split_whitespace()
                .filter_map(|number| number.parse().ok())
                .collect();

            if is_safe(numbers.clone()).is_some() {
                return Some(());
            }

            let len = numbers.len();
            numbers
                .into_iter()
                .combinations(len - 1)
                .any(|comb| is_safe(comb).is_some())
                .then_some(())
        })
        .count();

    Some(result.to_string())
}

fn is_safe<T>(numbers: T) -> Option<()>
where
    T: IntoIterator<Item = i32>,
{
    enum State {
        Unknown,
        Increasing,
        Decreasing,
    }
    let mut status = State::Unknown;
    let mut numbers_iter = numbers.into_iter();
    let mut current = numbers_iter.next()?;
    for next in numbers_iter {
        match status {
            State::Unknown if (current + 1..current + 4).contains(&next) => {
                status = State::Increasing;
                current = next;
            }
            State::Unknown if (current - 3..current).contains(&next) => {
                status = State::Decreasing;
                current = next;
            }
            State::Increasing if (current + 1..current + 4).contains(&next) => {
                current = next;
            }
            State::Decreasing if (current - 3..current).contains(&next) => {
                current = next;
            }
            _ => return None,
        }
    }
    Some(())
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
