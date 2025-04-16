#[must_use]
pub fn task(_input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "117440");
    }
}
