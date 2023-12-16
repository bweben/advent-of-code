use std::collections::HashSet;
use crate::TileType::{Ground, Horizontal, NorthToEast, NorthToWest, SouthToEast, SouthToWest, Start, Vertical};
advent_of_code::solution!(10);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum TileType {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl TileType {
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
}

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
}

impl Tile {
    pub fn new(c: char, position: (usize, usize)) -> Self {
        Self {
            x: position.0,
            y: position.1,
            tile_type: TileType::new(c),
        }
    }

    pub fn navigate_from_to(&self, from_position: (usize, usize)) -> (usize, usize) {
        let (x_from, y_from) = from_position;

        match self.tile_type {
            Vertical => if y_from > self.y { (self.x, self.y - 1) } else { (self.x, self.y + 1) }
            Horizontal => if x_from > self.x { (self.x - 1, self.y) } else { (self.x + 1, self.y) }
            NorthToEast => if y_from < self.y { (self.x + 1, self.y) } else { (self.x, self.y - 1) }
            NorthToWest => if y_from < self.y { (self.x - 1, self.y) } else { (self.x, self.y - 1) }
            SouthToWest => if y_from > self.y { (self.x - 1, self.y) } else { (self.x, self.y + 1) }
            SouthToEast => if y_from > self.y { (self.x + 1, self.y) } else { (self.x, self.y + 1) }
            Ground => (self.x, self.y),
            Start => (self.x, self.y)
        }
    }

    pub fn is_connected_to(&self, to_position: (usize, usize)) -> bool {
        let (x_to, y_to) = to_position;

        match self.tile_type {
            Vertical => self.x == x_to && (self.y - 1 == y_to || self.y + 1 == y_to),
            Horizontal => (self.x - 1 == x_to || self.x + 1 == x_to) && self.y == y_to,
            NorthToEast => (self.x + 1 == x_to && self.y == y_to) || (self.x == x_to && self.y - 1 == y_to),
            NorthToWest => (self.x - 1 == x_to && self.y == y_to) || (self.x == x_to && self.y - 1 == y_to),
            SouthToWest => (self.x - 1 == x_to && self.y == y_to) || (self.x == x_to && self.y + 1 == y_to),
            SouthToEast => (self.x + 1 == x_to && self.y == y_to) || (self.x == x_to && self.y + 1 == y_to),
            Ground => false,
            Start => true
        }
    }
}

struct Maze {
    data: Vec<Vec<Tile>>,
}

impl Maze {
    pub fn calculate_pipe_path(&self) -> Vec<&Tile> {
        let mut start_tiles: Vec<&Tile> = Vec::new();
        let start_tile = self.data.iter()
            .enumerate()
            .find_map(|y| y.1.iter()
                .enumerate()
                .find_map(|x| {
                    if x.1.tile_type == Start {
                        Some(x.1)
                    } else {
                        None
                    }
                })
            ).unwrap();
        let start_position = (start_tile.x, start_tile.y);

        let mut search_positions = vec![
            (start_position.0 + 1, start_position.1),
            (start_position.0, start_position.1 + 1),
        ];
        if start_position.0 > 0 { search_positions.push((start_position.0 - 1, start_position.1)) }
        if start_position.1 > 0 { search_positions.push((start_position.0, start_position.1 - 1)) }

        for search_position in search_positions {
            if start_tile.is_connected_to(search_position) {
                start_tiles.push(self.get_by(search_position));
            }
        }

        let from_tile = *start_tiles.iter().find(|tile| tile.is_connected_to(start_position)).unwrap();
        let mut from_position = start_position;
        let mut self_tile = from_tile;
        let mut path: Vec<&Tile> = vec![self_tile];

        while self_tile.tile_type != Start {
            let to_position = self_tile.navigate_from_to(from_position);
            from_position = (self_tile.x, self_tile.y);
            self_tile = self.get_by(to_position);
            path.push(self_tile);
        }

        path
    }

    pub fn get_by(&self, pos: (usize, usize)) -> &Tile {
        &self.data[pos.1][pos.0]
    }

    pub fn calculate_enclosed_tiles(&self, path: &Vec<&Tile>) -> HashSet<&Tile> {
        let set = HashSet::new();

        set
    }
}

fn parse(input: &str) -> Maze {
    let data: Vec<Vec<_>> = input.lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(|(x, c)| Tile::new(c, (x, y))).collect())
        .collect();

    let maze = Maze {
        data,
    };

    maze
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = parse(input);

    let path = maze.calculate_pipe_path();
    Some((path.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = parse(input);

    let path = maze.calculate_pipe_path();
    Some(maze.calculate_enclosed_tiles(&path).len() as u32)
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
        assert_eq!(result, Some(10));
    }
}
