// Advent of Code 2015 - Day 02: I Was Told There Would Be No Math
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

// calculate the amount of ribbon necessary for each present
// ribbon = min_perimeter + lwh
pub fn part2(input: &str) -> i32 {
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
    fn ribbon_len() {
        let result = part2("2x3x4");
        assert_eq!(result, 34);

        let result = part2("1x1x10");
        assert_eq!(result, 14);
    }
}
