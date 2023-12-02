#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        extern crate test;

        use advent_of_code_2023::{read_example_file, read_input_file, Part};

        const DAY: u32 = $day;

        fn main() {
            let input = read_input_file(DAY);
            println!("Part 1: {:?}", part_1(&input));
            println!("Part 2: {:?}", part_2(&input));
        }

        #[cfg(test)]
        mod tests {
            use test::{black_box, Bencher};

            use super::*;

            #[test]
            fn part_1_example() {
                let (expected_result, input) = read_example_file(DAY, Part::One);
                let result = part_1(&input);

                assert_eq!(result, Some(expected_result));
            }

            #[test]
            fn part_2_example() {
                let (expected_result, input) = read_example_file(DAY, Part::Two);
                let result = part_2(&input);

                assert_eq!(result, Some(expected_result));
            }

            #[bench]
            fn bench_part_1(b: &mut test::Bencher) {
                let input = read_input_file(DAY);
                b.iter(|| black_box(part_1(&input)));
            }

            #[bench]
            fn bench_part_2(b: &mut test::Bencher) {
                let input = read_input_file(DAY);
                b.iter(|| black_box(part_2(&input)));
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Part {
    One = 1,
    Two = 2,
}

pub fn read_input_file(day: u32) -> String {
    let path = format!("data/inputs/{:02}.txt", day);
    std::fs::read_to_string(path).unwrap()
}

pub fn read_example_file(day: u32, part: Part) -> (u32, String) {
    let path = format!("data/examples/{:02}/part-{:01}.txt", day, part as u8);
    let file = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Example file not found: {}", path));

    let (expected_result, input) = file
        .split_once("\n---\n")
        .expect("Example file is not in the correct format");

    let expected_result = expected_result
        .trim()
        .parse::<u32>()
        .expect("Expected result is not a number");

    let input = input.trim().to_string();

    (expected_result, input)
}
