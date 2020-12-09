use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let numbers: Vec<usize> = input.iter().map(|s| s.parse().unwrap()).collect();
    // I added the window as the first value, so both the test and input data
    // can be used without hardcoding special logic for them
    let (window, cipher) = numbers.split_first().expect("number list is empty");

    // part 1: find the one outlier in the list
    let mut invalid = 0usize;
    // let's use .windows again, as it's made for such a use case;
    // needs to be one item longer (the target value)
    for w in cipher.windows(window + 1) {
        // separate the number to check and the pool of terms
        let (current, pool) = w.split_last().unwrap();

        let mut matched = false;

        // use our nested for looping in same way as in day 1, so we use each
        // term only once
        for (idx, a) in pool.iter().enumerate() {
            for b in &pool[idx..] {
                // why all those asterisks? Rust doesn't auto-derefs &usize
                // for math operations apparently; sad, but it's okay I guess;
                // if there are 2 numbers adding up to target, then we found a
                // match, and therefore not our invalid number
                if *current == (*a + *b) {
                    matched = true;
                }
            }
        }
        // if the previous looping did **not** find a result, then that is our
        // solution!
        // we can now break out of our outer loop, only one result needed
        if !matched {
            invalid = *current;
            break;
        }
    }

    match step {
        Step::One => {
            let result: String = format!("{}", invalid);
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            // pseudo efficancy: use a shorter slice for search;
            // it also works without this sub slicing
            let iidx = cipher.iter().position(|&e| e == invalid).unwrap();
            let cipher = &cipher[..iidx];

            // collection for the potential solution
            let mut summands: Vec<usize> = Vec::new();

            // walk through the slice and search for all terms accumulating to
            // the number in question
            for (i, x) in cipher.iter().enumerate() {
                // temporary collection, different word, but same meaning
                let mut addends: Vec<usize> = vec![*x];
                // let's iteratively substract until we get either exactly zero,
                // or negative value (so we overshot and did not succeed)
                let res = cipher[i..].iter().fold(invalid as isize, |acc, b| {
                    if acc == 0 {
                        return 0;
                    }
                    let rest = acc - (*b as isize);
                    if rest >= 0 {
                        addends.push(*b);
                    };
                    rest
                });
                // leave if we found our solution, we only need a first match
                if res == 0 {
                    summands = addends;
                    break;
                }
            }
            // sort, so we can easily grab the smallest and biggest value
            summands.sort();
            let small = summands.first().unwrap();
            let big = summands.last().unwrap();
            // the sum of both terms is our solution
            let result: String = format!("{}", small + big);
            println!("Result = {}", result);
            Ok(result)
        }
    }
}
