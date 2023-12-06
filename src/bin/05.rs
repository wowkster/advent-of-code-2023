#![feature(test)]

advent_of_code_2023::solution!(5);

use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, newline, space1},
    combinator::map_res,
    multi::{count, separated_list0},
    sequence::tuple,
    IResult,
};

pub fn part_1(input: &str) -> Option<i64> {
    let (input, (seeds, maps)) = parse_input(input, parse_seed_list).unwrap();

    assert_eq!(input, "");

    let locations = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |acc, m| m.transform_value(acc)));

    locations.min()
}

#[inline]
pub fn part_2(input: &str) -> Option<i64> {
    let (input, (seeds, maps)) = parse_input(input, parse_seed_range_list).unwrap();

    assert_eq!(input, "");

    let mut ranges = seeds;
    let mut transformed_seed_ranges = Vec::new();

    for map in maps {
        for range in &ranges {
            transformed_seed_ranges.extend(map.transform_range(range))
        }

        std::mem::swap(&mut ranges, &mut transformed_seed_ranges);
        transformed_seed_ranges.clear();
    }

    ranges.sort_by_key(|r| r.start);

    Some(ranges[0].start)
}

#[derive(Debug)]
struct Map(Vec<MapRange>);

impl Map {
    pub fn new(mut ranges: Vec<MapRange>) -> Self {
        ranges.sort_by_key(|r| r.src_range.start);
        Self(ranges)
    }

    pub fn transform_value(&self, value: i64) -> i64 {
        for range in &self.0 {
            if !range.contains_value(value) {
                continue;
            }

            let offset = range.dest_range.start - range.src_range.start;
            return value + offset;
        }

        value
    }

    /// Uses the rules of the map to transform a range to one or more new ranges
    pub fn transform_range(&self, input_range: &Range<i64>) -> Vec<Range<i64>> {
        let mut result_ranges = Vec::new();

        // Create sorted list of ranges that are relevant to the input (partially or fully cover the input)
        let relevant_ranges: Vec<_> = self
            .0
            .iter()
            .filter(|mr| mr.partially_covers_range(input_range))
            .collect();

        // If this map does not cover any parts of the input range, we can just return the input range
        if relevant_ranges.is_empty() {
            return vec![input_range.clone()];
        }

        let mut current_range = input_range.clone();

        for relevant_range in relevant_ranges {
            // Start of current range is not covered by this range
            // Current Range:   |--------------|
            // Relevant Range:   |-----|
            if current_range.start < relevant_range.src_range.start {
                result_ranges.push(current_range.start..relevant_range.src_range.start);
                current_range.start = relevant_range.src_range.start;
            }

            // Current range is fully contained within this relevant range (end case)
            // Current Range:  |------|
            // Relevant Range: |----------|
            // or
            // Current Range:    |----|
            // Relevant Range: |----------|
            if relevant_range.src_range.end > current_range.end {
                result_ranges.push(
                    relevant_range.map_value(current_range.start)
                        ..relevant_range.map_value(current_range.end - 1) + 1,
                );
                current_range.start = current_range.end;
                break;
            }

            // Relevant range covers part of the start of the current range
            // Current Range:  |------------|
            // Relevant Range: |-----| ...
            // or
            // Current Range:    |----------|
            // Relevant Range: |-----| ...
            if relevant_range.src_range.end < current_range.end {
                result_ranges.push(
                    relevant_range.map_value(current_range.start)
                        ..relevant_range.map_value(relevant_range.src_range.end - 1) + 1,
                );
                current_range.start = relevant_range.src_range.end;
                continue;
            }

            // Current range end lines up with relevant range end
            // Current Range:    |----|
            // Relevant Range: |------|
            // or
            // Current Range:  |------|
            // Relevant Range: |------|
            if relevant_range.src_range.end == current_range.end {
                result_ranges.push(
                    relevant_range.map_value(current_range.start)
                        ..relevant_range.map_value(current_range.end - 1) + 1,
                );
                current_range.start = current_range.end;
                break;
            }
        }

        if current_range.start != current_range.end {
            result_ranges.push(current_range);
        }

        result_ranges
    }
}

#[derive(Debug)]
struct MapRange {
    dest_range: Range<i64>,
    src_range: Range<i64>,
}

impl MapRange {
    pub fn new(dest_range_start: i64, src_range_start: i64, range_length: i64) -> Self {
        Self {
            dest_range: dest_range_start..dest_range_start + range_length,
            src_range: src_range_start..src_range_start + range_length,
        }
    }

    pub fn contains_value(&self, value: i64) -> bool {
        self.src_range.contains(&value)
    }

    pub fn map_value(&self, value: i64) -> i64 {
        if !self.contains_value(value) {
            return value;
        }

        let offset = self.dest_range.start - self.src_range.start;
        value + offset
    }

    pub fn partially_covers_range(&self, range: &Range<i64>) -> bool {
        range.end >= self.src_range.start && range.start <= self.src_range.end
    }
}

fn parse_input<T>(
    input: &str,
    seed_parsing_fn: fn(&str) -> IResult<&str, T>,
) -> IResult<&str, (T, Vec<Map>)> {
    let (input, seeds) = seed_parsing_fn(input)?;
    let (input, _) = count(newline, 2)(input)?;
    let (input, maps) = separated_list0(count(newline, 2), parse_map)(input)?;

    Ok((input, (seeds, maps)))
}

fn parse_seed_list(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list0(space1, parse_int)(input)?;

    Ok((input, seeds))
}

fn parse_seed_range_list(input: &str) -> IResult<&str, Vec<Range<i64>>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list0(space1, tuple((parse_int, space1, parse_int)))(input)?;

    let seeds = seeds
        .iter()
        .map(|(start, _, length)| *start..(start + length))
        .collect::<Vec<_>>();

    Ok((input, seeds))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:"), newline))(input)?;

    let (input, ranges) = separated_list0(newline, parse_map_range)(input)?;

    Ok((input, Map::new(ranges)))
}

fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    let (input, (dest_range_start, _, src_range_start, _, range_length)) =
        tuple((parse_int, space1, parse_int, space1, parse_int))(input)?;

    Ok((
        input,
        MapRange::new(dest_range_start, src_range_start, range_length),
    ))
}

fn parse_int(input: &str) -> IResult<&str, i64> {
    map_res(take_while(|c: char| c.is_ascii_digit()), str::parse)(input)
}
