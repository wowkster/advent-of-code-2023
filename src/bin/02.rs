#![feature(test)]

advent_of_code_2023::solution!(2);

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    multi::separated_list0,
    IResult,
};

pub fn part_1(input: &str) -> Option<u32> {
    let games = parse_games(input);
    let sum = games
        .iter()
        .filter(|g| g.is_valid())
        .map(|g| g.id as u32)
        .sum();

    Some(sum)
}

#[inline]
pub fn part_2(input: &str) -> Option<u32> {
    let games = parse_games(input);
    let sum = games.iter().map(|g| g.minimum_set().power()).sum();

    Some(sum)
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_valid(&self) -> bool {
        for set in &self.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                return false;
            }
        }

        true
    }

    pub fn minimum_set(&self) -> Set {
        let mut minimum_set = self.sets[0].clone();

        for set in &self.sets {
            if set.red > minimum_set.red {
                minimum_set.red = set.red;
            }

            if set.green > minimum_set.green {
                minimum_set.green = set.green;
            }

            if set.blue > minimum_set.blue {
                minimum_set.blue = set.blue;
            }
        }

        minimum_set
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Set {
    red: u8,
    green: u8,
    blue: u8,
}

impl Set {
    pub fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
}

fn parse_games(input: &str) -> Vec<Game> {
    let (input, games) = separated_list0(tag("\n"), parse_game)(input).unwrap();

    assert_eq!(input, "");

    games
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = parse_int(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, sets) = separated_list0(tag("; "), parse_set)(input)?;

    Ok((input, Game { id, sets }))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let (input, cubes) = separated_list0(tag(", "), parse_cube)(input)?;

    let mut set =
        Set {
            red: 0,
            green: 0,
            blue: 0,
        };

    for cube in cubes {
        match cube {
            (num, Color::Red) => set.red += num,
            (num, Color::Green) => set.green += num,
            (num, Color::Blue) => set.blue += num,
        }
    }

    Ok((input, set))
}

fn parse_cube(input: &str) -> IResult<&str, (u8, Color)> {
    let (input, num) = parse_int(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_color(input)?;

    Ok((input, (num, color)))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;

    match color {
        "red" => Ok((input, Color::Red)),
        "green" => Ok((input, Color::Green)),
        "blue" => Ok((input, Color::Blue)),
        _ => unreachable!(),
    }
}

fn parse_int(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
        str::parse,
    )(input)
}
