#![feature(test)]

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, newline},
    combinator::map_res,
    multi::{count, many1, separated_list0},
    IResult,
};
use rayon::prelude::*;

advent_of_code_2023::solution!(8);

pub fn part_1(input: &str) -> Option<u64> {
    let (input, (instructions, network)) = parse_input(input).unwrap();

    assert_eq!(input, "");

    Some(count_steps(&instructions, &network, "AAA", |n| n == "ZZZ"))
}

#[inline]
pub fn part_2(input: &str) -> Option<u64> {
    let (input, (instructions, network)) = parse_input(input).unwrap();

    assert_eq!(input, "");

    let steps = network
        .par_iter()
        // Get all node ids (no method for getting keys in parallel)
        .map(|(k, _)| k)
        // Find all the starting node ids
        .filter(|k| k.ends_with('A'))
        // Find the individual path for each node
        .map(|node| count_steps(&instructions, &network, node, |n| n.ends_with('Z')))
        // Find the LCM of all the paths to find the total step count
        .reduce(|| 1, num::integer::lcm);

    Some(steps)
}

fn count_steps(
    instructions: &InstructionList,
    network: &Network,
    starting_node: &str,
    end_condition_predicate: fn(&str) -> bool,
) -> u64 {
    let mut steps = 0;
    let mut current_node = starting_node;

    'outer: loop {
        for instruction in instructions {
            if end_condition_predicate(current_node) {
                break 'outer;
            }

            let node = network.get(current_node).unwrap();

            current_node =
                match instruction {
                    Instruction::Left => node.0,
                    Instruction::Right => node.1,
                };

            steps += 1;
        }
    }

    steps
}

// ================== TYPES ==================

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let instruction =
            match value {
                'L' => Self::Left,
                'R' => Self::Right,
                _ => return Err(()),
            };

        Ok(instruction)
    }
}

type Node<'a> = (&'a str, (&'a str, &'a str));
type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;
type InstructionList = Vec<Instruction>;

// ================== PARSING ==================

fn parse_input(input: &str) -> IResult<&str, (InstructionList, Network)> {
    let (input, instructions) = many1(parse_instruction)(input)?;
    let (input, _) = count(newline, 2)(input)?;
    let (input, nodes) = separated_list0(newline, parse_node)(input)?;

    let network: HashMap<_, _> = nodes.into_iter().collect();

    Ok((input, (instructions, network)))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, node_id) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = alphanumeric1(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = alphanumeric1(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (node_id, (left, right))))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map_res(anychar, Instruction::try_from)(input)
}
