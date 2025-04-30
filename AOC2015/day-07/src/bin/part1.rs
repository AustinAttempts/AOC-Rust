use std::{collections::HashMap, u16};

// Advent of Code 2015 - Day 07: Some Assembly Required
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

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

fn part1(input: &str) -> u16 {
    let mut instuction_map = HashMap::new();

    for line in input.lines() {
        let instr = Instruction::from_line(line);
        instuction_map.insert(instr.out_name(), instr);
    }

    let mut circuit = HashMap::new();
    evaluate("a", &instuction_map, &mut circuit)
}

impl Instruction {
    pub fn from_line(line: &str) -> Self {
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

    pub fn out_name(&self) -> String {
        match self {
            Instruction::And { out, .. } => out.clone(),
            Instruction::Or { out, .. } => out.clone(),
            Instruction::Not { out, .. } => out.clone(),
            Instruction::Lshift { out, .. } => out.clone(),
            Instruction::Rshift { out, .. } => out.clone(),
            Instruction::Assign { out, .. } => out.clone(),
        }
    }
}

fn evaluate(
    wire: &str,
    instructions: &HashMap<String, Instruction>,
    circuit: &mut HashMap<String, u16>,
) -> u16 {
    if let Ok(val) = wire.parse::<u16>() {
        return val;
    }

    if let Some(&val) = circuit.get(wire) {
        return val;
    }

    let inst = instructions
        .get(wire)
        .expect(&format!("No instruction for wire '{}'", wire));
    let value = match inst {
        Instruction::Assign { input, .. } => evaluate(input, instructions, circuit),

        Instruction::And { lhs, rhs, .. } => {
            evaluate(lhs, instructions, circuit) & evaluate(rhs, instructions, circuit)
        }
        Instruction::Or { lhs, rhs, .. } => {
            evaluate(lhs, instructions, circuit) | evaluate(rhs, instructions, circuit)
        }
        Instruction::Not { input, .. } => !evaluate(input, instructions, circuit),

        Instruction::Lshift { input, amount, .. } => {
            evaluate(input, instructions, circuit) << amount
        }
        Instruction::Rshift { input, amount, .. } => {
            evaluate(input, instructions, circuit) >> amount
        }
    };

    circuit.insert(wire.to_string(), value);
    value
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

    #[test]
    fn test_input() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let mut instuction_map = HashMap::new();

        for line in input.lines() {
            let instr = Instruction::from_line(line);
            instuction_map.insert(instr.out_name(), instr);
        }

        let mut circuit = HashMap::new();

        assert_eq!(evaluate("d", &instuction_map, &mut circuit), 72);
        assert_eq!(evaluate("e", &instuction_map, &mut circuit), 507);
        assert_eq!(evaluate("f", &instuction_map, &mut circuit), 492);
        assert_eq!(evaluate("g", &instuction_map, &mut circuit), 114);
        assert_eq!(evaluate("h", &instuction_map, &mut circuit), 65412);
        assert_eq!(evaluate("i", &instuction_map, &mut circuit), 65079);
        assert_eq!(evaluate("x", &instuction_map, &mut circuit), 123);
        assert_eq!(evaluate("y", &instuction_map, &mut circuit), 456);
    }
}
