use crate::parser;

#[must_use]
pub fn task(input: &str, width: i32, height: i32) -> Option<String> {
    let (_, mut robots) = parser::robots(input).ok()?;

    for _ in 0..100 {
        robots
            .iter_mut()
            .for_each(|robot| robot.step(width, height));
    }

    let mut quadrants = [0; 4];

    for robot in robots {
        let (x, y) = robot.position;
        if let Some(quadrant) = Quadrant::from_position(x, y, width, height) {
            quadrants[quadrant as usize] += 1;
        }
    }

    let result: i32 = quadrants.iter().product();

    Some(result.to_string())
}

enum Quadrant {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}

impl Quadrant {
    fn from_position(x: i32, y: i32, width: i32, height: i32) -> Option<Self> {
        let hw = width / 2;
        let hh = height / 2;
        match (x, y) {
            (x, y) if x < hw && y < hh => Some(Quadrant::TopLeft),
            (x, y) if x > hw && y < hh => Some(Quadrant::TopRight),
            (x, y) if x < hw && y > hh => Some(Quadrant::BottomLeft),
            (x, y) if x > hw && y > hh => Some(Quadrant::BottomRight),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input, 11, 7);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "12");
    }
}
