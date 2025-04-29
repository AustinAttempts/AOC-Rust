// Advent of Code 2015 - Day 05: Doesn't He Have Intern-Elves For This?
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

// return the number of string that follow these rules
// 1. conatins at least 3 vowels (a, e, i, o, u)
// 2. contains at least 1 occurance of a repeated letter (i.e. cc)
// 3. conatins non of these sequences ("ab", "cd", "pq", "xy")
fn part1(input: &str) -> usize {
    let mut nice_strings: Vec<String> = Vec::new();
    let mut naughty_strings: Vec<String> = Vec::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for str in input.lines() {
        if str.contains("ab") || str.contains("cd") || str.contains("pq") || str.contains("xy") {
            naughty_strings.push(str.to_string());
        } else if str
            .chars()
            .filter(|c| c.to_lowercase().any(|lc| vowels.contains(&lc)))
            .count()
            < 3
        {
            naughty_strings.push(str.to_string());
        } else if str
            .chars()
            .zip(str.chars().skip(1))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_string_1() {
        let result = part1("ugknbfddgicrmopn");
        assert_eq!(result, 1);
    }

    #[test]
    fn nice_string_2() {
        let result = part1("aaa");
        assert_eq!(result, 1);
    }

    #[test]
    fn naughty_dbl_letter() {
        let result = part1("jchzalrnumimnmhp");
        assert_eq!(result, 0);
    }

    #[test]
    fn naughty_contains_xy() {
        let result = part1("haegwjzuvuyypxyu");
        assert_eq!(result, 0);
    }

    #[test]
    fn naughty_one_vowel() {
        let result = part1("dvszwmarrgswjxmb");
        assert_eq!(result, 0);
    }
}
