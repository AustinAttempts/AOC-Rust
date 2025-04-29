// Advent of Code 2015 - Day 03: Perfectly Spherical Houses in a Vacuum

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn part2(input: &str) -> usize {
    let mut santa_current_position = Position { x: 0, y: 0 };
    let mut robot_current_position = Position { x: 0, y: 0 };

    let mut visited_houses: HashMap<Position, u32> = HashMap::new();

    *visited_houses.entry(santa_current_position).or_insert(0) += 1;
    *visited_houses.entry(robot_current_position).or_insert(0) += 1;

    for (i, dir) in input.chars().enumerate() {
        if i % 2 == 0 {
            match dir {
                '^' => santa_current_position.y += 1,
                'v' => santa_current_position.y -= 1,
                '>' => santa_current_position.x += 1,
                '<' => santa_current_position.x -= 1,
                _ => panic!("Unknown character parsed"),
            }
            *visited_houses.entry(santa_current_position).or_insert(0) += 1;
        } else {
            match dir {
                '^' => robot_current_position.y += 1,
                'v' => robot_current_position.y -= 1,
                '>' => robot_current_position.x += 1,
                '<' => robot_current_position.x -= 1,
                _ => panic!("Unknown character parsed"),
            }
            *visited_houses.entry(robot_current_position).or_insert(0) += 1;
        }
    }

    visited_houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_houses() {
        let result = part2("^v");
        assert_eq!(result, 3);

        let result = part2("^>v<");
        assert_eq!(result, 3);

        let result = part2("^v^v^v^v^v");
        assert_eq!(result, 11);
    }
}
