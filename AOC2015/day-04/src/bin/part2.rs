// Advent of Code 2015 - Day 04: The Ideal Stocking Stuffer
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_houses() {
        let result = part2("");
        assert_eq!(result, 3);
    }
}
