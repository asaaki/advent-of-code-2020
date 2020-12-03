use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: Vec<String>, expected: String) -> NullResult {
    let actual = run(step, input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

pub(crate) fn run(step: Step, input: Vec<String>) -> CustomResult<String> {
    let database: Vec<(Policy, String)> = input
        .into_iter()
        .map(|e| {
            let parts: Vec<&str> = e.split(": ").collect();
            let password = parts[1].to_owned();
            let raw_policy = parts[0];
            let policy_parts: Vec<&str> = raw_policy.split(" ").collect();
            let range_parts: Vec<&str> = policy_parts[0].split("-").collect();
            let range_start = range_parts[0].parse().expect("to be a number");
            let range_end = range_parts[1].parse().expect("to be a number");
            let policy_char = &policy_parts[1]
                .chars()
                .nth(0)
                .expect("raw policy to have exactly 1 char");

            let policy = Policy {
                min: range_start,
                max: range_end,
                letter: policy_char.to_owned(),
            };

            (policy, password)
        })
        .collect();

    match step {
        Step::One => {
            let valid_passwords: Vec<bool> = database
                .iter()
                .map(|(policy, pw)| {
                    let has_letter = pw.contains(policy.letter);
                    has_letter && {
                        let count = pw.chars().filter(|c| *c == policy.letter).count();
                        count <= policy.max && count >= policy.min
                    }
                })
                .filter(|e| *e)
                .collect();

            let result: String = format!("{}", valid_passwords.len());

            println!("Inputs: {} entries", database.len());
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            let valid_passwords: Vec<bool> = database
                .iter()
                .map(|(policy, pw)| {
                    let matches: Vec<usize> = pw
                        .match_indices(policy.letter)
                        .map(|(i, _)| i + 1)
                        .collect();
                    matches.contains(&policy.min) ^ matches.contains(&policy.max)
                })
                .filter(|e| *e)
                .collect();

            let result: String = format!("{}", valid_passwords.len());

            println!("Inputs: {} entries", database.len());
            println!("Result = {}", result);
            Ok(result)
        }
    }
}
