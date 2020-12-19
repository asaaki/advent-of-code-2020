use crate::structs::*;
use crate::utils::*;
// use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[derive(Debug)]
enum Token {
    // inclusive range, otherwise the end index could overflow
    Int { start: usize, end: usize },
    Op { index: usize, op: MathOp },
    // non-inclusive range is fine, the terminating ')' is always within bounds,
    // but also carries no real meaning for the nested context OTOH
    Nest { start: usize, end: usize, parent: usize },
}
type Tokens = Vec<Token>;

#[derive(Debug, Clone, PartialEq)]
enum MathOp {
    Add, Mul,
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let now0 = std::time::Instant::now();
    let ops_lines: Vec<_> = input.iter().map(|l| (l, tokenize(l))).collect();
    let elapsed0 = now0.elapsed();

    let now = std::time::Instant::now();

    let result: usize = ops_lines.iter().map(|(l,tt)|calculate(l, &tt, &step)).sum();

    let elapsed = now.elapsed();
    let result: String = format!("{}", result);
    println!("Result = {}", result);
    println!(
        "[run] tokenization took: {}ms ({}us)",
        elapsed0.as_millis(),
        elapsed0.as_micros()
    );
    println!(
        "[run] calculation took: {}ms ({}us)",
        elapsed.as_millis(),
        elapsed.as_micros()
    );
    Ok(result)
}



fn tokenize(input: &String) -> Vec<Token> {
    use Token::*;
    use MathOp::*;

    let last_idx = input.len() - 1;

    let mut nesting = vec![];
    let mut digit_start: Option<usize> = None;

    let mut stack: Tokens = Vec::with_capacity(64);

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => {
                let current_stack_id = stack.len();
                nesting.push((current_stack_id, i));
            }
            ')' => {
                if let Some(idx) = digit_start {
                    stack.push(Int { start: idx, end: i - 1 });
                }
                digit_start = None;
                let nest_lvl = nesting.len() as usize;
                if let Some((stack_idx,idx)) = nesting.pop() {
                    stack.insert(stack_idx, Nest { start: idx, end: i, parent: nest_lvl - 1 });
                }
            }
            '+' => stack.push(Op { index: i, op: Add }),
            '*' => stack.push(Op { index: i, op: Mul }),
            d if d.is_ascii_digit() => {
                if let Some(idx) = digit_start {
                    // is this the last char, finalize the number
                    if i == last_idx {
                        stack.push(Int { start: idx, end: i - 1 });
                    }
                } else {
                    if digit_start.is_none() {
                        if i == last_idx {
                            stack.push(Int { start: i, end: i });
                        } else {
                            digit_start = Some(i);
                        }
                    }
                }
            },
            // spaces (+ exhaustiveness)
            _ => {
                if let Some(idx) = digit_start {
                    stack.push(Int { start: idx, end: i -1 });
                }
                digit_start = None;
            },
        }
    }

    stack
}

fn calculate(line: &str, tokens: &[Token], step: &Step) -> usize {
    match step {
        Step::One => { parse_and_calculate(line, tokens, 0, line.len()-1, false) }
        Step::Two => { parse_and_calculate(line, tokens, 0, line.len()-1, true) }
    }
}

fn parse_and_calculate(line: &str, tokens: &[Token], start_idx: usize, end_idx: usize, weak_mul: bool) -> usize {
    use Token::*;
    use MathOp::*;

    let mut mop: Option<MathOp> = None;
    let mut result = 0usize;
    let mut start_idx = start_idx;

    for (t_idx, token) in tokens.iter().enumerate() {
        match token {
            Int { start, end } => {
                if start < &start_idx || end > &end_idx { continue; }

                let int_slice = &line[*start..=*end];
                let value: usize = int_slice.parse().expect("not an int");
                if let Some(ref op) = mop {
                    match op {
                        Add => result += value,
                        Mul => result *= value,
                    }
                } else {
                    result = value;
                }
            },
            Op { index, op, .. } => {
                if index < &start_idx || index > &end_idx { continue; }

                if weak_mul && op == &Mul {
                    let right = parse_and_calculate(line, &tokens[t_idx+1 ..], index+2, end_idx, weak_mul);
                    return result * right;
                } else {
                    mop = Some(op.clone());
                }
            },
            Nest { start, end, .. } => {
                if start < &start_idx || end > &end_idx { continue; }

                let nest_result = parse_and_calculate(line, &tokens[t_idx+1 ..], start+1, *end-1, weak_mul);
                // move start index to after parsed nesting,
                // so we skip them in the parent context
                start_idx = *end;
                if let Some(ref op) = mop {
                    match op {
                        Add => result += nest_result,
                        Mul => result *= nest_result,
                    }
                } else {
                    result = nest_result;
                }
            },
        };
    }
    result
}
