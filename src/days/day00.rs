use crate::structs::*;
use crate::utils::*;
// use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    eprintln!("input={:?}", input);
    match step {
        Step::One => {
            let result: String = format!("{}", 0);
            println!("Result = {}", result);
            Ok(result)
        },
        Step::Two => {
            let result: String = format!("{}", 0);
            println!("Result = {}", result);
            Ok(result)
        }
    }
}
