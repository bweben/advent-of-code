use std::collections::{HashMap, HashSet};
use std::str::Lines;
use rayon::prelude::*;
advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone)]
struct Instruction {
    symbol: char,
}

#[derive(Debug, Copy, Clone)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
    symbols: &'a str,
}

impl Instruction {
    fn traverse<'a>(&'a self, graph: &'a Node) -> &str {
        match self.symbol {
            'L' => graph.left,
            'R' => graph.right,
            _ => "ZZZ"
        }
    }
}

fn parse(input: &str) -> (Vec<Instruction>, HashMap<&str, Node>) {
    let mut lines_iter = input.lines();
    let raw_instructions = lines_iter.next();
    let instructions: Vec<Instruction> = parse_instructions(raw_instructions);

    lines_iter.next();
    let graph: HashMap<&str, Node> = parse_graph(&mut lines_iter);
    (instructions, graph)
}

fn parse_instructions(raw_instructions: Option<&str>) -> Vec<Instruction> {
    raw_instructions.unwrap().chars()
        .filter(|raw_instruction| *raw_instruction == 'L' || *raw_instruction == 'R')
        .map(|raw_instruction| Instruction { symbol: raw_instruction })
        .collect()
}

fn parse_graph<'a>(lines_iter: &mut Lines<'a>) -> HashMap<&'a str, Node<'a>> {
    lines_iter
        .map(|line| {
            let mut splitted_line = line.split(" = ");
            let symbols = splitted_line.next().unwrap();
            let next_token = splitted_line.next();
            let mut next = next_token
                .iter()
                .flat_map(|l| l.strip_prefix('('))
                .flat_map(|l| l.strip_suffix(')'))
                .flat_map(|l| l.split(", "));
            let left = next.next().unwrap();
            let right = next.next().unwrap();

            Node {
                left,
                right,
                symbols: symbols,
            }
        })
        .map(|node| (node.symbols.clone(), node))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, graph) = parse(input);
    let steps = calculate_steps(instructions, graph, "AAA");

    Some(steps)
}

fn calculate_steps(instructions: Vec<Instruction>, graph: HashMap<&str, Node>, starting_node: &str) -> u32 {
    let mut steps: u32 = 0;
    let mut node = graph.get(starting_node).unwrap();
    loop {
        let instruction = instructions.get(steps as usize % instructions.len()).unwrap();
        node = graph.get(instruction.traverse(node)).unwrap();
        steps += 1;

        if node.symbols.ends_with('Z') {
            break;
        }
    }
    steps
}

pub fn prime_factorization(mut input: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut prime = 2;

    println!("{}", input);

    while input >= prime * prime {
        if input % prime == 0 {
            result.push(prime);
            input = input / prime;
        } else {
            prime = prime + 1;
        }
    }

    result.push(input);
    result
}

pub fn lcm(input: Vec<u32>) -> u32 {
    let set = input.iter()
        .flat_map(|v| prime_factorization(*v))
        .collect::<HashSet<_>>();

    let result = set
        .iter()
        .product();

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (instructions, graph) = parse(input);
    let starting_nodes: Vec<u32> = graph
        .par_iter()
        .map(|v| v.1)
        .filter(|v| v.symbols.ends_with('A'))
        .map(|v| calculate_steps(instructions.clone(), graph.clone(), v.symbols))
        .collect();

    Some(lcm(starting_nodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
