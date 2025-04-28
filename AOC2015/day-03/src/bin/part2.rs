// Advent of Code 2015 - Day 03: Perfectly Spherical Houses in a Vacuum
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    todo!();
}

mod tests {
    use super::*;

    #[test]
    fn repeated_houses() {
        let result = part2(">");
        assert_eq!(result, 2);

        let result = part2("^>v<");
        assert_eq!(result, 4);

        let result = part2("^v^v^v^v^v");
        assert_eq!(result, 2);
    }
}
