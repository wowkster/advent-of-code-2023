#![feature(test)]

advent_of_code_2023::solution!(4);

use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::space1,
    combinator::map_res,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use rayon::prelude::*;

pub fn part_1(input: &str) -> Option<u32> {
    let cards = parse_cards(input);

    Some(cards.iter().map(Card::score).sum())
}

pub fn part_2(input: &str) -> Option<u32> {
    let original_cards = parse_cards(input);

    let mut solution: HashMap<u32, u32> = HashMap::new();

    original_cards.iter().for_each(|card| {
        // Include the original counts for each card id
        *solution.entry(card.id).or_insert(0) += 1;

        // Get the current count for the card
        let current_amount = solution[&card.id];

        // For each match, add our current amount to that card. This works becauase
        // adding the current amount is like iterating it but cheaper.
        for i in 0..card.matches {
            *solution.entry(card.id + 1 + i).or_insert(0) += current_amount;
        }
    });

    Some(solution.values().sum())
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    matches: u32,
}

impl Card {
    pub fn new(id: u32, winning_numbers: Vec<u32>, my_numbers: Vec<u32>) -> Self {
        // OPTIMIZATION: Cache matches in constructor
        let matches = winning_numbers
            .iter()
            .filter(|w| my_numbers.contains(w))
            .count() as u32;

        Self { id, matches }
    }

    pub fn score(&self) -> u32 {
        if self.matches == 0 {
            return 0;
        }

        2u32.pow(self.matches - 1)
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    // OPTIMIZATION: Instead of using nom to parse out the newline separators,
    // we can parse in parallel which spreads out card `matches` compute
    input
        .par_lines()
        .map(|l| {
            let (_, card) = parse_card(l).unwrap();
            card
        })
        .collect()
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = parse_int(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;

    let (input, (winning_numbers, _, _, _, my_numbers)) =
        tuple((parse_int_list, space1, tag("|"), space1, parse_int_list))(input)?;

    Ok((input, Card::new(id, winning_numbers, my_numbers)))
}

fn parse_int_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(space1, parse_int)(input)
}

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 8, |c: char| c.is_ascii_digit()),
        str::parse,
    )(input)
}
