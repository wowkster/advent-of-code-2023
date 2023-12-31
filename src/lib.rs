use std::str::FromStr;
use std::fmt::Debug;

#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        #[cfg(feature = "bench")]
        extern crate test;

        use advent_of_code_2023::{read_example_file, read_input_file, Part};

        const DAY: u32 = $day;

        fn main() {
            let input = read_input_file(DAY);

            let start_time = std::time::Instant::now();
            println!(
                "Part 1: {:?} (in {:.2?})",
                part_1(&input),
                start_time.elapsed()
            );

            let start_time = std::time::Instant::now();
            println!(
                "Part 2: {:?} (in {:.2?})",
                part_2(&input),
                start_time.elapsed()
            );
        }

        #[cfg(test)]
        mod tests {
            #[cfg(feature = "bench")]
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

            #[cfg(feature = "bench")]
            #[bench]
            fn bench_part_1(b: &mut test::Bencher) {
                let input = read_input_file(DAY);
                b.iter(|| black_box(part_1(&input)));
            }

            #[cfg(feature = "bench")]
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
    std::fs::read_to_string(path).unwrap().trim().to_string()
}

pub fn read_example_file<T: FromStr>(day: u32, part: Part) -> (T, String)
where
    <T as FromStr>::Err: Debug,
{
    let path = format!("data/examples/{:02}/part-{:01}.txt", day, part as u8);
    let file = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Example file not found: {}", path));

    let (expected_result, input) = file
        .split_once("\n---\n")
        .expect("Example file is not in the correct format");

    let expected_result = expected_result
        .trim()
        .parse::<T>()
        .expect("Expected result is not a number");

    let input = input.trim().to_string();

    (expected_result, input)
}
