#![feature(test)]

use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use nom::{
    bytes::complete::take_while,
    character::complete::{anychar, newline, space1},
    combinator::map_res,
    multi::{count, separated_list0},
    IResult,
};

advent_of_code_2023::solution!(7);

pub fn part_1(input: &str) -> Option<u64> {
    let (input, mut hands) = parse_input(input).unwrap();

    assert_eq!(input, "");

    hands.sort_by(|a, b| a.cmp(b, Hand::get_kind_simple, Card::get_value));

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum();

    Some(sum)
}

pub fn part_2(input: &str) -> Option<u64> {
    let (input, mut hands) = parse_input(input).unwrap();

    assert_eq!(input, "");

    hands.sort_by(|a, b| {
        a.cmp(
            b,
            Hand::get_kind_with_wildcards,
            Card::get_value_with_wildcard,
        )
    });

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum();

    Some(sum)
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn get_value(&self) -> u8 {
        *self as u8
    }

    fn get_value_with_wildcard(&self) -> u8 {
        const ONE: u8 = Card::One as u8;
        const TEN: u8 = Card::Ten as u8;
        const JACK: u8 = Card::Jack as u8;

        match *self as u8 {
            // Jack is lowest
            JACK => 0,
            // All up to jack need to be shifted up one
            x @ ONE..=TEN => x + 1,
            x => x,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
#[repr(u8)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn get_kind_simple(&self) -> HandKind {
        let mut card_counts: HashMap<Card, u32> = HashMap::new();

        // Add up the counts of each card in the hand
        for card in &self.cards {
            *card_counts.entry(*card).or_insert(0) += 1;
        }

        // Check for 5 of a kind
        if card_counts.values().any(|count| *count == 5) {
            return HandKind::FiveOfAKind;
        }

        // Check for 4 of a kind
        if card_counts.values().any(|count| *count == 4) {
            return HandKind::FourOfAKind;
        }

        // Check for full house (can only have 5 cards total, so this always works)
        if card_counts.values().any(|count| *count == 2)
            && card_counts.values().any(|count| *count == 3)
        {
            return HandKind::FullHouse;
        }

        // Check for 3 of a kind (already checked for full house, so this always works)
        if card_counts.values().any(|count| *count == 3) {
            return HandKind::ThreeOfAKind;
        }

        // Count the remaining number of pairs
        let num_pairs = card_counts.values().filter(|count| **count == 2).count();

        // Check for the rest of the hand kinds
        match num_pairs {
            2 => HandKind::TwoPair,
            1 => HandKind::OnePair,
            0 => HandKind::HighCard,
            _ => unreachable!(),
        }
    }

    fn get_kind_with_wildcards(&self) -> HandKind {
        let mut card_counts: HashMap<Card, u32> = HashMap::new();

        // Add up the counts of each card in the hand
        for card in &self.cards {
            *card_counts.entry(*card).or_insert(0) += 1;
        }

        let has_card_amount_excluding_jacks =
            |card_counts: &HashMap<Card, u32>, expected_count: u32| {
                card_counts
                    .iter()
                    .any(|(card, count)| *count == expected_count && *card != Card::Jack)
            };

        let num_jacks = *card_counts.entry(Card::Jack).or_default();

        // Check for 5 of a kind
        if card_counts.values().any(|count| *count == 5)
            || (has_card_amount_excluding_jacks(&card_counts, 4) && num_jacks == 1)
            || (has_card_amount_excluding_jacks(&card_counts, 3) && num_jacks == 2)
            || (has_card_amount_excluding_jacks(&card_counts, 2) && num_jacks == 3)
            || (has_card_amount_excluding_jacks(&card_counts, 1) && num_jacks == 4)
        {
            return HandKind::FiveOfAKind;
        }

        // Check for 4 of a kind
        if card_counts.values().any(|count| *count == 4)
            || (has_card_amount_excluding_jacks(&card_counts, 3) && num_jacks == 1)
            || (has_card_amount_excluding_jacks(&card_counts, 2) && num_jacks == 2)
            || (has_card_amount_excluding_jacks(&card_counts, 1) && num_jacks == 3)
        {
            return HandKind::FourOfAKind;
        }

        // Count the remaining number of pairs
        let num_pairs = card_counts.values().filter(|count| **count == 2).count();

        // Check for full house (can only have 5 cards total, so this always works)
        if (card_counts.values().any(|count| *count == 2)
            && card_counts.values().any(|count| *count == 3))
            || (num_pairs == 2 && num_jacks == 1)
        {
            return HandKind::FullHouse;
        }

        // Check for 3 of a kind (already checked for full house, so this always works)
        if card_counts.values().any(|count| *count == 3)
            || (has_card_amount_excluding_jacks(&card_counts, 2) && num_jacks == 1)
            || (has_card_amount_excluding_jacks(&card_counts, 1) && num_jacks == 2)
        {
            return HandKind::ThreeOfAKind;
        }

        // Check for the rest of the hand kinds
        match (num_pairs, num_jacks) {
            (2, _) | (1, 1) => HandKind::TwoPair,
            (1, _) | (0, 1) => HandKind::OnePair,
            (0, _) => HandKind::HighCard,
            _ => unreachable!(),
        }
    }

    fn cmp(
        &self,
        other: &Self,
        get_kind_fn: fn(&Hand) -> HandKind,
        get_card_value_fn: fn(&Card) -> u8,
    ) -> Ordering {
        let self_kind = get_kind_fn(self) as u8;
        let other_kind = get_kind_fn(other) as u8;

        // Simple case (hand kinds are not equal)
        #[allow(clippy::comparison_chain)]
        if self_kind < other_kind {
            return Ordering::Less;
        } else if self_kind > other_kind {
            return Ordering::Greater;
        }

        // Complex tie breaking case (must respect other rules)
        for i in 0..5 {
            let self_value = get_card_value_fn(&self.cards[i]);
            let other_value = get_card_value_fn(&other.cards[i]);

            if self_value < other_value {
                return Ordering::Less;
            }
            if self_value > other_value {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}

// ================== PARSING ==================

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list0(newline, parse_hand)(input)?;

    Ok((input, hands))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = count(parse_card, 5)(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = parse_int(input)?;

    Ok((
        input,
        Hand {
            cards: cards.try_into().unwrap(),
            bid,
        },
    ))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map_res(anychar, Card::try_from)(input)
}

fn parse_int<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(take_while(|c: char| c.is_ascii_digit()), str::parse)(input)
}
