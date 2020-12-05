use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: Vec<String>, expected: String) -> NullResult {
    let actual = run(step, input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[derive(Clone, Debug)]
struct Seat<'c> {
    code: &'c str,
    row: u8,
    col: u8,
    id: usize,
}

pub(crate) fn run(step: Step, input: Vec<String>) -> CustomResult<String> {
    let mut seats: Vec<Seat> = input
        .iter()
        .map(|code| {
            let (rcode, ccode) = code.split_at(7);
            let row = parse_code(&rcode, 'B');
            let col = parse_code(&ccode, 'R');
            let id = (row as usize) * 8 + (col as usize);
            Seat {
                code: code,
                row,
                col,
                id,
            }
        })
        .collect();
    seats.sort_by_key(|s| s.id);

    match step {
        Step::One => {
            let result: String = format!("{}", seats.pop().unwrap().id);
            println!("Inputs: seats={}", seats.len());
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            let (first, last) = (seats.first().unwrap().row, seats.last().unwrap().row);
            let filtered: Vec<Seat> = seats
                .iter()
                .filter(|s| s.row > first && s.row < last)
                .map(|s| s.clone())
                .collect();

            let my_seat_id = filtered.windows(2).fold(0usize, |id, pairs| {
                if pairs[0].id + 2 == pairs[1].id {
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

fn parse_code(code: &str, high: char) -> u8 {
    // we have to count the index number from the other end,
    // so we use the length of the code to determine the upper bound
    let len = code.len() - 1;
    code.match_indices(high)
        .map(|(pos, _)| (1 << (len - pos) as u8))
        .fold(0, |acc, i| acc + i)
}
