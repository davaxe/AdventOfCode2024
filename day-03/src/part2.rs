use crate::parser::{operations, Operation};

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, operations) = operations(input).ok()?;
    let mut enable = true;
    let mut sum = 0;
    for operation in operations {
        match operation {
            Operation::Enable => enable = true,
            Operation::Disable => enable = false,
            Operation::Multiplication((a, b)) => {
                if enable {
                    sum += a * b;
                }
            }
        }
    }
    Some(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: Implement test, and remove ignore.
    fn test_task() {
        let input = include_str!("../example_part2.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "48");
    }
}
