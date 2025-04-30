// Advent of Code 2015 - Day 07: Some Assembly Required
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum Instruction {
    And {
        lhs: String,
        rhs: String,
        out: String,
    },
    Or {
        lhs: String,
        rhs: String,
        out: String,
    },
    Not {
        input: String,
        out: String,
    },
    Lshift {
        input: String,
        amount: u32,
        out: String,
    },
    Rshift {
        input: String,
        amount: u32,
        out: String,
    },
    Assign {
        input: String,
        out: String,
    },
}

fn part1(input: &str) -> i32 {
    for line in input.lines() {
        let instr = Instruction::from_line(line);
    }
    todo!();
}

impl Instruction {
    pub fn from_line(line: &str) -> Self {
        if let Some(rest) = line.strip_suffix("\n") {
            return Instruction::from_line(rest);
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        if parts.len() != 2 {
            panic!("Invalid instruction format: {line}");
        }

        let out = parts[1].to_string();
        let expr = parts[0];

        if expr.contains("AND") {
            let tokens: Vec<&str> = expr.split(" AND ").collect();
            Instruction::And {
                lhs: tokens[0].to_string(),
                rhs: tokens[1].to_string(),
                out,
            }
        } else if expr.contains("OR") {
            let tokens: Vec<&str> = expr.split(" OR ").collect();
            Instruction::Or {
                lhs: tokens[0].to_string(),
                rhs: tokens[1].to_string(),
                out,
            }
        } else if expr.contains("NOT") {
            let token = expr.trim_start_matches("NOT ").to_string();
            Instruction::Not { input: token, out }
        } else if expr.contains("LSHIFT") {
            let tokens: Vec<&str> = expr.split(" LSHIFT ").collect();
            Instruction::Lshift {
                input: tokens[0].to_string(),
                amount: tokens[1].parse().expect("Invalid shift amount"),
                out,
            }
        } else if expr.contains("RSHIFT") {
            let tokens: Vec<&str> = expr.split(" RSHIFT ").collect();
            Instruction::Rshift {
                input: tokens[0].to_string(),
                amount: tokens[1].parse().expect("Invalid shift amount"),
                out,
            }
        } else {
            Instruction::Assign {
                input: expr.to_string(),
                out,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_assign() {
        let result = Instruction::from_line("123 -> x");
        match result {
            Instruction::Assign { ref input, ref out } => {
                assert_eq!(input, "123");
                assert_eq!(out, "x");
            }
            _ => panic!("Expected an Assign instruction"),
        }

        let result = Instruction::from_line("456 -> y");
        match result {
            Instruction::Assign { ref input, ref out } => {
                assert_eq!(input, "456");
                assert_eq!(out, "y");
            }
            _ => panic!("Expected a -> instruction"),
        }
    }

    #[test]
    fn verify_and() {
        let result = Instruction::from_line("x AND y -> d");
        match result {
            Instruction::And {
                ref lhs,
                ref rhs,
                ref out,
            } => {
                assert_eq!(lhs, "x");
                assert_eq!(rhs, "y");
                assert_eq!(out, "d");
            }
            _ => panic!("Expected an AND instruction"),
        }
    }

    #[test]
    fn verify_or() {
        let result = Instruction::from_line("x OR y -> e");
        match result {
            Instruction::Or {
                ref lhs,
                ref rhs,
                ref out,
            } => {
                assert_eq!(lhs, "x");
                assert_eq!(rhs, "y");
                assert_eq!(out, "e");
            }
            _ => panic!("Expected an OR instruction"),
        }
    }

    #[test]
    fn verify_lshift() {
        let result = Instruction::from_line("x LSHIFT 2 -> f");
        match result {
            Instruction::Lshift {
                ref input,
                amount,
                ref out,
            } => {
                assert_eq!(input, "x");
                assert_eq!(amount, 2);
                assert_eq!(out, "f");
            }
            _ => panic!("Expected a LSHIFT instruction"),
        }
    }

    #[test]
    fn verify_rshift() {
        let result = Instruction::from_line("y RSHIFT 2 -> g");
        match result {
            Instruction::Rshift {
                ref input,
                amount,
                ref out,
            } => {
                assert_eq!(input, "y");
                assert_eq!(amount, 2);
                assert_eq!(out, "g");
            }
            _ => panic!("Expected a RSHIFT instruction"),
        }
    }

    #[test]
    fn verify_not() {
        let result = Instruction::from_line("NOT x -> h");
        match result {
            Instruction::Not { ref input, ref out } => {
                assert_eq!(input, "x");
                assert_eq!(out, "h");
            }
            _ => panic!("Expected a NOT instruction"),
        }

        let result = Instruction::from_line("NOT y -> i");
        match result {
            Instruction::Not { ref input, ref out } => {
                assert_eq!(input, "y");
                assert_eq!(out, "i");
            }
            _ => panic!("Expected a NOT instruction"),
        }
    }
}
