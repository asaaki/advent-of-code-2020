use crate::structs::*;
use crate::utils::*;
use std::collections::HashMap;
// use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let numbers: Vec<_> = input[0]
        .split(",")
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    match step {
        Step::One => {
            let now = std::time::Instant::now();

            let last_spoken = run_turns(numbers, 2020);

            let elapsed = now.elapsed();
            let result: String = format!("{}", last_spoken);
            println!("Result = {}", result);
            println!(
                "[run] step took: {}ms ({}us)",
                elapsed.as_millis(),
                elapsed.as_micros()
            );

            Ok(result)
        }
        Step::Two => {
            let now = std::time::Instant::now();

            // probably the slowest part so far
            let last_spoken = run_turns(numbers, 30_000_000);

            let elapsed = now.elapsed();
            let result: String = format!("{}", last_spoken);
            println!("Result = {}", result);
            println!(
                "[run] step took: {}ms ({}us)",
                elapsed.as_millis(),
                elapsed.as_micros()
            );

            Ok(result)
        }
    }
}

// this is extremely slow for high number of turns;
// what would be the better option to solve it then?
fn run_turns(numbers: Vec<usize>, final_turn: usize) -> usize {
    let mut ages = HashMap::<usize, usize>::new();
    for (turn0, num) in numbers.iter().enumerate() {
        ages.insert(*num, turn0 + 1);
    }

    ((numbers.len() + 1)..=final_turn)
        .fold((0, 0), |(mut last_turn, _), turn| {
            let to_speak = if last_turn == 0 {
                0
            } else {
                (turn - 1) - last_turn
            };
            last_turn = *(ages.get(&to_speak).unwrap_or(&0));
            ages.insert(to_speak, turn);
            (last_turn, to_speak)
        })
        .1 // second item is our final spoken number
}
