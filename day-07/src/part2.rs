#[derive(Debug)]
struct Equation {
    pub input: Vec<i64>,
    pub result: i64,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Equation {
    pub fn solve(&self) -> Option<i64> {
        let n_operators = (self.input.len() - 1).try_into().ok()?;
        for i in 0..3u32.pow(n_operators) {
            let start = *self.input.first()?;
            let result = self
                .input
                .iter()
                .skip(1)
                .zip(Equation::get_operators(i, n_operators))
                .fold(start, |acc, (n, op)| match op {
                    Operator::Add => acc + n,
                    Operator::Multiply => acc * n,
                    Operator::Concatenate => {
                        format!("{acc}{n}").parse::<i64>().unwrap_or(acc)
                    }
                });

            if result == self.result {
                return Some(result);
            }
        }
        None
    }

    fn get_operators(
        i: u32,
        n_operators: u32,
    ) -> impl Iterator<Item = Operator> {
        (0..n_operators)
            .map(move |j| i / (3u32.pow(j)))
            .map(|value| match value % 3 {
                0 => Operator::Add,
                1 => Operator::Multiply,
                2 => Operator::Concatenate,
                _ => unreachable!(),
            })
            .rev()
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
        assert_eq!(result.unwrap(), "11387");
    }
}
