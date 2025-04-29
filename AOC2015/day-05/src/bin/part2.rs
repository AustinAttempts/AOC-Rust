use std::collections::HashMap;

// Advent of Code 2015 - Day 05: Doesn't He Have Intern-Elves For This?
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

// return the number of string that follow these rules
// 1. contains at least 2 occurance of a repeated letter (i.e. ccdd)
// 2. conatins a repeated character with a gap between it (i.e. axa)
fn part2(input: &str) -> usize {
    let mut nice_strings: Vec<String> = Vec::new();
    let mut naughty_strings: Vec<String> = Vec::new();

    for str in input.lines() {
        if !repeated_chars(str) {
            naughty_strings.push(str.to_string());
        } else if str
            .chars()
            .zip(str.chars().skip(2))
            .filter_map(|(a, b)| if a == b { Some(a) } else { None })
            .count()
            == 0
        {
            naughty_strings.push(str.to_string());
        } else {
            nice_strings.push(str.to_string());
        }
    }

    nice_strings.len()
}

// checks if the string has at least 2 repeated characters
// and that those repeats do not overlap
fn repeated_chars(line: &str) -> bool {
    let mut pairs = HashMap::new();

    for i in 0..line.as_bytes().len().saturating_sub(1) {
        let pair = &line.as_bytes()[i..=i + 1];
        if let Some(&prev_i) = pairs.get(pair) {
            if i > prev_i + 1 {
                return true;
            }
        } else {
            pairs.insert(pair, i);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_string_1() {
        let result = part2("qjhvhtzxzqqjkmpb");
        assert_eq!(result, 1);
    }

    #[test]
    fn nice_string_2() {
        let result = part2("xxyxx");
        assert_eq!(result, 1);
    }

    #[test]
    fn naughty_mid_repeat() {
        let result = part2("uurcxstgmygtbstg");
        assert_eq!(result, 0);
    }

    #[test]
    fn naughty_single_repeat() {
        let result = part2("ieodomkazucvgmuy");
        assert_eq!(result, 0);
    }
}
