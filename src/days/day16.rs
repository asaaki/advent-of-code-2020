use crate::structs::*;
use crate::utils::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
// use rayon::prelude::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

static RANGE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<name>[^:]+): (?P<r1a>\d+)-(?P<r1b>\d+) or (?P<r2a>\d+)-(?P<r2b>\d+)$")
        .unwrap()
});
static MT_MARKER: &str = "your ticket:";
static NBT_MARKER: &str = "nearby tickets:";

// type RangeMap = HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>;
type SubRange = RangeInclusive<usize>;
type FieldRanges = Vec<SubRange>;
type RangeMap = HashMap<String, FieldRanges>;
type Ticket = Vec<usize>;

fn parse_input(input: &Vec<String>) -> (RangeMap, Ticket, Vec<Ticket>) {
    let mut mt_break = 0usize;
    let mut nbt_break = 0usize;

    // pass 1: where are the sections
    for (idx, line) in input.iter().enumerate() {
        if line.starts_with(&MT_MARKER) {
            mt_break = idx;
        }
        if line.starts_with(&NBT_MARKER) {
            nbt_break = idx;
        }
    }

    // pass 2

    let ranges = &input[0..mt_break - 1];
    let my_ticket = &input[mt_break + 1];
    let tickets = &input[nbt_break + 1..];

    let mut range_map = RangeMap::new();

    for line in ranges {
        if let Some(captures) = RANGE_RE.captures(line) {
            let name = captures.name("name").unwrap().as_str();
            let r1a: usize = captures.name("r1a").unwrap().as_str().parse().unwrap();
            let r1b: usize = captures.name("r1b").unwrap().as_str().parse().unwrap();
            let r2a: usize = captures.name("r2a").unwrap().as_str().parse().unwrap();
            let r2b: usize = captures.name("r2b").unwrap().as_str().parse().unwrap();
            let r1 = r1a..=r1b;
            let r2 = r2a..=r2b;
            // range_map.insert(name.to_owned(), (r1, r2));
            range_map.insert(name.to_owned(), vec![r1, r2]);
        }
    }

    let my_ticket: Vec<_> = my_ticket
        .split(",")
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    let tickets: Vec<Vec<_>> = tickets
        .iter()
        .map(|t| {
            t.split(",")
                .map(str::parse::<usize>)
                .filter_map(Result::ok)
                .collect()
        })
        .collect();

    (range_map, my_ticket, tickets)
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let (rm, mt, nbt) = parse_input(input);

    match step {
        Step::One => {
            let now = std::time::Instant::now();

            let mut invalid_numbers = Vec::<usize>::new();
            let flat_ranges = rm
                .values()
                .flat_map(|v| v.into_iter())
                .collect::<Vec<&RangeInclusive<usize>>>();
            for ticket in nbt {
                for number in &ticket {
                    let mut covered = false;
                    'ranges: for range in &flat_ranges {
                        if range.contains(number) {
                            covered = true;
                            break 'ranges;
                        }
                    }
                    if !covered {
                        invalid_numbers.push(*number);
                    }
                }
            }
            let result = invalid_numbers.iter().sum::<usize>();

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
        Step::Two => {
            let now = std::time::Instant::now();

            let valid_tickets = nbt
                .iter()
                .filter(|&t| {
                    t.iter().all(|f| {
                        rm.values()
                            .any(|rr| rr[0].contains(&f) || rr[1].contains(&f))
                    })
                })
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<Ticket>>();

            let mut found: Vec<Option<String>> = vec![None; mt.len()];

            while !found.iter().all(Option::is_some) {
                for pos in 0..mt.len() {
                    if found[pos].is_some() {
                        continue;
                    }

                    let mut field_candidates: Vec<Vec<String>> = vec![vec![]; mt.len()];

                    for (name, rr) in &rm {
                        let is_match = valid_tickets
                            .iter()
                            .map(|t| t[pos])
                            .all(|t| rr[0].contains(&t) || rr[1].contains(&t));
                        if is_match {
                            field_candidates[pos].push(name.to_owned());
                        }
                    }
                    for (i, candidates) in field_candidates.iter().enumerate() {
                        if candidates.len() == 0 {
                            continue;
                        }
                        let reduced: Vec<&String> = candidates
                            .iter()
                            .filter(|&n| !found.contains(&Some(n.clone())))
                            .collect();
                        if reduced.len() == 1 {
                            found[i] = Some(reduced[0].to_owned());
                        }
                    }
                }
            }

            let departure_indices: Vec<usize> = found
                .iter()
                .enumerate()
                .filter_map(|(idx, name)| {
                    name.as_ref()
                        .filter(|n| n.starts_with("departure"))
                        .and_then(|_| Some(idx))
                })
                .collect();

            let product = departure_indices.iter().fold(1usize, |p, i| p * mt[*i]);

            let elapsed = now.elapsed();
            let result: String = format!("{}", product);
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
