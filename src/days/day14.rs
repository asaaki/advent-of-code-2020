use crate::structs::*;
use crate::utils::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

static MEM_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^mem\[(?P<index>\d+)\] = (?P<value>\d+)$").unwrap());

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let now = std::time::Instant::now();

    let mut memory = HashMap::new();
    process(input, &mut memory, step);
    let result: u64 = memory.values().sum();

    let elapsed = now.elapsed();
    let result: String = format!("{}", result);
    println!("Result = {}", result);
    println!(
        "[run] step took: {}ms ({}us)",
        elapsed.as_millis(),
        elapsed.as_micros()
    );
    Ok(result)
}

fn process(input: &Vec<String>, memory: &mut HashMap<u64, u64>, step: Step) {
    let mut masks = (0, 0, 0);
    for line in input {
        if line.starts_with("mask") {
            masks = compute_masks(line.split_at(7).1);
        } else {
            let captures = MEM_RE.captures(line).expect("invalid mem line");
            let address: u64 = captures
                .name("index")
                .expect("index missing")
                .as_str()
                .parse()
                .expect("to be a number");
            let value: u64 = captures
                .name("value")
                .expect("value missing")
                .as_str()
                .parse()
                .expect("to be a number");

            match step {
                Step::One => {
                    memory.insert(address, apply_masks(value, masks.0, masks.1));
                }
                Step::Two => {
                    // adjust address with high bit mask
                    let base_addr = address | masks.0;
                    // recurse through float mask
                    update_memory(memory, base_addr, value, masks.2);
                }
            };
        }
    }
}

// there is probably a better way than recursion, but it will do for now
fn update_memory(memory: &mut HashMap<u64, u64>, addr: u64, value: u64, float_mask: u64) {
    // we processed all the float bits in the mask and can write the address
    if float_mask == 0 {
        memory.insert(addr, value);

    // float mask has bits left to be processed
    } else {
        // move to the next float bit and use only that single bit
        let float_bit: u64 = 1 << float_mask.trailing_zeros();
        // branch: unaltered address (float bit matches address bit)
        update_memory(memory, addr, value, float_mask & !float_bit);
        // branch: float bit affects address and swaps the value
        update_memory(memory, addr ^ float_bit, value, float_mask & !float_bit);
    }
}

fn compute_masks(input: &str) -> (u64, u64, u64) {
    input.chars().rev().enumerate().fold(
        (0u64, 0u64, 0u64),
        |(mut high, mut low, mut float), (i, c)| {
            match c {
                '1' => high |= 1 << i,
                '0' => low |= 1 << i,
                'X' => float |= 1 << i,
                _ => (),
            };
            (high, low, float)
        },
    )
}

fn apply_masks(input: u64, high: u64, low: u64) -> u64 {
    // yes, parentheses are very important here!
    (input | high) & !low
}
