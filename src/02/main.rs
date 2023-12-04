use std::fs;

const TOTAL_COUNT_RED: u8 = 12;
const TOTAL_COUNT_GREEN: u8 = 13;
const TOTAL_COUNT_BLUE: u8 = 14;

fn main() {
    println!("Welcome to day 2.");

    let file_path = "./src/02/input.txt";
    let file_content = fs::read_to_string(file_path).expect("File not found!");

    let games: Vec<Game> = file_content.lines().map(|line| parse_game(line)).collect();

    let sum_valid_games: u32 = games
        .iter()
        .filter_map(|game| game_valid(game, TOTAL_COUNT_RED, TOTAL_COUNT_GREEN, TOTAL_COUNT_BLUE))
        .sum();

    let sum_power_games: u32 = games.iter().map(|game| calculate_game_power(game)).sum();

    println!("Sum of the valid game ids: {:?}", sum_valid_games);
    println!("Sum of the games' power: {}", sum_power_games);
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<GameReveal>,
}

#[derive(Debug)]
struct GameReveal {
    red_cubes: Option<u8>,
    green_cubes: Option<u8>,
    blue_cubes: Option<u8>,
}

fn parse_game(game: &str) -> Game {
    let colon_idx = game.find(':').unwrap();
    let id = game[5..colon_idx].parse::<u32>().unwrap();

    let reveals: Vec<GameReveal> = game[colon_idx + 1..]
        .split(';')
        .map(|reveal| parse_reveal(reveal))
        .collect();

    return Game { id, reveals };
}

fn parse_reveal(reveal: &str) -> GameReveal {
    let mut green_count: Option<u8> = None;
    let mut red_count: Option<u8> = None;
    let mut blue_count: Option<u8> = None;

    reveal.split(',').for_each(|cube| {
        let reds = cube.find("red");
        let blues = cube.find("blue");
        let greens = cube.find("green");

        match reds {
            Some(idx) => red_count = Some(cube[1..idx - 1].parse::<u8>().unwrap()),
            None => (),
        }

        match blues {
            Some(idx) => blue_count = Some(cube[1..idx - 1].parse::<u8>().unwrap()),
            None => (),
        }

        match greens {
            Some(idx) => green_count = Some(cube[1..idx - 1].parse::<u8>().unwrap()),
            None => (),
        }
    });

    return GameReveal {
        red_cubes: red_count,
        green_cubes: green_count,
        blue_cubes: blue_count,
    };
}

/*
    Return Some(GameId) if game is valid, otherwise returns None
*/
fn game_valid(
    game: &Game,
    total_red_cubes: u8,
    total_green_cubes: u8,
    total_blue_cubes: u8,
) -> Option<u32> {
    let mut valid = true;
    game.reveals.iter().for_each(|reveal| {
        match reveal.red_cubes {
            Some(x) => {
                if x > total_red_cubes {
                    valid = false;
                }
            }
            None => (),
        }
        match reveal.green_cubes {
            Some(x) => {
                if x > total_green_cubes {
                    valid = false;
                }
            }
            None => (),
        }
        match reveal.blue_cubes {
            Some(x) => {
                if x > total_blue_cubes {
                    valid = false;
                }
            }
            None => (),
        }
    });

    if valid {
        return Some(game.id);
    }
    return None;
}

fn calculate_game_power(game: &Game) -> u32 {
    let mut fewest_red: u8 = 0;
    let mut fewest_green: u8 = 0;
    let mut fewest_blue: u8 = 0;

    game.reveals.iter().for_each(|reveal| {
        match reveal.red_cubes {
            Some(x) => {
                if x > fewest_red {
                    fewest_red = x;
                }
            }
            None => (),
        }

        match reveal.green_cubes {
            Some(x) => {
                if x > fewest_green {
                    fewest_green = x;
                }
            }
            None => (),
        }

        match reveal.blue_cubes {
            Some(x) => {
                if x > fewest_blue {
                    fewest_blue = x;
                }
            }
            None => (),
        }
    });

    return u32::from(fewest_red) * u32::from(fewest_green) * u32::from(fewest_blue);
}

#[cfg(test)]
mod tests {

    use super::*;

    const GAME1: &str = "Game 97: 5 red, 2 green; 8 red; 1 blue, 7 green, 2 red; 7 red, 15 green";
    const GAME2: &str = "Game 98: 6 green, 1 blue, 1 red; 3 green, 3 red; 1 blue, 13 green, 4 red";
    const GAME3: &str = "Game 9: 11 red; 1 green, 2 red, 2 blue; 1 blue, 2 green, 9 red; 4 red, 2 green, 2 blue; 1 blue, 2 green; 1 blue, 9 red, 2 green";

    #[test]
    fn test_parse_game_id() {
        let parsed_game1 = parse_game(GAME1);
        let parsed_game2 = parse_game(GAME2);
        let parsed_game3 = parse_game(GAME3);
        assert_eq!(parsed_game1.id, 97);
        assert_eq!(parsed_game2.id, 98);
        assert_eq!(parsed_game3.id, 9);
    }

    #[test]
    fn test_valid_game_id() {
        let parsed_game1 = parse_game(GAME1);
        let parsed_game2 = parse_game(GAME2);
        let parsed_game3 = parse_game(GAME3);
        assert_eq!(game_valid(&parsed_game1, 12, 13, 14), None);
        assert_eq!(game_valid(&parsed_game1, 12, 16, 14), Some(97));
        assert_eq!(game_valid(&parsed_game2, 3, 13, 14), None);
        assert_eq!(game_valid(&parsed_game2, 4, 13, 14), Some(98));
        assert_eq!(game_valid(&parsed_game3, 12, 13, 1), None);
        assert_eq!(game_valid(&parsed_game3, 12, 13, 14), Some(9));
    }

    #[test]
    fn test_game_power() {
        let parsed_game1 = parse_game(GAME1);
        let parsed_game2 = parse_game(GAME2);
        let parsed_game3 = parse_game(GAME3);
        assert_eq!(calculate_game_power(&parsed_game1), 120);
        assert_eq!(calculate_game_power(&parsed_game2), 52);
        assert_eq!(calculate_game_power(&parsed_game3), 44);
    }
}
