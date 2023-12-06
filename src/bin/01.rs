advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split('\n')
        .filter(|line| {!line.is_empty()})
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
    let result = input.split('\n')
        .filter(|line| {!line.is_empty()})
        .map(|line| {


            12
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
        assert_eq!(result, None);
    }
}
