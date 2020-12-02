use structopt::StructOpt;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod days;
mod structs;
mod utils;

use structs::*;
use utils::*;

fn main() -> NullResult {
    let args = Cli::from_args();

    let is_test = args.test.is_some();
    let maybe_test = if let Some(ref t) = args.test {
        format!("_{}", t)
    } else {
        String::new()
    };
    let input: std::path::PathBuf =
        format!("inputs/day_{}_{}{}.txt", args.day, args.step, maybe_test)
            .parse()
            .expect("arguments to form a valid path string");
    let day = Day::try_from(args.day).expect("day to be a valid AOC day (1-25)");
    let step = Step::try_from(args.step).expect("step to be a valid number (1-2)");

    let file = File::open(input)?;
    let mut data: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let expected = if is_test { data.pop() } else { None };

    run_day(day, step, data, expected)
}

fn run_day(day: Day, step: Step, data: Vec<String>, expected: Option<String>) -> NullResult {
    match day {
        Day::One => {
            if let Some(expected_value) = expected {
                days::day1::run_test(step, data, expected_value)?;
            } else {
                days::day1::run(step, data)?;
            }
        }
    }

    Ok(())
}
