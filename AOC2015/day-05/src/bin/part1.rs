// Advent of Code 2015 - Day 05: Doesn't He Have Intern-Elves For This?
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1("");
        assert_ne!(result, 0);
    }
}
