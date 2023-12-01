pub fn sum_calibration_values(input: &str, extraction_fn: fn(&str) -> Vec<u32>) -> u32 {
    input
        .lines()
        .map(extraction_fn)
        .map(|digits| [*digits.first().unwrap(), *digits.last().unwrap()])
        .map(|[first, last]| format!("{}{}", first, last))
        .map(|number| number.parse::<u32>().unwrap())
        .sum()
}

pub fn extract_digits_1(line: &str) -> Vec<u32> {
    line.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn extract_digits_2(line: &str) -> Vec<u32> {
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

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./input.txt");

    #[test]
    fn part_1() {
        let result = sum_calibration_values(INPUT, extract_digits_1);
        assert_eq!(result, 55130);
    }

    #[test]
    fn part_1_example() {
        let input = "1abc2
                    pqr3stu8vwx
                    a1b2c3d4e5f
                    treb7uchet";

        let result = sum_calibration_values(input, extract_digits_1);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_2() {
        let result = sum_calibration_values(INPUT, extract_digits_2);
        assert_eq!(result, 54985);
    }

    #[test]
    fn part_2_example() {
        let input = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";

        let result = sum_calibration_values(input, extract_digits_2);
        assert_eq!(result, 281);
    }
}
