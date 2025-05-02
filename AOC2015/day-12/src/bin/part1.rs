// Advent of Code 2015 = Day 12: JSAbacusFramework.io
fn main() {
    let input = include_str!("input1.txt");
    let answer = part1(input);

    dbg!(answer);
}

fn part1(_input: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_holder() {
        let result = part1("");
        assert_eq!(result, 0);
    }
}
