// Advent of Code 2015 = Day 12: JSAbacusFramework.io
fn main() {
    let input = include_str!("input1.txt");
    let answer = part1(input);

    dbg!(answer);
}

fn part1(input: &str) -> i32 {
    let cleaned = input.replace(['[', ']', '{', '}', ':', ',', '"'], " ");
    cleaned
        .split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        let result = part1("[1,2,3]");
        assert_eq!(result, 6);

        let result = part1("{\"a\":2,\"b\":4}");
        assert_eq!(result, 6);

        let result = part1("[[[3]]]");
        assert_eq!(result, 3);

        let result = part1("{\"a\":{\"b\":4},\"c\":-1}");
        assert_eq!(result, 3);

        let result = part1("{\"a\":[-1,1]}");
        assert_eq!(result, 0);

        let result = part1("[-1,{\"a\":1}]");
        assert_eq!(result, 0);

        let result = part1("[]");
        assert_eq!(result, 0);

        let result = part1("{}");
        assert_eq!(result, 0);
    }
}
