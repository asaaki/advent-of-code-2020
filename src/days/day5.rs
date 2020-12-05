use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: Vec<String>, expected: String) -> NullResult {
    let actual = run(step, input)?;
    assert_eq!(actual, expected);
    Ok(())
}

// This struct mostly helps during debugging,
// but also keeping score of the row was valuable for step 2
#[derive(Clone, Debug)]
struct Seat<'c> {
    code: &'c str, // informational
    row: u8,       // good for step 2 (filtering)
    col: u8,       // informational
    id: usize,     // required (could be also a u16)
}

pub(crate) fn run(step: Step, input: Vec<String>) -> CustomResult<String> {
    let mut seats: Vec<Seat> = input
        .iter()
        .map(|code| {
            let (rcode, ccode) = code.split_at(7);
            // we only need the high (1) bit markers
            let row = parse_code(&rcode, 'B');
            let col = parse_code(&ccode, 'R');
            // we could also have parsed the whole string as one and avoid the
            // post-calculation of the ID, but hey, we gained extra info instead
            let id = (row as usize) * 8 + (col as usize);

            Seat { code, row, col, id }
        })
        .collect();
    // this allocates for sorting, but provides us extreme convenience in step 2
    seats.sort_by_cached_key(|s| s.id);

    match step {
        Step::One => {
            let result: String = format!("{}", seats.pop().unwrap().id);
            println!("Inputs: seats={}", seats.len());
            println!("Result = {}", result);
            Ok(result)
        }

        // To encourage to code the filtering instead of just guessing, the
        // inputs will include a gap in the first and/or last row;
        // sure, you could still try all results, but they also have a cooldown
        // on the form.
        Step::Two => {
            // so row tracking was a good idea ;-)
            let (first, last) = (seats.first().unwrap().row, seats.last().unwrap().row);
            let filtered: Vec<Seat> = seats
                .iter()
                .filter(|s| s.row > first && s.row < last)
                .map(|s| s.clone())
                .collect();

            // .windows(...) makes it so easy, otherwise we would have to either
            // use indices or .peek(), which would be less convenient to use.
            let my_seat_id = filtered.windows(2).fold(0usize, |id, pairs| {
                // gap check
                if pairs[0].id + 2 == pairs[1].id {
                    // our result;
                    // due to filtering only a single value will be in the set
                    pairs[0].id + 1
                } else {
                    id
                }
            });

            let result = format!("{}", my_seat_id);
            println!("Inputs: seats={}", filtered.len());
            println!("Result = {}", result);
            Ok(result)
        }
    }
}

/*
Takes a string and uses the `high` marker to calculate a number,
it is a bit pattern. High bits are marked with B and R.

Example code: FBFBBFFRLR

              row col
    CODE  FBFBBFF RLR
     BIN  0101100 101
     DEC       44   5

Notes:
- Instead of using the input length, we could also reverse the input,
  but that would allocate unnecessarily.
  Why do we need to do that? Because match_indices counts from left, but we need
  the low number indices starting from the right side.
- `as u8` is okay in our case, the requirements do not allow values bigger
  than 127, so we can safely truncate the usizes in the end.
  We could also make all our struct fields usize instead. :shrug:
*/
fn parse_code(code: &str, high: char) -> u8 {
    code.match_indices(high)
        .map(|(pos, _)| (1 << ((code.len() - 1) - pos) as u8))
        .fold(0, |acc, i| acc + i)
}
