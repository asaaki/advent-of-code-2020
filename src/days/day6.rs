use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

const A_IDX: u8 = 'a' as u8;

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    // a bit unhappy about the owning, but easier than fidlling with lifetimes
    let groups: Vec<String> = input
        .join("\n")
        .split("\n\n")
        .map(std::borrow::ToOwned::to_owned)
        .collect();

    match step {
        Step::One => {
            let records: Vec<char> = groups
                .iter()
                .map(|g| {
                    // collect all answers into a single stream of chars
                    let mut chars: Vec<char> = g.replace('\n', "").chars().collect();
                    // sort them for the next step; unstable is okay,
                    // because "order of equals" does not matter for chars anyway
                    chars.sort_unstable();
                    // remove duplicates (works only on consecutive occurrences, thus the sorting)
                    chars.dedup();
                    chars
                })
                .flatten()
                .collect();
            let result: String = format!("{}", records.len());
            println!("Inputs: groups={}", groups.len());
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            let records: Vec<usize> = groups
                .iter()
                .map(|g| {
                    let answers: Vec<&str> = g.split('\n').collect();
                    if answers.len() == 1 {
                        // single person answer's are much simpler to calculate
                        answers[0].len() as usize
                    } else {
                        // monstrosity of mapping, folding, bitshifting, bitwise-and-ing, and counting
                        answers
                            .iter()
                            .map(|&a| {
                                // step 1: store each person's answer as an u32
                                // (each letter is a bit, starting from zero, therefore the "minus 'a'")
                                a.chars()
                                    .fold(0u32, |acc, c| acc | 1 << ((c as u8) - A_IDX))
                            })
                            // step 2: bitwise-AND all answers, so only bits common to all of them remain
                            .fold(u32::MAX, |acc, b| acc & b)
                            // step 3: count the 1 bits --- Thank you, Rust, for having that in the stdlib!
                            .count_ones() as usize
                    }
                })
                .collect();
            let result: String = format!("{}", records.iter().sum::<usize>());
            println!("Inputs: groups={}", groups.len());
            println!("Result = {}", result);
            Ok(result)
        }
    }
}
