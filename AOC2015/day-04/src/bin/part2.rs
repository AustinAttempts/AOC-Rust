use md5::Digest;

// Advent of Code 2015 - Day 04: The Ideal Stocking Stuffer
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut counter: u32 = 0;
    let mut hash = compute_hash(input, counter);
    while !good_hash(hash) {
        counter += 1;
        hash = compute_hash(input, counter);
    }

    counter
}

fn good_hash(hash: Digest) -> bool {
    // first 5 nibbles (4 bits) are 0
    hash[0] == 0 && hash[1] == 0 && hash[2] == 0
}

fn compute_hash(input: &str, counter: u32) -> Digest {
    let value = input.to_owned() + &counter.to_string();
    md5::compute(value.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_key_low() {
        let result = part2("abcdef");
        assert_ne!(result, 609043);
    }

    #[test]
    fn secret_key_high() {
        let result = part2("pqrstuv");
        assert_ne!(result, 1048970);
    }
}
