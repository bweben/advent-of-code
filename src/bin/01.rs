advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split('\n')
        .filter(|line| { !line.is_empty() })
        .map(|line| {
            let chars = line.chars();
            let first_digit = chars.clone().find(|c| { c.is_ascii_digit() })
                .expect(&*format!("no digit found: {line}"))
                .to_digit(10)
                .expect("digit was no digit") * 10;
            let last_digit = chars.clone().rfind(|c| { c.is_ascii_digit() })
                .expect(&*format!("no digit found: {line}"))
                .to_digit(10)
                .expect("digit was no digit");

            first_digit + last_digit
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let searchable = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let result = input.split('\n')
        .filter(|line| { !line.is_empty() })
        .map(|line| {
            let mut min_position = line.len();
            let mut min_result = 0u32;
            let mut max_position = 0;
            let mut max_result = 0u32;

            for (i, x) in (0u32..).zip(searchable.iter()) {
                let min = line.find(x).unwrap_or(usize::MAX);
                let max = line.rfind(x).unwrap_or(usize::MAX);

                if min <= min_position {
                    min_position = min;
                    min_result = (i % 9) + 1;
                }

                if max >= max_position && max != usize::MAX {
                    max_position = max;
                    max_result = (i % 9) + 1;
                }
            }

            (min_result * 10) + max_result
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(407));
    }
}
