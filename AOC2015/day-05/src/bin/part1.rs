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
