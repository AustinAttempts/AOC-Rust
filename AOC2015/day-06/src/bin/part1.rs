// Advent of Code 2015 - Day 06: Doesn't He Have Intern-Elves For This?
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(_input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_lights_on() {
        let result = part1("turn on 0,0 through 999,999");
        assert_eq!(result, 1000000);
    }

    #[test]
    fn toggle_lights_on() {
        let result = part1("toggle 0,0 through 999,0");
        assert_eq!(result, 1000);
    }

    #[test]
    fn turn_lights_off() {
        let result = part1("turn off 499,499 through 500,500");
        assert_eq!(result, 0);
    }
}
