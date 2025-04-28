// Advent of Code 2015 - Day 02: I Was Told There Would Be No Math
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

// calculate the total amount of wrapping paper needed:
// per present = 2(lw + wh + hl) + min(side)
pub fn part1(input: &str) -> u32 {
    let mut wrapping_paper_ft = 0;

    for lines in input.lines() {
        let mut box_deminsions: Vec<&str> = lines.split('x').collect();
        let length: u32 = box_deminsions
            .pop()
            .expect("No value found")
            .parse()
            .expect("could not parse");
        let width: u32 = box_deminsions
            .pop()
            .expect("No value found")
            .parse()
            .expect("could not parse");
        let height: u32 = box_deminsions
            .pop()
            .expect("No value found")
            .parse()
            .expect("could not parse");

        // calculate surface area
        let side1 = length * width;
        let side2 = width * height;
        let side3 = height * length;
        wrapping_paper_ft += 2 * (side1 + side2 + side3);
        // calulate extra
        wrapping_paper_ft += side1.min(side2).min(side3);
    }

    wrapping_paper_ft
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
