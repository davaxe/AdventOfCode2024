#[must_use]
pub fn task(_input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: Implement test, and remove ignore.
    #[ignore = "Not implemented"]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }
}
