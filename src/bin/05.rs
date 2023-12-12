use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(Debug, Copy, Clone)]
struct Range {
    dst_start: u32,
    src_start: u32,
    range_length: u32,
}

#[derive(Debug, Copy, Clone)]
struct SeedRange {
    start: u32,
    lenght: u32,
}

impl Range {
    fn is_in(&self, value: u32) -> bool {
        self.src_start <= value && self.src_start + self.range_length >= value
    }

    fn convert(&self, value: u32) -> u32 {
        self.dst_start + (value - self.src_start)
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,

    ranges: Vec<Range>,
}

impl Map {
    pub fn new(from: String, to: String, ranges: &Vec<Range>) -> Self {
        Self { from, to, ranges: ranges.clone() }
    }

    fn convert(&self, value: u32) -> u32 {
        let result = self.ranges.iter()
            .find(|range| range.is_in(value))
            .map(|range| range.convert(value))
            .unwrap_or(value);

        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let filtered_lines: Vec<&str> = input.lines()
        .filter(|line| !line.is_empty())
        .collect();

    let (seeds, maps) = parse_map(filtered_lines);
    let mut converted_values: Vec<u32> = Vec::new();
    for seed in seeds {
        let mut map = maps.get("seed")
            .unwrap();

        let mut converted_value = map.convert(seed);
        while map.to != "location" {
            map = maps.get(&map.to).unwrap();
            converted_value = map.convert(converted_value);
        }

        converted_values.push(converted_value);
    }

    Some(*converted_values.iter().min().unwrap())
}

fn parse_map(filtered_lines: Vec<&str>) -> (Vec<u32>, HashMap<String, Map>) {
    let mut seeds: Vec<u32> = Vec::new();
    let mut maps: HashMap<String, Map> = HashMap::new();
    let mut map_ongoing = false;
    let mut from: &str = "";
    let mut to: &str = "";
    let mut ranges: Vec<Range> = Vec::new();
    for line in filtered_lines {
        if line.starts_with("seeds:") {
            if map_ongoing {
                maps.insert(from.to_string(), Map::new(from.to_string(), to.to_string(), &ranges));
                map_ongoing = false;
            }

            let stripped_line = line.strip_prefix("seeds:").unwrap();
            seeds.append(
                &mut stripped_line.split_ascii_whitespace()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            );

            continue;
        }

        if line.ends_with("map:") {
            if map_ongoing {
                maps.insert(from.to_string(), Map::new(from.to_string(), to.to_string(), &ranges));
            }

            let stripped_line = line.strip_suffix(" map:").unwrap();
            let mut split = stripped_line.split("-to-");
            from = split.next().unwrap().trim();
            to = split.next().unwrap().trim();
            ranges = Vec::new();
            map_ongoing = true;

            continue;
        }

        let mut split = line.split_ascii_whitespace();
        let dst = split.next().unwrap().trim().parse::<u32>().unwrap();
        let src = split.next().unwrap().trim().parse::<u32>().unwrap();
        let length = split.next().unwrap().trim().parse::<u32>().unwrap();
        ranges.push(Range { dst_start: dst, src_start: src, range_length: length });
    }

    if map_ongoing {
        maps.insert(from.to_string(), Map::new(from.to_string(), to.to_string(), &ranges));
    }

    (seeds, maps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let filtered_lines: Vec<&str> = input.lines()
        .filter(|line| !line.is_empty())
        .collect();

    let (seeds, maps) = parse_map(filtered_lines);
    let seed_with_range = seeds.chunks(2);
    let mut actual_seeds: Vec<u32> = Vec::new();
    for chunk in seed_with_range {
        let start = *chunk.get(0).unwrap();
        let length = *chunk.get(1).unwrap();

        lengths += length;

        for i in start..(start + length) {
            actual_seeds.push(i);
        }
    }

    let mut converted_values: Vec<u32> = actual_seeds;
    let mut map = maps.get("seed").unwrap();
    while true {
        let mut temp_values: Vec<u32> = Vec::new();
        for (i, value) in converted_values.iter().enumerate() {
            temp_values.push(map.convert(*value));
        }

        converted_values = temp_values;

        if map.to == "location" {
            break;
        }
        map = maps.get(&map.to).unwrap()
    }

    Some(*converted_values.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(46));
    }
}
