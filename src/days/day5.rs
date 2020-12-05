use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: Vec<String>, expected: String) -> NullResult {
    let actual = run(step, input)?;
    assert_eq!(actual, expected);
    Ok(())
}

pub(crate) fn run(step: Step, input: Vec<String>) -> CustomResult<String> {
    let mut seat_ids: Vec<u16> = input.iter().map(|code| parse_code(code)).collect();
    // we can use (faster) unstable sorting, because we have only unique IDs
    seat_ids.sort_unstable();

    match step {
        Step::One => {
            let result: String = format!("{}", seat_ids.pop().unwrap());
            println!("Inputs: seats={}", seat_ids.len());
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            // .windows(...) makes it so easy, otherwise we would have to either
            // use indices or .peek(), which would be less convenient to use.
            let my_seat_id = seat_ids.windows(2).fold(0u16, |id, pairs| {
                // gap check
                if pairs[0] + 2 == pairs[1] {
                    // our result;
                    // only a single value will be in the set
                    pairs[0] + 1
                } else {
                    id
                }
            });

            let result = format!("{}", my_seat_id);
            println!("Inputs: seats={}", seat_ids.len());
            println!("Result = {}", result);
            Ok(result)
        }
    }
}

// We only need the high (1) bits
const BIT_MARKERS: [char; 2] = ['B', 'R'];

/*
Takes a string and uses the `high` marker to calculate a number,
it is a bit pattern. High bits are marked with B and R.

Example code: FBFBBFFRLR, ID: 357

              row col    complete
    CODE  FBFBBFF RLR  FBFBBFFRLR
     BIN  0101100 101  0101100101
     DEC       44   5         357

Notes:
- Instead of using the input length, we could also reverse the input,
  but that would allocate unnecessarily.
  Why do we need to do that? Because match_indices counts from left, but we need
  the low number indices starting from the right side.
- `as u16` is okay in our case, the requirements do not allow values bigger
  than 1023 (10 bits), so we can safely truncate the usizes in the end.
*/
#[inline]
fn parse_code(code: &str) -> u16 {
    // TIL: this is how to use "slice of chars"; was tricky to figure out
    code.match_indices(&BIT_MARKERS[..])
        .map(|(pos, _)| (1 << ((code.len() - 1) - pos) as u16))
        .fold(0, |acc, i| acc | i)
}
