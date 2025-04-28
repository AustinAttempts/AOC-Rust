// Advent of Code 2015 - Day 02: I Was Told There Would Be No Math
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

// calculate the total amount of wrapping paper needed:
// per present = 2(lw + wh + hl) + min(lwh)
pub fn part1(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_paper_size() {
        let result = part1("2x3x4");
        assert_eq!(result, 58);

        let result = part1("1x1x10");
        assert_eq!(result, 43);
    }
}
