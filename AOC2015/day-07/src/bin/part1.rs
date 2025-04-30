// Advent of Code 2015 - Day 07: Some Assembly Required
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(_input: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_holder_test() {
        let result = part1(" ");
        assert_eq!(result, 0);
    }
}
