use std::iter::zip;
advent_of_code::solution!(6);

struct Run {
    time: u64,
    distance: u64,
}

impl Run {
    fn new(zipped: (u64, u64)) -> Run {
        Self {
            time: zipped.0,
            distance: zipped.1,
        }
    }

    fn calculate_ways(&self) -> u32 {
        let mut ways = 0;
        for t in 0..(self.time / 2) {
            if ((self.time - t) * t) <= self.distance {
                ways += 1;
            } else {
                return (self.time - (ways * 2) + 1) as u32
            }
        }

        ways as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines_iterator = input.lines();
    let time_line = lines_iterator.find(|line| line.starts_with("Time:")).unwrap();
    let distance_line = lines_iterator.find(|line| line.starts_with("Distance:")).unwrap();

    let runs: Vec<Run> = zip(
        time_line.strip_prefix("Time:")
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<u64>>(),
        distance_line.strip_prefix("Distance:")
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<u64>>(),
    ).map(Run::new)
        .collect();

    let result = runs.iter()
        .map(Run::calculate_ways)
        .product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines_iterator = input.lines();
    let time_line = lines_iterator.find(|line| line.starts_with("Time:")).unwrap();
    let distance_line = lines_iterator.find(|line| line.starts_with("Distance:")).unwrap();

    let run = Run::new(
        (time_line.strip_prefix("Time:")
             .unwrap()
             .split_ascii_whitespace()
             .collect::<String>()
             .parse::<u64>().unwrap(),
         distance_line.strip_prefix("Distance:")
             .unwrap()
             .split_ascii_whitespace()
             .collect::<String>()
             .parse::<u64>().unwrap())
    );

    let result = run.calculate_ways();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(71503));
    }
}
