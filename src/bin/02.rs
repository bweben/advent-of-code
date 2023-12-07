advent_of_code::solution!(2);

enum Color {
    GREEN,
    RED,
    BLUE,
}

struct Play {
    color: Color,
    amount: u32,
}

struct Game {
    id: u32,
    plays: Vec<Vec<Play>>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (red_cubes, green_cubes, blue_cubes) = (Play { color: Color::RED, amount: 12 }, Play { color: Color::GREEN, amount: 13 }, Play { color: Color::BLUE, amount: 14 });

    let result = input.lines()
        .filter(|line| !line.is_empty())
        .map(parse_game)
        .filter(|game| {
            !game.plays.iter().any(|play| {
                play.iter().any(|color_play| {
                    match color_play.color {
                        Color::GREEN => {
                            green_cubes.amount < color_play.amount
                        }
                        Color::RED => {
                            red_cubes.amount < color_play.amount
                        }
                        Color::BLUE => {
                            blue_cubes.amount < color_play.amount
                        }
                    }
                })
            })
        })
        .map(|game| game.id)
        .sum();

    Some(result)
}

fn parse_game(line: &str) -> Game {
    let [game, plays] = (line.split(':').collect::<Vec<&str>>())[0..2] else {
        panic!("split did not work, invalid format on line {line}")
    };

    let id = game.strip_prefix("Game ")
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);

    let parsed_plays = plays.split(';')
        .map(|play| {
            play.split(',')
                .map(parse_play)
                .collect::<Vec<Play>>()
        })
        .collect::<Vec<Vec<Play>>>();

    Game { id: id, plays: parsed_plays }
}

fn parse_play(color_play: &str) -> Play {
    let [amount, color] = (color_play.trim().split(' ').collect::<Vec<&str>>())[0..2] else {
        panic!("color play did not match see {color_play}");
    };

    let converted_amount = amount.parse::<u32>().unwrap_or(0);
    match color {
        "blue" => Play { color: Color::BLUE, amount: converted_amount },
        "red" => Play { color: Color::RED, amount: converted_amount },
        "green" => Play { color: Color::GREEN, amount: converted_amount },
        _ => panic!("color play did not match color see {color_play}")
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines()
        .filter(|line| !line.is_empty())
        .map(parse_game)
        .map(|game| {
            let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);

            for play in game.plays.iter().flatten() {
                match play.color {
                    Color::GREEN => {
                        if play.amount > min_green { min_green = play.amount }
                    }
                    Color::RED => {
                        if play.amount > min_red { min_red = play.amount }
                    }
                    Color::BLUE => {
                        if play.amount > min_blue { min_blue = play.amount }
                    }
                }
            }

            min_red * min_blue * min_green
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2286));
    }
}
