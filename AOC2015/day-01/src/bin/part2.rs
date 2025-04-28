// Advent of Code 2015 - Day 01: Not Quite Lisp
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

// Starting from 0 every '(' is +1 floor and
// every ')' is -1 floor.
// returns the final value
pub fn part1(input: &str) -> i32 {
    let mut floor = 0;
    for elem in input.chars() {
        match elem {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => panic!("Unknown character found"),
        }
    }
    floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_0() {
        let result = part1("(())");
        assert_eq!(result, 0);

        let result = part1("()()");
        assert_eq!(result, 0);
    }

    #[test]
    fn floor_3() {
        let result = part1("(((");
        assert_eq!(result, 3);

        let result = part1("(()(()(");
        assert_eq!(result, 3);

        let result = part1("))(((((");
        assert_eq!(result, 3);
    }

    #[test]
    fn basement_1() {
        let result = part1("())");
        assert_eq!(result, -1);

        let result = part1("))(");
        assert_eq!(result, -1);
    }

    #[test]
    fn basement_3() {
        let result = part1(")))");
        assert_eq!(result, -3);

        let result = part1(")())())");
        assert_eq!(result, -3);
    }
}
