#[must_use]
pub fn task(_input: &str, _width: usize, _height: usize) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Not implemented"]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input, 10, 10);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "6");
    }
}
