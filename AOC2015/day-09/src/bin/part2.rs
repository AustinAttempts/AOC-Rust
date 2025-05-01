// Advent of Code 2015 - Day 09: All in a Single Night

use itertools::Itertools;
use std::{collections::HashMap, i32};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut map = WeightedGraph::new();

    for (i, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 5 || parts[1] != "to" || parts[3] != "=" {
            panic!(
                "Line {}: expected format 'A to B = x', got {:?}",
                i + 1,
                parts
            );
        }

        let dest_a = parts[0];
        let dest_b = parts[2];
        let distance: i32 = parts[4]
            .parse()
            .unwrap_or_else(|_| panic!("Line {}: '{}' is not a valid number", i + 1, parts[4]));

        map.add_edge(dest_a.to_string(), dest_b.to_string(), distance);
        map.add_edge(dest_b.to_string(), dest_a.to_string(), distance);
    }

    map.print_graph();

    dbg!(longest_path(&map).expect("could not find a distance"))
}

fn longest_path(graph: &WeightedGraph) -> Option<i32> {
    let cities: Vec<&String> = graph.adj_list.keys().collect();
    let mut max_distance = i32::MIN;

    for path in cities.iter().permutations(cities.len()).unique() {
        let mut total = 0;
        let mut valid = true;

        for pair in path.windows(2) {
            let from = pair[0];
            let to = pair[1];

            if let Some(neighbors) = graph.neighbors(from) {
                if let Some((_, dist)) = neighbors.iter().find(|(n, _)| n == *to) {
                    total += *dist;
                } else {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            max_distance = max_distance.max(total);
        }
    }

    if max_distance > i32::MIN {
        Some(max_distance)
    } else {
        None
    }
}

struct WeightedGraph {
    adj_list: HashMap<String, Vec<(String, i32)>>,
}

impl WeightedGraph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: String, to: String, weight: i32) {
        self.adj_list.entry(from).or_default().push((to, weight));
    }

    fn neighbors(&self, node: &String) -> Option<&Vec<(String, i32)>> {
        self.adj_list.get(node)
    }

    fn print_graph(&self) {
        for (node, edges) in &self.adj_list {
            for (neighbor, weight) in edges {
                println!("{node} -> {neighbor} [weight: {weight}]");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_distance() {
        let result = part2(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        );
        assert_eq!(result, 982);
    }
}
