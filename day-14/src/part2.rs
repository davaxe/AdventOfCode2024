use crate::parser;

#[must_use]
pub fn task(input: &str, width: i32, height: i32) -> Option<String> {
    let (_, mut robots) = parser::robots(input).ok()?;

    let width_u: usize = usize::try_from(width).ok()?;
    let height_u: usize = usize::try_from(height).ok()?;

    for t in 1..10000 {
        // Move each robot
        robots
            .iter_mut()
            .for_each(|robot| robot.step(width, height));

        // Create an empty image grid
        let mut image = vec![vec![0; width_u]; height_u];

        // Mark robot positions on the image grid
        for robot in &robots {
            let (x, y) = robot.position;
            let x = usize::try_from(x).ok()?;
            let y = usize::try_from(y).ok()?;
            image[y][x] = 1;
        }

        // Find the longest sequence of robots (ones) in a row
        let longest_sequence = image
            .iter()
            .map(|row| {
                row.split(|&x| x == 0).map(<[i32]>::len).max().unwrap_or(0)
            })
            .max()
            .unwrap_or(0);

        if longest_sequence >= 8 {
            return Some(t.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "No test for part 2"]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input, 101, 103);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }
}
