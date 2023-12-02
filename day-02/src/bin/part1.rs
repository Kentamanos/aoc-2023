use nom::multi::separated_list1;
use nom::{branch::alt, bytes::complete::tag, IResult};

use nom::character::complete::{multispace0, multispace1};

#[derive(Debug)]
struct Game {
    game_number: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

struct Draw {
    count: u32,
    color: Color,
}

fn color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;

    match color {
        "red" => Ok((input, Color::Red)),
        "green" => Ok((input, Color::Green)),
        "blue" => Ok((input, Color::Blue)),
        _ => {
            panic!("Unknown color: {}", color)
        }
    }
}

fn draw(input: &str) -> IResult<&str, Draw> {
    let (input, _) = multispace0(input)?;
    let (input, count) = nom::character::complete::u32(input)?;
    let (input, _) = multispace1(input)?;
    let (input, color) = color(input)?;

    Ok((input, Draw { count, color }))
}

fn round(input: &str) -> IResult<&str, Round> {
    let (input, draws) = separated_list1(tag(","), draw)(input)?;

    let (mut r, mut g, mut b) = (0, 0, 0);
    for draw in draws {
        match draw.color {
            Color::Red => r += draw.count,
            Color::Green => g += draw.count,
            Color::Blue => b += draw.count,
        }
    }

    Ok((
        input,
        Round {
            red: r,
            green: g,
            blue: b,
        },
    ))
}

fn game_number(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, game_number) = nom::character::complete::digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let gn = u32::from_str_radix(game_number, 10).unwrap();
    Ok((input, gn))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, game_number) = game_number(input)?;
    let (input, rounds) = separated_list1(tag(";"), round)(input)?;

    Ok((
        input,
        Game {
            game_number,
            rounds,
        },
    ))
}

fn part1(input: &str) -> u32 {
    let max_round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut total_of_game_numbers = 0;
    for line in input.lines() {
        let (remaining, g) = game(line).unwrap();
        assert_eq!(remaining, "");

        let valid_game = g.rounds.iter().all(|round| {
            round.red <= max_round.red
                && round.green <= max_round.green
                && round.blue <= max_round.blue
        });

        if valid_game {
            total_of_game_numbers += g.game_number;
        }
    }
    total_of_game_numbers
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test.txt");
        assert_eq!(part1(test), 8);
    }
}
