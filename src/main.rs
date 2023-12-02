use std::{fs, path::Path};

use aoc::Client;

fn main() {
    let session_token = std::env::var("AOC_SESSION_TOKEN")
        .unwrap_or_else(|_| abort!("AOC_SESSION_TOKEN environment variable not set"));

    print!("Initializing AOC client connection...");

    let client = Client::new("2023", session_token)
        .unwrap_or_else(|_| abort!("Failed to create AOC client"));

    println!("OK");

    if !Path::new("./data/inputs").exists() {
        fs::create_dir("./data/inputs").unwrap();
    }

    for day in 1..=25 {
        print!("Downloading day {:02}...", day);

        let Ok(input) = client.get_input(day) else {
            abort!("FAILED");
        };

        let path = format!("./data/inputs/{:02}.txt", day);
        fs::write(path, input).unwrap();

        println!("OK");
    }
}

#[macro_export]
macro_rules! abort {
    ($($arg:tt)*) => {
        {
            println!($($arg)*);
            std::process::exit(1);
        }
    };
}
