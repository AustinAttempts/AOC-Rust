// Advent of Code 2015 - Day 10: Elves Look, Elves Say
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut encoded_string = input.to_string();

    for _ in 0..50 {
        encoded_string = encode(&encoded_string);
    }

    encoded_string.len()
}

fn encode(input: &str) -> String {
    let mut encoded_str = String::new();
    let mut current_char = input.chars().next().expect("no char found");
    let mut char_count = 0;

    for c in input.chars() {
        if c == current_char {
            char_count += 1;
        } else {
            encoded_str.push_str(&char_count.to_string());
            encoded_str.push_str(&current_char.to_string());

            current_char = c;
            char_count = 1;
        }
    }

    encoded_str.push_str(&char_count.to_string());
    encoded_str.push_str(&current_char.to_string());

    encoded_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_1() {
        let result = encode("1");
        assert_eq!(result, "11");
    }

    #[test]
    fn step_2() {
        let result = encode("11");
        assert_eq!(result, "21");
    }

    #[test]
    fn step_3() {
        let result = encode("1211");
        assert_eq!(result, "111221");
    }

    #[test]
    fn step_4() {
        let result = encode("111221");
        assert_eq!(result, "312211");
    }

    #[test]
    fn iterations() {
        let mut test_input = "1".to_string();
        for _ in 0..=4 {
            test_input = encode(&test_input);
        }
        assert_eq!(test_input, "312211");
    }
}
