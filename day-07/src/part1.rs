#[derive(Debug)]
struct Equation {
    pub input: Vec<i64>,
    pub result: i64,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Equation {
    pub fn solve(&self) -> Option<i64> {
        let n_operators = self.input.len() - 1;
        for i in 0..1 << n_operators {
            let operators =
                (0..n_operators).rev().map(|bit| match (i >> bit) & 1 {
                    0 => Operator::Add,
                    1 => Operator::Multiply,
                    _ => unreachable!(),
                });

            let start = *self.input.first()?;
            let result = self.input.iter().skip(1).zip(operators).fold(
                start,
                |acc, (n, op)| match op {
                    Operator::Add => acc + n,
                    Operator::Multiply => acc * n,
                },
            );

            if result == self.result {
                return Some(result);
            }
        }
        None
    }
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let result: i64 = input
        .lines()
        .filter_map(|line| {
            match line
                .split(':')
                .map(str::trim)
                .collect::<Vec<&str>>()
                .as_slice()
            {
                [result, input] => Equation {
                    input: input
                        .split(' ')
                        .filter_map(|n| n.parse::<i64>().ok())
                        .collect(),
                    result: result.parse::<i64>().ok()?,
                }
                .solve(),
                _ => None,
            }
        })
        .sum();

    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "3749");
    }
}
