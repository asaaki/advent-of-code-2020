use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, input)?;
    assert_eq!(actual, expected);
    Ok(())
}

const ADD_RESULT: u64 = 2020;

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let numbers: Vec<u64> = input
        .into_iter()
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    match step {
        Step::One => {
            for (i, num) in numbers.iter().enumerate() {
                for num2 in &numbers[i..] {
                    if num + num2 == ADD_RESULT {
                        let result = format!("{}", num * num2);
                        println!("Inputs: {} and {}", num, num2);
                        println!("Result = {}", result);
                        return Ok(result);
                    }
                }
            }
        }

        Step::Two => {
            for (i, num) in numbers.iter().enumerate() {
                for (j, num2) in (&numbers[i..]).iter().enumerate() {
                    for num3 in &numbers[j..] {
                        if num + num2 + num3 == ADD_RESULT {
                            let result = format!("{}", num * num2 * num3);
                            println!("Inputs: {}, {}, {}", num, num2, num3);
                            println!("Result = {}", result);
                            return Ok(result);
                        }
                    }
                }
            }
        }
    }

    Err(CustomError("no matching pair found".into()))
}
