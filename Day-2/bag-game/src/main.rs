use color_eyre::eyre::Result;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Color {
    Blue,
    Green,
    Red,
}

#[derive(Debug, PartialEq)]
struct ColorCount {
    color: Color,
    count: usize,
}

#[derive(Debug, PartialEq)]
struct Round(Vec<ColorCount>);

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn main() {
    color_eyre::install().unwrap();
    install_tracing("info");
    info!("Starting up...");

    // 1) Read input file
    let input = read_input("../input.txt").unwrap();

    // 2) Parse input file
    let data = parse_data(input).unwrap();

    // 3) Process data
    let total = data.iter().fold(0, |acc, game| {
        if is_feasible(game) {
            acc + game.id
        } else {
            acc
        }
    });

    // 4) Print result
    println!("Total: {}", total);

    // 5) Determine power
    let total_power = data.iter().fold(0, |acc, game| {
        let power = get_power(game);
        acc + power
    });

    // 6) Print result
    println!("Total Power: {}", total_power);

    info!("Winding Down...");
}

#[tracing::instrument]
fn read_input(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }

    Ok(result)
}

// Parse a single color
fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color_str) = nom::branch::alt((tag("blue"), tag("green"), tag("red")))(input)?;
    let color = match color_str {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        _ => unreachable!(),
    };
    Ok((input, color))
}

// Parse a color count pair
fn parse_color_count(input: &str) -> IResult<&str, ColorCount> {
    let (input, (count, _, color)) =
        tuple((map_res(digit1, str::parse::<usize>), space1, parse_color))(input)?;
    Ok((input, ColorCount { color, count }))
}

// Parse a round
fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, list) = separated_list1(tag(", "), parse_color_count)(input)?;
    Ok((input, Round(list)))
}

// Parse a game
fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list1(tag("; "), parse_round)(input)?;
    Ok((input, Game { id, rounds }))
}

// Determine feasibility of a game
fn is_feasible(game: &Game) -> bool {
    game.rounds.iter().all(|round| {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        round
            .0
            .iter()
            .for_each(|color_count| match color_count.color {
                Color::Blue => blue += color_count.count,
                Color::Green => green += color_count.count,
                Color::Red => red += color_count.count,
            });

        blue <= 14 && green <= 13 && red <= 12
    })
}

fn get_power(game: &Game) -> usize {
    let mut blue_max = 0;
    let mut green_max = 0;
    let mut red_max = 0;

    game.rounds.iter().for_each(|round| {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        round
            .0
            .iter()
            .for_each(|color_count| match color_count.color {
                Color::Blue => {
                    blue += color_count.count;
                    blue_max = blue_max.max(blue);
                }
                Color::Green => {
                    green += color_count.count;
                    green_max = green_max.max(green);
                }
                Color::Red => {
                    red += color_count.count;
                    red_max = red_max.max(red);
                }
            });
    });

    blue_max * green_max * red_max
}

// parse a vector of games
#[tracing::instrument]
fn parse_data(input: Vec<String>) -> Result<Vec<Game>> {
    let mut result = Vec::new();
    input.iter().for_each(|line| {
        let (_, game) = parse_game(line).unwrap();
        result.push(game);
    });

    Ok(result)
}

fn install_tracing(level: &str) {
    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or(level)).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    fn test_read_input() {
        let result = read_input("../test-1.txt").unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(
            result[0],
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        );
        assert_eq!(
            result[1],
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
        );
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("blue"), Ok(("", Color::Blue)));
        assert_eq!(parse_color("green"), Ok(("", Color::Green)));
        assert_eq!(parse_color("red"), Ok(("", Color::Red)));
        assert!(parse_color("invalid").is_err());
    }

    #[test]
    fn test_parse_color_count() {
        assert_eq!(
            parse_color_count("3 blue"),
            Ok((
                "",
                ColorCount {
                    color: Color::Blue,
                    count: 3
                }
            ))
        );
        assert_eq!(
            parse_color_count("4 red"),
            Ok((
                "",
                ColorCount {
                    color: Color::Red,
                    count: 4
                }
            ))
        );
        assert_eq!(
            parse_color_count("2 green"),
            Ok((
                "",
                ColorCount {
                    color: Color::Green,
                    count: 2
                }
            ))
        );
        assert!(parse_color_count("invalid").is_err());
    }

    #[test]
    fn test_parse_round() {
        let round = Round(vec![
            ColorCount {
                color: Color::Blue,
                count: 3,
            },
            ColorCount {
                color: Color::Red,
                count: 4,
            },
        ]);
        assert_eq!(parse_round("3 blue, 4 red"), Ok(("", round)));
        assert!(parse_round("invalid").is_err());
    }

    #[test]
    fn test_parse_game() {
        let game_str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_game = Game {
            id: 1,
            rounds: vec![
                Round(vec![
                    ColorCount {
                        color: Color::Blue,
                        count: 3,
                    },
                    ColorCount {
                        color: Color::Red,
                        count: 4,
                    },
                ]),
                Round(vec![
                    ColorCount {
                        color: Color::Red,
                        count: 1,
                    },
                    ColorCount {
                        color: Color::Green,
                        count: 2,
                    },
                    ColorCount {
                        color: Color::Blue,
                        count: 6,
                    },
                ]),
                Round(vec![ColorCount {
                    color: Color::Green,
                    count: 2,
                }]),
            ],
        };
        assert_eq!(parse_game(game_str), Ok(("", expected_game)));

        let game_str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected_game = Game {
            id: 2,
            rounds: vec![
                Round(vec![
                    ColorCount {
                        color: Color::Blue,
                        count: 1,
                    },
                    ColorCount {
                        color: Color::Green,
                        count: 2,
                    },
                ]),
                Round(vec![
                    ColorCount {
                        color: Color::Green,
                        count: 3,
                    },
                    ColorCount {
                        color: Color::Blue,
                        count: 4,
                    },
                    ColorCount {
                        color: Color::Red,
                        count: 1,
                    },
                ]),
                Round(vec![
                    ColorCount {
                        color: Color::Green,
                        count: 1,
                    },
                    ColorCount {
                        color: Color::Blue,
                        count: 1,
                    },
                ]),
            ],
        };
        assert_eq!(parse_game(game_str), Ok(("", expected_game)));

        assert!(parse_game("invalid").is_err());
    }

    #[test]
    fn test_parse_data() {
        let input = read_input("../test-1.txt").unwrap();

        let expected = vec![
            Game {
                id: 1,
                rounds: vec![
                    Round(vec![
                        ColorCount {
                            color: Color::Blue,
                            count: 3,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 4,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Red,
                            count: 1,
                        },
                        ColorCount {
                            color: Color::Green,
                            count: 2,
                        },
                        ColorCount {
                            color: Color::Blue,
                            count: 6,
                        },
                    ]),
                    Round(vec![ColorCount {
                        color: Color::Green,
                        count: 2,
                    }]),
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Round(vec![
                        ColorCount {
                            color: Color::Blue,
                            count: 1,
                        },
                        ColorCount {
                            color: Color::Green,
                            count: 2,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Green,
                            count: 3,
                        },
                        ColorCount {
                            color: Color::Blue,
                            count: 4,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 1,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Green,
                            count: 1,
                        },
                        ColorCount {
                            color: Color::Blue,
                            count: 1,
                        },
                    ]),
                ],
            },
            Game {
                id: 3,
                rounds: vec![
                    Round(vec![
                        ColorCount {
                            color: Color::Green,
                            count: 8,
                        },
                        ColorCount {
                            color: Color::Blue,
                            count: 6,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 20,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Blue,
                            count: 5,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 4,
                        },
                        ColorCount {
                            color: Color::Green,
                            count: 13,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Green,
                            count: 5,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 1,
                        },
                    ]),
                ],
            },
            Game {
                id: 4,
                rounds: vec![
                    Round(vec![
                        ColorCount {
                            color: Color::Green,
                            count: 1,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 3,
                        },
                        ColorCount {
                            color: Color::Blue,
                            count: 6,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Green,
                            count: 3,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 6,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Green,
                            count: 3,
                        },
                        ColorCount {
                            color: Color::Blue,
                            count: 15,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 14,
                        },
                    ]),
                ],
            },
            Game {
                id: 5,
                rounds: vec![
                    Round(vec![
                        ColorCount {
                            color: Color::Red,
                            count: 6,
                        },
                        ColorCount {
                            color: Color::Blue,
                            count: 1,
                        },
                        ColorCount {
                            color: Color::Green,
                            count: 3,
                        },
                    ]),
                    Round(vec![
                        ColorCount {
                            color: Color::Blue,
                            count: 2,
                        },
                        ColorCount {
                            color: Color::Red,
                            count: 1,
                        },
                        ColorCount {
                            color: Color::Green,
                            count: 2,
                        },
                    ]),
                ],
            },
        ];

        let result = parse_data(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_feasible() {
        let input = read_input("../test-1.txt").unwrap();
        let data = parse_data(input).unwrap();
        assert!(is_feasible(&data[0]));
        assert!(is_feasible(&data[1]));
        assert!(!is_feasible(&data[2]));
        assert!(!is_feasible(&data[3]));
        assert!(is_feasible(&data[4]));
    }

    #[test]
    fn test_sum() {
        let input = read_input("../test-1.txt").unwrap();
        let data = parse_data(input).unwrap();
        let total = data.iter().fold(0, |acc, game| {
            if is_feasible(game) {
                acc + game.id
            } else {
                acc
            }
        });
        assert_eq!(total, 8);
    }
}
