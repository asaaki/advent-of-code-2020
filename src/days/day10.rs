use crate::structs::*;
use crate::utils::*;
use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    // first is the outlet with 0
    let mut joltages = vec![0];
    let mut adapters: Vec<usize> = input.iter().map(|s| s.parse().unwrap()).collect();
    adapters.sort_unstable();
    joltages.append(&mut adapters);
    // our device (+3 of highest)
    joltages.push(joltages.last().unwrap() + 3);

    match step {
        Step::One => {
            let mut diffs: Vec<usize> = vec![0; 4]; // 0,0,0,0
            joltages.windows(2).fold(&mut diffs, |acc, pair| {
                acc[pair[1] - pair[0]] += 1;
                acc
            });
            let result: String = format!("{}", diffs[1] * diffs[3]);
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            let now = std::time::Instant::now();
            let len = joltages.len();
            let mut g = vec![vec![0usize; len]; len];

            // build the input graph for count_options function
            for (row, rv) in joltages.iter().enumerate() {
                for (col, cv) in joltages[row + 1..].iter().enumerate() {
                    if cv - rv > 0 && cv - rv <= 3 {
                        g[row][row + col + 1] = 1;
                    }
                }
            }

            // rayon's parallel iterators improves the processing time significantly
            let count = (0..len)
                .into_par_iter()
                .map(|edges| count_options(&g, 0, len - 1, edges, len))
                .reduce(|| 0usize, |total, part| total + part);

            let elapsed = now.elapsed();
            let result: String = format!("{}", count);
            println!("Result = {}", result);
            println!("[run] step took: {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
            Ok(result)
        }
    }
}

// Rustified version of
// https://www.geeksforgeeks.org/count-possible-paths-source-destination-exactly-k-edges/
fn count_options(
    g: &Vec<Vec<usize>>,
    from: usize,
    to: usize,
    edges: usize,
    vertices: usize,
) -> usize {
    let mut counts = vec![vec![vec![0usize; edges + 1]; vertices]; vertices];

    for e in 0..=edges {
        for i in 0..vertices {
            for j in 0..vertices {
                if e == 0 && i == j {
                    counts[i][j][e] = 1;
                }
                if e == 1 && g[i][j] != 0 {
                    counts[i][j][e] = 1;
                }

                if e > 1 {
                    for a in 0..vertices {
                        if g[i][a] != 0 {
                            counts[i][j][e] += counts[a][j][e - 1];
                        }
                    }
                }
            }
        }
    }

    counts[from][to][edges]
}
