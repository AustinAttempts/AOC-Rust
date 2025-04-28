// Advent of Code 2015 - Day 02: I Was Told There Would Be No Math
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

// calculate the amount of ribbon necessary for each present
// ribbon = min_perimeter + lwh
pub fn part2(input: &str) -> u32 {
    let mut ribbon_len = 0;

    for lines in input.lines() {
        let box_deminsions: Vec<&str> = lines.split('x').collect();
        let mut deminsions: Vec<u32> = Vec::new();
        for elem in box_deminsions {
            deminsions.push(elem.parse().expect("could not parse"));
        }
        deminsions.sort();

        ribbon_len += (2 * deminsions[0]) + (2 * deminsions[1]);
        ribbon_len += deminsions[0] * deminsions[1] * deminsions[2];
    }

    ribbon_len
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
