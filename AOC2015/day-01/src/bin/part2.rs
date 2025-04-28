// Advent of Code 2015 - Day 01: Not Quite Lisp
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

// Starting from 0 every '(' is +1 floor and
// every ')' is -1 floor.
//
// returns the index of instruction that first put santa
// into the basement.
//
// Note: indexing is base 1 not 0!
pub fn part1(input: &str) -> usize {
    let mut floor: i32 = 0;
    for (i, elem) in input.chars().enumerate() {
        match elem {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => panic!("Unknown character found"),
        }

        if floor < 0 {
            return i + 1;
        }
    }
    panic!("Santa never enters basement")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_basement() {
        let result = part1(")");
        assert_eq!(result, 1);

        let result = part1("()())");
        assert_eq!(result, 5);
    }
}
