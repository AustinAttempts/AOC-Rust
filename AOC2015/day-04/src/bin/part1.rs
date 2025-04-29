// Advent of Code 2015 - Day 04: The Ideal Stocking Stuffer
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_key() {
        let result = part1("abcdef");
        assert_eq!(result, 609043);

        let result = part1("pqrstuv");
        assert_eq!(result, 1048970);
    }
}
