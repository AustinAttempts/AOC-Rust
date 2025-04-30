// Advent of Code 2015 - Day 08: Matchsticks
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut original_string_len = 0;
    let mut new_string_len = 0;

    for lines in input.lines() {
        original_string_len += string_literals_len(lines);
        new_string_len += string_encode(lines);
    }

    new_string_len - original_string_len
}

fn string_literals_len(str: &str) -> usize {
    str.len()
}

fn string_encode(line: &str) -> usize {
    let result = format!("{:?}", line);
    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_len_test() {
        let result = string_literals_len("\"\"");
        assert_eq!(result, 2);

        let result = string_literals_len("\"abc\"");
        assert_eq!(result, 5);

        let result = string_literals_len("\"aaa\\\"aaa\"");
        assert_eq!(result, 10);

        let result = string_literals_len("\"\\x27\"");
        assert_eq!(result, 6);
    }

    #[test]
    fn memory_len_test() {
        let result = string_encode("\"\"");
        assert_eq!(result, 6);

        let result = string_encode("\"abc\"");
        assert_eq!(result, 9);

        let result = string_encode("\"aaa\\\"aaa\"");
        assert_eq!(result, 16);

        let result = string_encode("\"\\x27\"");
        assert_eq!(result, 11);
    }

    #[test]
    fn compare_test() {
        let result = part2(
            "\"abc\"
        \"abc\"
        \"aaa\\\"aaa\"
        \"\\x27\"",
        );
        assert_eq!(result, 19);
    }
}
