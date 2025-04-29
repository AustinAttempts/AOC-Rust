// Advent of Code 2015 - Day 05: Doesn't He Have Intern-Elves For This?
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result = part2("");
        assert_ne!(result, 0);
    }
}
