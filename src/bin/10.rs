use crate::Tile::{Ground, Horizontal, NorthToEast, NorthToWest, SouthToEast, SouthToWest, Start, Vertical};
advent_of_code::solution!(10);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl Tile {
    pub fn new(c: char) -> Self {
        match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthToEast,
            'J' => NorthToWest,
            '7' => SouthToWest,
            'F' => SouthToEast,
            'S' => Start,
            _ => Ground
        }
    }

    pub fn navigate_from_to(&self, self_position: (usize, usize), from_position: (usize, usize)) -> (usize, usize) {
        let (x_from, y_from) = from_position;
        let (x_self, y_self) = self_position;

        match self {
            Vertical => if y_from > y_self { (x_self, y_self - 1) } else { (x_self, y_self + 1) }
            Horizontal => if x_from > x_self { (x_self - 1, y_self) } else { (x_self + 1, y_self) }
            NorthToEast => if y_from < y_self { (x_self + 1, y_self) } else { (x_self, y_self - 1) }
            NorthToWest => if y_from < y_self { (x_self - 1, y_self) } else { (x_self, y_self - 1) }
            SouthToWest => if y_from > y_self { (x_self - 1, y_self) } else { (x_self, y_self + 1) }
            SouthToEast => if y_from > y_self { (x_self + 1, y_self) } else { (x_self, y_self + 1) }
            Ground => (x_self, y_self),
            Start => (x_self, y_self)
        }
    }

    pub fn is_connected_to(&self, self_position: (usize, usize), to_position: (usize, usize)) -> bool {
        let (x_self, y_self) = self_position;
        let (x_to, y_to) = to_position;

        match self {
            Vertical => x_self == x_to && (y_self - 1 == y_to || y_self + 1 == y_to),
            Horizontal => (x_self - 1 == x_to || x_self + 1 == x_to) && y_self == y_to,
            NorthToEast => (x_self + 1 == x_to && y_self == y_to) || (x_self == x_to && y_self - 1 == y_to),
            NorthToWest => (x_self - 1 == x_to && y_self == y_to) || (x_self == x_to && y_self - 1 == y_to),
            SouthToWest => (x_self - 1 == x_to && y_self == y_to) || (x_self == x_to && y_self + 1 == y_to),
            SouthToEast => (x_self + 1 == x_to && y_self == y_to) || (x_self == x_to && y_self + 1 == y_to),
            Ground => false,
            Start => true
        }
    }
}

struct Maze {
    data: Vec<Vec<Tile>>,
}

impl Maze {
    pub fn max_distance(&self) -> u32 {
        let mut start_tiles: Vec<(usize, usize)> = Vec::new();
        let start_position = self.data.iter()
            .enumerate()
            .find_map(|y| y.1.iter()
                .enumerate()
                .find_map(|x| {
                    if x.1 == &Start {
                        Some((x.0, y.0))
                    } else {
                        None
                    }
                })
            ).unwrap();

        let start_tile = self.get_by(start_position);
        let mut search_positions = vec![
            (start_position.0 + 1, start_position.1),
            (start_position.0, start_position.1 + 1),
        ];
        if start_position.0 > 0 { search_positions.push((start_position.0 - 1, start_position.1)) }
        if start_position.1 > 0 { search_positions.push((start_position.0, start_position.1 - 1)) }

        for search_position in search_positions {
            if start_tile.is_connected_to(start_position, search_position) {
                start_tiles.push(search_position);
            }
        }

        let from_tile = *start_tiles.iter().find(|tile| self.get_by(**tile).is_connected_to(**tile, start_position)).unwrap();
        let mut tile: Option<Tile> = None;
        let mut from_position = start_position;
        let mut self_position = from_tile;
        let mut distance = 1;

        while tile.is_none() || tile.unwrap() != Start {
            let to_position = self.get_by(self_position).navigate_from_to(self_position, from_position);
            tile = Some(self.get_by(to_position));
            // println!("from: {:?}, via: {:?}, to: {:?}, next_tile: {:?}", from_position, self_position, to_position, tile);
            from_position = self_position;
            self_position = to_position;
            distance += 1;
        }

        distance / 2
    }

    pub fn get_by(&self, pos: (usize, usize)) -> Tile {
        self.data[pos.1][pos.0]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars().map(Tile::new).collect())
        .collect();
    let maze = Maze {
        data
    };

    Some(maze.max_distance())
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
