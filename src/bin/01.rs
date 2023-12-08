#![feature(test)]

advent_of_code_2023::solution!(1);

use rayon::prelude::*;

pub fn part_1(input: &str) -> Option<u32> {
    Some(input.lines().map(extract_calibration_value_1).sum())
}

pub fn part_2(input: &str) -> Option<u32> {
    Some(input.par_lines().map(extract_calibration_value_2).sum())
}

pub fn extract_calibration_value_1(line: &str) -> u32 {
    let mut digits = line
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .peekable();

    let first = *digits.peek().unwrap();
    let last = digits.last().unwrap();

    (first - b'0') as u32 * 10 + (last - b'0') as u32
}

pub fn extract_calibration_value_2(line: &str) -> u32 {
    let chars: Vec<_> = line.chars().collect();

    let mut digits = Vec::new();

    for i in -4..chars.len() as isize {
        let min = i.clamp(0, chars.len() as isize);
        let max = (i + 5).clamp(0, chars.len() as isize);
        let window = &chars[min as usize..max as usize];

        let digit = match window {
            ['0'..='9', ..] => window[0].to_digit(10),
            ['o', 'n', 'e', ..] => Some(1),
            ['t', 'w', 'o', ..] => Some(2),
            ['t', 'h', 'r', 'e', 'e'] => Some(3),
            ['f', 'o', 'u', 'r', ..] => Some(4),
            ['f', 'i', 'v', 'e', ..] => Some(5),
            ['s', 'i', 'x', ..] => Some(6),
            ['s', 'e', 'v', 'e', 'n'] => Some(7),
            ['e', 'i', 'g', 'h', 't'] => Some(8),
            ['n', 'i', 'n', 'e', ..] => Some(9),
            _ => None,
        };

        if let Some(digit) = digit {
            digits.push(digit);
        }
    }

    *digits.first().unwrap() * 10 + *digits.last().unwrap()
}
