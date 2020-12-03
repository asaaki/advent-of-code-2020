use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: Vec<String>, expected: String) -> NullResult {
    let actual = run(step, input)?;
    assert_eq!(actual, expected);
    Ok(())
}

// jumps: R3 D1
const R_STEPS_1: usize = 3;
const D_STEPS_1: usize = 1;
const TREE: &str = "#";

const STEPPINGS_2: [(usize,usize);5] = [(1,1),(3,1),(5,1),(7,1),(1,2)];

// used from task 2
const MAX_R_STEP: usize = 7;

pub(crate) fn run(step: Step, input: Vec<String>) -> CustomResult<String> {
    let height = input.len();
    let chunk_size = input[0].len();
    let lines_covered = chunk_size.div_euclid(MAX_R_STEP);
    let repeats_needed = height.div_euclid(lines_covered);
    let width = chunk_size * repeats_needed;
    // println!("H={} CHUNK={} W={}", height, chunk_size, width);

    // naive way: use memory instead of efficient "ring buffer/circular jumping"
    let map: Vec<String> = input.iter().map(|l| l.repeat(repeats_needed)).collect();

    match step {
        Step::One => {
            let trees_met = map_walker(&map, height, R_STEPS_1, D_STEPS_1);
            let result: String = format!("{}", trees_met);

            println!("Inputs: expanded map {}x{}", width, height);
            println!("Result = {}", result);
            Ok(result)
        },

        Step::Two => {
            let mut results: Vec<usize> = Vec::with_capacity(STEPPINGS_2.len());
            for (r,d) in &STEPPINGS_2 {
              let trees_met = map_walker(&map, height, *r, *d);
              results.push(trees_met);
            }
            let tree_product: usize = results.iter().product();
            let result: String = format!("{}", tree_product);

            println!("Inputs: expanded map {}x{}", width, height);
            println!("Result = {}", result);
            Ok(result)
        },
    }
}

fn map_walker(map: &Vec<String>, height: usize, rstep: usize, dstep: usize) -> usize {
  let mut x = 0usize;
  let mut y = 0usize;
  let mut trees_met = 0usize;

  while y < (height - 1) {
      x += rstep;
      y += dstep;
      let line = &map[y];
      let xpos = line.get(x..=x).expect("to be able to jump at x pos");
      if xpos.find(TREE).is_some() { trees_met += 1; }
  }

  return trees_met;
}
