# Crate Template

This is a template used for the challenges. It contains the following:

- Two separate modules (`part1.rs` and `part2.rs`) for each part.
- 2 initial (`example.txt` and `input.txt`) text files for input.
- `main.rs` file to display results.

Use [`cargo generate`](https://crates.io/crates/cargo-generate) to crate a
package using this template, e.g.:

```bash
cargo generate --git https://github.com/davaxe/AdventOfCode2024.git --name day-XX
```

or using the path directly (if in the AdventOfCode2024 directory):

```bash
cargo generate --path ./template/ --name day-XX
```
