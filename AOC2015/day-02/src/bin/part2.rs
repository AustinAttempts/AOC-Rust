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

//This is a more Rust way to solve this same problem
pub fn part2_rust(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut deminsions: Vec<u32> = line
                .split('x')
                .map(|str_numbers| str_numbers.parse().expect("could not parse this number"))
                .collect();
            deminsions.sort();
            let perimeter = 2 * (deminsions[0] + deminsions[1]);
            let volume = deminsions[0] * deminsions[1] * deminsions[2];
            perimeter + volume
        })
        .sum()
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

    #[test]
    fn ribbon_len_rust_way() {
        let result = part2_rust("2x3x4");
        assert_eq!(result, 34);

        let result = part2_rust("1x1x10");
        assert_eq!(result, 14);

        let result = part2_rust(include_str!("./input1.txt"));
        assert_eq!(result, 3812909);
    }
}
