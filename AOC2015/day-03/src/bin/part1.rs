// Advent of Code 2015 - Day 03: Perfectly Spherical Houses in a Vacuum

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn part1(input: &str) -> usize {
    let mut current_position = Position { x: 0, y: 0 };
    let mut visited_houses: HashMap<Position, u32> = HashMap::new();

    visited_houses.insert(current_position, 1);

    for elem in input.chars() {
        match elem {
            '^' => current_position.y += 1,
            'v' => current_position.y -= 1,
            '>' => current_position.x += 1,
            '<' => current_position.x -= 1,
            _ => panic!("Unknown character parsed"),
        }

        *visited_houses.entry(current_position).or_insert(0) += 1;
    }

    visited_houses.len()
}

mod tests {
    use super::*;

    #[test]
    fn repeated_houses() {
        let result = part1(">");
        assert_eq!(result, 2);

        let result = part1("^>v<");
        assert_eq!(result, 4);

        let result = part1("^v^v^v^v^v");
        assert_eq!(result, 2);
    }
}
