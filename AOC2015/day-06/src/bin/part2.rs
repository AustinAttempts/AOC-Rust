// Advent of Code 2015 - Day 06: Probably a Fire Hazard
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
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

fn part2(input: &str) -> u32 {
    let mut light_array = vec![vec![0usize; 1000]; 1000];

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
                    Command::On => light_array[row_index][col_index] += 1, // brighten light +1
                    Command::Off => {
                        // dim light -1
                        light_array[row_index][col_index] =
                            light_array[row_index][col_index].saturating_sub(1)
                    }
                    Command::Toggle => light_array[row_index][col_index] += 2, // turn light brighter +2
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
    fn light_on() {
        let result = part2("turn on 0,0 through 0,0");
        assert_eq!(result, 1);
    }

    #[test]
    fn light_on_bright() {
        let result = part2("toggle 0,0 through 999,999");
        assert_eq!(result, 2000000);
    }
}
