// Advent of Code 2015 - Day 02: I Was Told There Would Be No Math
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(input: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check2() {
        let result = part2("");
        assert_eq!(result, 0);

        let result = part2("");
        assert_eq!(result, 0);
    }
}
