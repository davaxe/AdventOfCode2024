use self::parser::ClawMachine;
use crate::parser;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, machines) = parser::claw_machines(input).ok()?;
    let result: i32 = machines
        .into_iter()
        .filter_map(|machine| minimal_cost(&machine, 3, 1))
        .sum();

    Some(result.to_string())
}

fn minimal_cost(
    machine: &ClawMachine,
    cost_a: i32,
    cost_b: i32,
) -> Option<i32> {
    // 2x2 Matrix (Ax=B)
    let (a11, a21) = machine.button_a;
    let (a12, a22) = machine.button_b;
    let (b1, b2) = machine.price_location;
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

    if (0..=100).contains(&s) && (0..=100).contains(&t) {
        return Some(cost_a * s + cost_b * t);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "480");
    }
}
