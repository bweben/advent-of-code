use std::collections::HashMap;
advent_of_code::solution!(8);

struct Instruction {
    symbol: char,
}

struct Node {
    left: String,
    right: String,
    symbols: String,
}

impl Instruction {
    fn traverse<'a>(&'a self, graph: &'a Node) -> &str {
        match self.symbol {
            'L' => graph.left.as_str(),
            'R' => graph.right.as_str(),
            _ => "ZZZ"
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines_iter = input.lines().into_iter();
    let raw_instructions = lines_iter.next();
    let instructions: Vec<Instruction> = raw_instructions.unwrap().chars()
        .filter(|raw_instruction| *raw_instruction == 'L' || *raw_instruction == 'R')
        .map(|raw_instruction| Instruction { symbol: raw_instruction })
        .collect();

    lines_iter.next();
    let graph: HashMap<String, Node> = lines_iter
        .map(|line| {
            let mut splitted_line = line.split(" = ");
            let symbols = splitted_line.next().unwrap();
            let next_token = splitted_line.next();
            let mut next = next_token
                .iter()
                .flat_map(|l| l.strip_prefix("("))
                .flat_map(|l| l.strip_suffix(")"))
                .flat_map(|l| l.split(", "));
            let left = next.next().unwrap().to_string();
            let right = next.next().unwrap().to_string();

            Node {
                left,
                right,
                symbols: symbols.to_string(),
            }
        })
        .map(|node| (node.symbols.clone(), node))
        .collect();

    let mut steps: u32 = 0;
    let mut node = graph.get("AAA").unwrap();
    loop {
        let instruction = instructions.get(steps as usize % instructions.len()).unwrap();
        node = graph.get(instruction.traverse(node)).unwrap();
        steps += 1;

        if node.symbols == "ZZZ" {
           break;
        }
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
