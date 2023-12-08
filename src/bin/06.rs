#![feature(test)]

use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{newline, space1},
    combinator::map_res,
    multi::separated_list0,
    IResult,
};

advent_of_code_2023::solution!(6);

pub fn part_1(input: &str) -> Option<u64> {
    let (input, races) = parse_input_as_list(input).unwrap();

    assert_eq!(input, "");

    let product = races.iter().map(Race::get_num_solutions).product::<u64>();

    Some(product)
}

pub fn part_2(input: &str) -> Option<u64> {
    let (input, race) = parse_input_as_single_race(input).unwrap();

    assert_eq!(input, "");

    Some(race.get_num_solutions())
}

#[derive(Debug)]
struct Race {
    race_time: u64,
    record_distance: u64,
}

impl Race {
    /// Finds the number of races in which we can beat the record by treating
    /// the distance as a funnction of the charge time and solving the inequality
    ///
    /// This is orders of magnitude more efficient than simply trying every possible
    /// value in the input range
    fn get_num_solutions(&self) -> u64 {
        // Helper to calculate the distance from the hold (charge) time
        let distance = |charge_time: u64| (self.race_time - charge_time) * charge_time;

        // Convert our numbers to doubles
        let race_time = self.race_time as f64;
        let record_distance = self.record_distance as f64;

        // Discriminant of our quadratic equation
        let discriminant = f64::sqrt(race_time * race_time - 4.0 * record_distance);

        // Solve for roots of quadratic (this gives us approximate values)
        let lower_bound = f64::ceil((race_time - discriminant) / 2.0) + 1.0;
        let upper_bound = f64::floor((race_time + discriminant) / 2.0) - 1.0;

        // Our input space is discrete
        let mut smallest = lower_bound as u64;
        let mut largest = upper_bound as u64;

        // Step backward until we stop winning
        loop {
            let time = smallest - 1;

            if distance(time) <= self.record_distance {
                break;
            }

            smallest = time;
        }

        // Step forward until we stop winning
        loop {
            let time = largest + 1;

            if distance(time) <= self.record_distance {
                break;
            }

            largest = time;
        }

        // Total number will be the difference in the range
        largest - smallest + 1
    }
}

// ================= PART 1 =================

fn parse_input_as_list(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = parse_line_as_list("Time:", input)?;
    let (input, _) = newline(input)?;
    let (input, distances) = parse_line_as_list("Distance:", input)?;

    let races: Vec<_> =
        times
            .into_iter()
            .zip(distances)
            .map(|t| Race {
                race_time: t.0,
                record_distance: t.1,
            })
            .collect();

    Ok((input, races))
}

fn parse_line_as_list<'a>(tag_str: &str, input: &'a str) -> IResult<&'a str, Vec<u64>> {
    let (input, _) = tag(tag_str)(input)?;
    let (input, _) = space1(input)?;
    let (input, ints) = separated_list0(space1, parse_int)(input)?;

    Ok((input, ints))
}

fn parse_int<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(take_while(|c: char| c.is_ascii_digit()), str::parse)(input)
}

// ================= PART 2 =================

fn parse_input_as_single_race(input: &str) -> IResult<&str, Race> {
    let (input, race_time) = parse_line_as_int("Time:", input)?;
    let (input, _) = newline(input)?;
    let (input, record_distance) = parse_line_as_int("Distance:", input)?;

    Ok((
        input,
        Race {
            race_time,
            record_distance,
        },
    ))
}

fn parse_line_as_int<'a>(tag_str: &str, input: &'a str) -> IResult<&'a str, u64> {
    let (input, _) = tag(tag_str)(input)?;
    let (input, _) = space1(input)?;
    let (input, ints) = separated_list0(space1, take_while(|c: char| c.is_ascii_digit()))(input)?;

    Ok((input, ints.concat().parse().unwrap()))
}
