use crate::parser::multiplications;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, multiplications) = multiplications(input).ok()?;

    multiplications
        .into_iter()
        .map(|(a, b)| a * b)
        .sum::<i32>()
        .to_string()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example_part1.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "161");
    }
}
