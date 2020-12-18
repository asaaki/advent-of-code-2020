use crate::structs::*;
use crate::utils::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

type Coord3 = (isize, isize, isize);
type Coord4 = (isize, isize, isize, isize);
type Space = Vec<Vec<Vec<Vec<usize>>>>;

static ADJACENT_NEIGHBOURS_3D: Lazy<Vec<Coord3>> = Lazy::new(|| {
    let mut n: Vec<Coord3> = Vec::with_capacity(26);
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if (x, y, z) == (0, 0, 0) {
                    continue;
                }
                n.push((x, y, z));
            }
        }
    }
    n
});
static ADJACENT_NEIGHBOURS_4D: Lazy<Vec<Coord4>> = Lazy::new(|| {
    let mut n: Vec<Coord4> = Vec::with_capacity(80);
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if (x, y, z, w) == (0, 0, 0, 0) {
                        continue;
                    }
                    n.push((x, y, z, w));
                }
            }
        }
    }
    n
});

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let steps = 6;
    let size = input.len() + 2 * (steps + 1);

    let mut space: Space = vec![vec![vec![vec![0; size]; size]; size]; size];

    for (y, line) in input.iter().enumerate() {
        for (x, state) in line.chars().enumerate() {
            if state == '#' {
                space[x + steps + 1][y + steps + 1][steps + 1][steps + 1] = 1;
            }
        }
    }

    match step {
        Step::One => {
            let now = std::time::Instant::now();

            iterate(&mut space, steps, false);
            let sum = space_sum(&space);

            let elapsed = now.elapsed();
            let result: String = format!("{}", sum);
            println!("Result = {}", result);
            println!(
                "[run] step took: {}ms ({}us)",
                elapsed.as_millis(),
                elapsed.as_micros()
            );
            Ok(result)
        }
        Step::Two => {
            let now = std::time::Instant::now();

            iterate(&mut space, steps, true);
            let sum = space_sum(&space);

            let elapsed = now.elapsed();
            let result: String = format!("{}", sum);
            println!("Result = {}", result);
            println!(
                "[run] step took: {}ms ({}us)",
                elapsed.as_millis(),
                elapsed.as_micros()
            );
            Ok(result)
        }
    }
}

fn space_sum(space: &Space) -> usize {
    space
        .iter()
        .flat_map(|x| x.iter().flat_map(|y| y.iter().flatten()))
        .sum::<usize>()
}

fn iterate(space: &mut Space, steps: usize, four_d: bool) {
    for _i in 0..steps {
        let range = 1..(space.len() - 1);
        let w_range = if four_d {
            1..(space.len() - 1)
        } else {
            (steps + 1)..(steps + 2)
        };

        let (sender, receiver) = std::sync::mpsc::channel();

        range.clone().into_par_iter().for_each_with(sender, |s, x| {
            range.clone().into_iter().for_each(|y| {
                range.clone().into_iter().for_each(|z| {
                    w_range.clone().into_iter().for_each(|w| {
                        let (ix, iy, iz, iw) = (x as isize, y as isize, z as isize, w as isize);
                        let mut neighbours = 0u8;

                        if four_d {
                            for (nx, ny, nz, nw) in ADJACENT_NEIGHBOURS_4D.iter() {
                                let (dx, dy, dz, dw) = (ix + nx, iy + ny, iz + nz, iw + nw);
                                if space[dx as usize][dy as usize][dz as usize][dw as usize] == 1 {
                                    neighbours += 1;
                                }
                            }
                        } else {
                            for (nx, ny, nz) in ADJACENT_NEIGHBOURS_3D.iter() {
                                let (dx, dy, dz) = (ix + nx, iy + ny, iz + nz);
                                if space[dx as usize][dy as usize][dz as usize][w] == 1 {
                                    neighbours += 1;
                                }
                            }
                        }

                        match space[x][y][z][w] {
                            0 => {
                                if neighbours == 3 {
                                    s.send((ix, iy, iz, iw)).unwrap();
                                }
                            }
                            1 => {
                                if !(2..=3).contains(&neighbours) {
                                    s.send((ix, iy, iz, iw)).unwrap();
                                }
                            }
                            _ => panic!("impossible value"),
                        }
                    })
                })
            })
        });

        for (x, y, z, w) in receiver.iter() {
            space[x as usize][y as usize][z as usize][w as usize] ^= 1;
        }
    }
}
