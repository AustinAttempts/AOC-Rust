// Advent of Code 2015 - Day 06: Probably a Fire Hazard
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(_input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let result = part2("");
        assert_eq!(result, 0);
    }
}
