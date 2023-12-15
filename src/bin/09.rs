use crate::Prediction::{Next, Previous};
advent_of_code::solution!(9);

#[derive(Copy, Clone)]
enum Prediction {
    Previous,
    Next
}

impl Prediction {
    pub fn operation(&self, a: i32, b: i32) -> i32 {
        match self {
            Previous => a - b,
            Next => a + b
        }
    }

    pub fn get(&self, vec: Vec<i32>) -> i32 {
        match self {
            Previous => *vec.first().unwrap_or(&0i32),
            Next => *vec.last().unwrap_or(&0i32)
        }
    }
}

fn predict_element(sequence: Vec<i32>, which: Prediction) -> i32 {
    let mut difference: Vec<_> = Vec::new();
    for window in sequence.windows(2) {
        if let [a, b] = window {
            let diff = *b - *a;
            difference.push(diff);
        };
    }

    if difference.iter().all(|diff| *diff == 0) {
        let element = which.operation(which.get(sequence), which.get(difference));
        return element;
    }

    let result = which.operation(which.get(sequence), predict_element(difference, which));
    result
}

fn solve(input: &str, prediction: Prediction) -> Option<u32> {
    let parsed: Vec<i32> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_ascii_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<_>>())
        .map(|v| predict_element(v, prediction))
        .collect();

    Some(parsed.iter().sum::<i32>() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, Next)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, Previous)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
