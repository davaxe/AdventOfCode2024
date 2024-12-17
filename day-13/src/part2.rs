use self::parser::ClawMachine;
use crate::parser;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, machines) = parser::claw_machines(input).ok()?;
    let result: i128 = machines
        .into_iter()
        .filter_map(|machine| minimal_cost(&machine, 3, 1))
        .sum();

    Some(result.to_string())
}

fn minimal_cost(
    machine: &ClawMachine,
    cost_a: i32,
    cost_b: i32,
) -> Option<i128> {
    // Convert to i128 to prevent overflow
    let (a11, a21) = machine.button_a;
    let (a11, a21) = (i128::from(a11), i128::from(a21));
    let (a12, a22) = machine.button_b;
    let (a12, a22) = (i128::from(a12), i128::from(a22));
    let (b1, b2) = machine.price_location;
    let b1 = i128::from(b1) + 10_000_000_000_000;
    let b2 = i128::from(b2) + 10_000_000_000_000;
    let det = a11 * a22 - a12 * a21;

    if det == 0 {
        return None;
    }

    // Solve for s and t
    let t = (a11 * b2 - a21 * b1) / det;
    let s = (a22 * b1 - a12 * b2) / det;

    if (a11 * s + a12 * t != b1) || (a21 * s + a22 * t != b2) {
        return None;
    }

    let (cost_a, cost_b) = (i128::from(cost_a), i128::from(cost_b));
    Some(cost_a * s + cost_b * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "No test for part 2"]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }
}
