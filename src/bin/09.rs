advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let parsed: Vec<i32> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_ascii_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<_>>())
        .map(predict_next_element)
        .collect();

    Some(parsed.iter().sum::<i32>() as u32)
}

fn predict_next_element(sequence: Vec<i32>) -> i32 {
    let mut difference: Vec<_> = Vec::new();
    for window in sequence.windows(2) {
        if let [a, b] = window {
            let diff = *b - *a;
            difference.push(diff);
        };
    }

    if difference.iter().all(|diff| *diff == 0) {
        return sequence.last().unwrap() + difference.last().unwrap_or(&0i32);
    }

    sequence.last().unwrap() + predict_next_element(difference)
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
