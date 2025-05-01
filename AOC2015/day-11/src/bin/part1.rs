// Advent of Code 2015 - Day 11: Corporate Policy

use itertools::Itertools;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut pswd = input.to_string();
    while !valid_password(&pswd) {
        pswd = inc_password(&pswd);
    }

    pswd.to_string()
}

// increment password rolling over at z
// for example: aaaaa -> aaaab
fn inc_password(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    for i in (0..chars.len()).rev() {
        if chars[i] == 'z' {
            chars[i] = 'a'
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        }
    }

    chars.into_iter().collect()
}

fn valid_password(input: &str) -> bool {
    rule_1(input) && rule_2(input) && rule_3(input)
}

// pass if password has at least 3 consecuitve increasing letters
// for example: abc, bcd, cde, def, ...
fn rule_1(input: &str) -> bool {
    for (a, b, c) in input.chars().tuple_windows() {
        if (b as u8 == a as u8 + 1) && (c as u8 == b as u8 + 1) {
            return true;
        }
    }

    false
}

// fail if password contains i, o, or l
fn rule_2(input: &str) -> bool {
    !(input.contains("i") || input.contains("o") || input.contains("l"))
}

// pass if password has at least 2 different non overlapping pairs
// example: aacdeff -> true
// example: aabcder -> false
// example: aaabcde -> false
fn rule_3(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut pairs_found = 0;

    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            pairs_found += 1;
            i += 2; // skip the next character to avoid overlapping
            if pairs_found == 2 {
                return true;
            }
        } else {
            i += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_1_test() {
        let result = rule_1("hijklmmn");
        assert_eq!(result, true);

        let result = rule_2("hijklmmn");
        assert_eq!(result, false);

        let result = rule_3("hijklmmn");
        assert_eq!(result, false);
    }

    #[test]
    fn rule_2_test() {
        let result = rule_1("abbceffg");
        assert_eq!(result, false);

        let result = rule_2("abbceffg");
        assert_eq!(result, true);

        let result = rule_3("abbceffg");
        assert_eq!(result, true);
    }

    #[test]
    fn rule_3_test() {
        let result = rule_1("abbcegjk");
        assert_eq!(result, false);

        let result = rule_2("abbcegjk");
        assert_eq!(result, true);

        let result = rule_3("abbcegjk");
        assert_eq!(result, false);
    }

    #[test]
    fn next_password() {
        let result = part1("abcdefgh");
        assert_eq!(result, "abcdffaa");

        let result = part1("ghijklmn");
        assert_eq!(result, "ghjaabcc");
    }
}
