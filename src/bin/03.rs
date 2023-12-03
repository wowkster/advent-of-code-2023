#![feature(test)]
#![allow(clippy::needless_range_loop)]

use std::collections::HashSet;

advent_of_code_2023::solution!(3);

#[inline]
pub fn part_1(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        // Parsing state for the current line
        let mut parsing_number = false;
        let mut found_symbol = false;
        let mut current_number = 0;

        for (j, c) in line.chars().enumerate() {
            // If the char is not a digit, check if we are at the end of parsing a number.
            // If it is, and we found an adjacent symbol, then add it to the sum.
            // Otherwise, reset the state and keep going.
            if !c.is_ascii_digit() {
                if parsing_number && found_symbol {
                    sum += current_number;
                }

                parsing_number = false;
                found_symbol = false;
                current_number = 0;
                continue;
            }

            // We found a number, so keep track of its value
            parsing_number = true;
            current_number = current_number * 10 + c.to_digit(10).unwrap();

            // Compute a box 1 distance around the character, respecting index boundaries
            let min_y = i32::max(i as i32 - 1, 0) as usize;
            let max_y = i32::min(i as i32 + 1, (lines.len() - 1) as i32) as usize;
            let min_x = i32::max(j as i32 - 1, 0) as usize;
            let max_x = i32::min(j as i32 + 1, (lines.len() - 1) as i32) as usize;

            // Search around the character for a symbol
            for x in min_x..=max_x {
                for line in lines.iter().take(max_y + 1).skip(min_y) {
                    let char = line.chars().nth(x).unwrap();

                    if char != '.' && !char.is_ascii_digit() {
                        found_symbol = true;
                    }
                }
            }

            // EDGE CASE: if we are at the end of the line and are parsing a
            // number and found a symbol, make sure to track this value
            if j == line.len() - 1 && parsing_number && found_symbol {
                sum += current_number;
            }
        }
    }

    Some(sum)
}

#[inline]
pub fn part_2(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();

    let mut number_spans = Vec::new();
    let mut star_positions = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        // Parsing state for the current line
        let mut parsing_number = false;
        let mut found_star = false;
        let mut current_number = 0;
        let mut start_position = 0;

        for (j, c) in line.chars().enumerate() {
            // If the char is not a digit, check if we are at the end of parsing a number.
            // If it is, and we found an adjacent star, then keep track of the number span.
            // Otherwise, reset the state and keep going.
            if !c.is_ascii_digit() {
                if parsing_number && found_star {
                    number_spans.push(NumberSpan {
                        value: current_number,
                        start: Position {
                            col: start_position,
                            row: i,
                        },
                        length: j - start_position,
                    })
                }

                parsing_number = false;
                found_star = false;
                current_number = 0;
                start_position = 0;
                continue;
            }

            // We found a number, so keep track of its value
            if !parsing_number {
                start_position = j;
                parsing_number = true;
            }
            current_number = current_number * 10 + c.to_digit(10).unwrap();

            // Compute a box 1 distance around the character, respecting index boundaries
            let min_y = i.saturating_sub(1);
            let max_y = usize::min(i + 1, lines.len() - 1);
            let min_x = j.saturating_sub(1);
            let max_x = usize::min(j + 1, lines.len() - 1);

            // Search around the character for a symbol
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let char = lines[y].chars().nth(x).unwrap();

                    if char == '*' {
                        found_star = true;
                        star_positions.insert(Position { col: x, row: y });
                    }
                }
            }

            // EDGE CASE: if we are at the end of the line and are parsing a
            // number and found a star, make sure to track this value
            if j == line.len() - 1 && parsing_number && found_star {
                number_spans.push(NumberSpan {
                    value: current_number,
                    start: Position {
                        col: start_position,
                        row: i,
                    },
                    length: j - start_position + 1,
                })
            }
        }
    }

    let mut sum = 0;

    // For every star, check all the spans to check if there are exactly 2 around it
    star_positions.iter().for_each(|star| {
        // POTENTIAL OPTIMIZATION: since we know that spans will only be adjacent
        // if they are within 2 lines of the star, we could store the spans in a
        // structure representing their lines and reduce the number of spans we have
        // to check here
        let adjacent_spans = number_spans
            .iter()
            .filter(|span| is_adjacent(star, span))
            .collect::<Vec<_>>();

        if adjacent_spans.len() == 2 {
            sum += adjacent_spans.iter().map(|s| s.value).product::<u32>();
        }
    });

    Some(sum)
}

/// Checks to see if a star is adjacent to the given number span
fn is_adjacent(star: &Position, span: &NumberSpan) -> bool {
    let star_left = star.col.saturating_sub(1);
    let star_right = star.col.saturating_add(1);
    let star_top = star.row.saturating_add(1);
    let star_bottom = star.row.saturating_sub(1);

    let span_left = span.start.col;
    let span_right = span.start.col + span.length - 1;
    let span_top = span.start.row;
    let span_bottom = span.start.row;

    star_left <= span_right
        && star_right >= span_left
        && star_top >= span_bottom
        && star_bottom <= span_top
}

#[derive(Debug, PartialEq)]
struct NumberSpan {
    pub value: u32,
    pub start: Position,
    pub length: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    pub row: usize,
    pub col: usize,
}
