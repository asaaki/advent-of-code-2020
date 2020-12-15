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
            let now = std::time::Instant::now();

            // biz logic here

            let elapsed = now.elapsed();
            let result: String = format!("{}", 0);
            println!("Result = {}", result);
            println!("[run] step took: {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
            Ok(result)
        },
        Step::Two => {
            let now = std::time::Instant::now();

            // biz logic here

            let elapsed = now.elapsed();
            let result: String = format!("{}", 0);
            println!("Result = {}", result);
            println!("[run] step took: {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
            Ok(result)
        }
    }
}
