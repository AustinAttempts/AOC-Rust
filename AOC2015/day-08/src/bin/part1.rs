// Advent of Code 2015 - Day 08: Matchsticks
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let mut string_len = 0;
    let mut memory_len = 0;

    for lines in input.lines() {
        string_len += string_literals_len(lines);
        memory_len += string_memory_len(lines);
    }

    string_len - memory_len
}

fn string_literals_len(str: &str) -> usize {
    str.len()
}

fn string_memory_len(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 1; // Skip opening quote
    let mut mem_len = 0;

    while i < chars.len() - 1 {
        if chars[i] == '\\' {
            if i + 1 >= chars.len() - 1 {
                // Trailing backslash
                i += 1;
                mem_len += 1;
            } else {
                match chars[i + 1] {
                    '\\' | '"' => {
                        mem_len += 1;
                        i += 2;
                    }
                    'x' => {
                        if i + 3 < chars.len() {
                            let hex_str = format!("{}{}", chars[i + 2], chars[i + 3]);
                            if u8::from_str_radix(&hex_str, 16).is_ok() {
                                mem_len += 1;
                                i += 4;
                            } else {
                                // malformed escape
                                mem_len += 1;
                                i += 1;
                            }
                        } else {
                            mem_len += 1;
                            i += 1;
                        }
                    }
                    _ => {
                        mem_len += 1;
                        i += 1;
                    }
                }
            }
        } else {
            mem_len += 1;
            i += 1;
        }
    }

    mem_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_len_test() {
        let result = string_literals_len("\"\"");
        assert_eq!(result, 2);

        let result = string_literals_len("\"abc\"");
        assert_eq!(result, 5);

        let result = string_literals_len("\"aaa\\\"aaa\"");
        assert_eq!(result, 10);

        let result = string_literals_len("\"\\x27\"");
        assert_eq!(result, 6);
    }

    #[test]
    fn memory_len_test() {
        let result = string_memory_len("\"\"");
        assert_eq!(result, 0);

        let result = string_memory_len("\"abc\"");
        assert_eq!(result, 3);

        let result = string_memory_len("\"aaa\\\"aaa\"");
        assert_eq!(result, 7);

        let result = string_memory_len("\"\\x27\"");
        assert_eq!(result, 1);
    }

    #[test]
    fn compare_test() {
        let result = part1(
            "\"abc\"
        \"abc\"
        \"aaa\\\"aaa\"
        \"\\x27\"",
        );
        assert_eq!(result, 12);
    }

    #[test]
    fn problem_child() {
        let result = string_memory_len("\"\\\\x27\"");
        assert_eq!(result, 4);
    }

    #[test]
    fn invalid_hex_test() {
        let result = string_memory_len("\"\\xZZ\"");
        // This should probably return 4: \ x Z Z
        assert_eq!(result, 4);
    }

    #[test]
    fn backslash_at_end() {
        let result = string_memory_len("\"abc\\\\\"");
        assert_eq!(result, 4); // Should be: a b c \ 
    }
}
