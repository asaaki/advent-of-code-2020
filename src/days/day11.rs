use crate::structs::*;
use crate::utils::*;
// use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

// game of life but with map constraints:
// only seat fields can be used,
/* seating rules
If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
*/

#[derive(Debug, Clone)]
enum Cell {
    Floor,
    Seat(bool),
}
use Cell::*;

type CellLine = Vec<Cell>;
type Field = Vec<CellLine>;
type Swaps = Vec<(usize, usize)>;

// the board
#[derive(Debug, Clone)]
struct Area {
    cols: usize,
    rows: usize,
    field: Field,
    round: usize,
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let mut area = Area::new(input);
    area.run_until_stable(&step);
    eprintln!(
        "size: {}x{}, rounds needed: {}",
        area.cols, area.rows, area.round
    );
    let result: String = format!("{}", area.occupied());
    println!("Result = {}", result);
    Ok(result)
}

impl Area {
    fn new(input: &Vec<String>) -> Self {
        let cols = input.len();
        let rows = input[0].len();
        let field = input
            .iter()
            .map(|col| {
                col.chars()
                    .map(|c| match c {
                        '.' => Floor,
                        'L' => Seat(false),
                        _ => panic!("invalid cell type detected; c={}", c),
                    })
                    .collect::<CellLine>()
            })
            .collect::<Field>();

        Area {
            cols,
            rows,
            field,
            round: 0,
        }
    }

    // a game tick
    fn next(&mut self, step: &Step) -> bool {
        let swaps = swap_collect(&self.field, step);

        for (x, y) in &swaps {
            // I could have used unsafe .get_unchecked_mut, but I'd like to
            // avoid unsafe for now (just know I was aware of the possibility).
            // Yes, bounds checks are a little bit more costly.
            let cell = (&mut self.field)
                .get_mut(*y)
                .and_then(|r| r.get_mut(*x))
                .expect("cell should exist");

            let new_cell = match cell {
                Seat(false) => Some(Seat(true)),
                Seat(true) => Some(Seat(false)),
                _ => None,
            };
            if let Some(c) = new_cell {
                let _ = std::mem::replace(cell, c);
            }
        }

        self.round += 1;
        swaps.len() > 0
    }

    fn run_until_stable(&mut self, step: &Step) {
        while self.next(step) {}
    }

    // step 1 question
    fn occupied(&self) -> usize {
        self.field
            .iter()
            .map(|cells| {
                cells
                    .iter()
                    .map(|cell| match cell {
                        Seat(true) => 1,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

// phase 1: check for swaps
fn swap_collect(field: &Field, step: &Step) -> Swaps {
    let mut swaps: Swaps = Vec::new();
    for (y, col) in field.iter().enumerate() {
        for (x, cell) in col.iter().enumerate() {
            let ncount = match step {
                Step::One => swap_check_step_1(field, x, y),
                Step::Two => swap_check_step_2(field, x, y),
            };

            // in part 2 deoccupation happens at 5
            let max = if step == &Step::One { 4 } else { 5 };
            match (cell, ncount == 0, ncount >= max) {
                (Seat(false), true, false) => swaps.push((x, y)),
                (Seat(true), false, true) => swaps.push((x, y)),
                _ => (),
            }
        }
    }
    swaps.sort_unstable();
    swaps
}

// can also act as stepping info for part2
const ADJACENT_NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    /* 0, 0 */
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn swap_check_step_1(field: &Field, x: usize, y: usize) -> usize {
    let mut ncount = 0usize;

    // with 2 for-loops:
    // advantage is that we do not need to typecast
    // for nx in (x.saturating_sub(1))..=(x.saturating_add(1)) {
    //     for ny in (y.saturating_sub(1))..=(y.saturating_add(1)) {
    //         if nx == x && ny == y {
    //             continue;
    //         }
    //         let neighbor = field.get(ny).and_then(|r| r.get(nx));
    //         if let Some(Seat(true)) = neighbor {
    //             ncount += 1;
    //         }
    //     }
    // }

    // with offset matrix:
    // advantage is that we only need to loop once
    for (dx, dy) in &ADJACENT_NEIGHBOURS {
        let nx = (x as isize) + dx;
        let ny = (y as isize) + dy;
        if nx < 0 && ny < 0 {
            continue;
        }
        let neighbor = field.get(ny as usize).and_then(|r| r.get(nx as usize));
        if let Some(Seat(true)) = neighbor {
            ncount += 1;
        }
    }

    ncount
}

// could benefit from par_iters
fn swap_check_step_2(field: &Field, x: usize, y: usize) -> usize {
    let mut neighbours = Vec::new();
    // use those coordinates as steppings to walk in each direction
    for (sx, sy) in &ADJACENT_NEIGHBOURS {
        let (mut cx, mut cy) = (x as isize, y as isize);
        let mut search = true;
        while search {
            cx = cx.saturating_add(*sx);
            cy = cy.saturating_add(*sy);
            if cx < 0 || cy < 0 {
                break;
            }
            let c = field.get(cy as usize).and_then(|r| r.get(cx as usize));
            if let Some(cell) = c {
                if let Seat(taken) = cell {
                    neighbours.push(taken.clone());
                    search = false;
                }
            } else {
                search = false;
            }
        }
    }
    neighbours
        .iter()
        .fold(0, |count, &taken| if taken { count + 1 } else { count })
}
