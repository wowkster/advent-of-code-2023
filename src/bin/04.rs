#![feature(test)]

advent_of_code_2023::solution!(4);

use std::{collections::HashMap, sync::Mutex};

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

    let sum = cards.iter().map(Card::score).sum();

    Some(sum)
}

#[inline]
pub fn part_2(input: &str) -> Option<u32> {
    // UNOPTIMIZATION: For the benchmark to be fair, we must ensure that the cache
    // is cleared between iterations
    COPIES_CACHE.lock().unwrap().clear();

    let original_cards = parse_cards(input);

    let total_cards = original_cards
        .par_iter()
        .map(|c| c.count_copies(&original_cards))
        .sum::<u32>();

    Some(total_cards)
}

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    matches: u32,
}

lazy_static::lazy_static! {
    static ref COPIES_CACHE: Mutex<HashMap<u32, u32>> = Mutex::default();
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

    pub fn count_copies(&self, original_cards: &[Card]) -> u32 {
        // OPTIMIZATION: Caches the copies internally because the same card
        // will always return the same number of new created cards

        let cached_count = COPIES_CACHE.lock().unwrap().get(&self.id).copied();

        if let Some(res) = cached_count {
            res
        } else {
            let res = self.count_copies_uncached(original_cards);

            COPIES_CACHE.lock().unwrap().insert(self.id, res);

            res
        }
    }

    #[inline]
    fn count_copies_uncached(&self, original_cards: &[Card]) -> u32 {
        if self.matches == 0 {
            return 1;
        }

        (0..self.matches)
            .into_par_iter()
            .map(|i| self.id + i)
            .map(|c| original_cards[c as usize].count_copies(original_cards))
            .sum::<u32>()
            + 1
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
    let (input, ints) = separated_list0(space1, parse_int)(input)?;

    Ok((input, ints))
}

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 8, |c: char| c.is_ascii_digit()),
        str::parse,
    )(input)
}
