use crate::structs::*;
use crate::utils::*;
// use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let earliest: usize = input[0].parse().expect("to be a number");

    match step {
        Step::One => {
            // "x" are filtered out
            let busses: Vec<_> = input[1]
                .split(",")
                // next 2 are basically: .filter_map(|s| s.parse().ok()),
                // but injecting the functions instead of building a closure
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect();

            let mut next_departures: Vec<(usize, usize)> = busses
                .iter()
                .map(|bus| {
                    let (mut round, rest) = divmod(&earliest, bus);
                    if rest > 0 {
                        round += 1
                    };
                    ((round * bus) - earliest, *bus)
                })
                .collect();
            next_departures.sort_unstable();

            let (w, b) = next_departures.first().expect("thou shall not be empty!");

            let result: String = format!("{}", w * b);
            println!("Result = {}", result);
            Ok(result)
        }
        Step::Two => {
            let now = std::time::Instant::now();
            // this is a condensed version of multiple steps of parsing and
            // iterating
            let ts = input[1]
                .split(",")
                // usize'able?
                // (will stick with usize, as it works nicely with enumerate)
                .map(str::parse::<usize>)
                // result-> option
                .map(Result::ok)
                // add indices
                .enumerate()
                // if bus id valid, map to tuple, rest is filtered
                .filter_map(|(offset, id)|
                    // options are .map'able
                    id.map(|id| (offset, id)))
                // heavy lifting
                // start with earliest time, puzzle hints that; so no reason to
                // start at 0; use stepping of 1 to kickstart finding a ts for
                // first bus
                .fold((earliest, 1), |(mut result, step), (offset, id)| {
                    // open/"infinite" range; the worst what could happen:
                    // reaching the maximum of the integer type used;
                    // the new result is the first ts matching the criteria;
                    // when we reach the last bus in the list, it will be also
                    // the final answer
                    result = (result..)
                        // step through ts with last given stepping
                        .step_by(step)
                        // find a time working with the offset
                        .find(|t| (t + offset) % id == 0)
                        // okay, Rust will panic first when it reaches
                        // {int type}::MAX, there wouldn't be a possible
                        // solution to the .find(…) as well,
                        // but unwrapping would not happen before the panic;
                        // this is a lot of text to explain, why an .unwrap()
                        // would be totally fine here :shrug:
                        .expect("this message cannot happen, we are INFINITY");

                    // return result and new stepping for next bus
                    // stepping: product of all previous busses
                    //
                    // why does it work?
                    // with zero offsets for all busses, they can only meet when
                    // all their modulos are zero, which only happens if the ts
                    // is divisible by the product of all bus IDs;
                    // the stepping is independent from the individual offsets
                    (result, step * id)
                })
                .0;

            let elapsed = now.elapsed();
            println!("[run] step took: {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
            let result: String = format!("{}", ts);
            println!("Result = {}", result);
            Ok(result)
        }
    }
}

// tiny helper to get both the quotient and remainder;
// why is that not part of the stdlib? It's sooooo useful!
fn divmod(a: &usize, b: &usize) -> (usize, usize) {
    (a / b, a % b)
}
