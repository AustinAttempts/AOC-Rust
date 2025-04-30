// Advent of Code 2015 - Day 06: Probably a Fire Hazard
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

enum Command {
    On,
    Off,
    Toggle,
}

struct Point {
    x: usize,
    y: usize,
}

fn part1(input: &str) -> u32 {
    let mut light_array = vec![vec![0u8; 1000]; 1000];

    for line in input.lines() {
        let instruction: Command;

        if line.contains("turn on") {
            // handle turning on lights
            instruction = Command::On;
        } else if line.contains("turn off") {
            // handle turning off lights
            instruction = Command::Off;
        } else if line.contains("toggle") {
            // handle toggling lights
            instruction = Command::Toggle;
        } else {
            panic!("could not decipher the command")
        }

        let mut coordinates: Vec<usize> = Vec::new();

        // get vector of coordinates
        for tokens in line.split_whitespace().filter(|s| s.contains(',')) {
            let numbers: Vec<usize> = tokens
                .replace(",", " ")
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            coordinates.extend(numbers);
        }

        // transfer coordinates to points
        let start_point = Point {
            x: coordinates[0],
            y: coordinates[1],
        };
        let end_point = Point {
            x: coordinates[2],
            y: coordinates[3],
        };

        // iterate over size of command
        for row_index in start_point.y..=end_point.y {
            for col_index in start_point.x..=end_point.x {
                match instruction {
                    Command::On => light_array[row_index][col_index] = 1, // turn light on
                    Command::Off => light_array[row_index][col_index] = 0, // turn light off
                    Command::Toggle => light_array[row_index][col_index] ^= 1, // toggle light on/off
                }
            }
        }
    }

    // count how many lights are on
    let mut lights_on: u32 = 0;
    for row in light_array {
        for elem in row {
            lights_on += elem as u32;
        }
    }
    lights_on
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
