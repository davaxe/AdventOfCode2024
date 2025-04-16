use day_18::{part1, part2};

fn main() {
    // Output results for both parts
    println!(
        "Part 1:\n  {}",
        part1::task(include_str!("../input.txt"), (70, 70), 1024)
            .unwrap_or("No solution found".to_string())
    );
    println!(
        "Part 2:\n  {}",
        part2::task(include_str!("../input.txt"), (70, 70))
            .unwrap_or("No solution found".to_string())
    );
}
