use crate::structs::*;
use crate::utils::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

// A contains (x amount of B)
type ContainsMap = HashMap<String, Vec<(usize, String)>>;
// A is contained in B
type ContainedMap = HashMap<String, Vec<String>>;
// counter helper
type ContainsSet = HashSet<String>;

// glowing, sparkling rainbow would have been preferred, but this will do, too
const MY_BAG: &str = "shiny gold";

// find container bag and its children
static RE_SOURCE: Lazy<Regex> = Lazy::new(||
    Regex::new(r"^(?P<source>\w+ \w+) bags?? contain (?P<children>[^.]+)*\.$").unwrap());
// find each child color
static RE_CHILDREN: Lazy<Regex> = Lazy::new(||
    Regex::new(r"(?P<count>\d+) (?P<child>\w+ \w+) bags??").unwrap());

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let (contains, contained) = create_maps(input);

    match step {
        Step::One => {
            let mut contains_mine: ContainsSet = ContainsSet::new();
            walk_contained(&contained, MY_BAG, &mut contains_mine);
            let result: String = format!("{}", contains_mine.len());
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            let result = color_cost(&contains, MY_BAG);
            let result: String = format!("{}", result);
            println!("Result = {}", result);
            Ok(result)
        }
    }
}

fn create_maps(input: &Vec<String>) -> (ContainsMap, ContainedMap) {
    let mut contains: ContainsMap = ContainsMap::new();
    let mut contained: ContainedMap = ContainedMap::new();

    for line in input {
        // we can skip empty container bags (REs are expensive enough already)
        if line.ends_with("no other bags.") { continue; }

        if let Some(caps) = RE_SOURCE.captures(line) {
            let source = caps.name("source").unwrap().as_str();

            if let Some(children) = caps.name("children") {
                for c in RE_CHILDREN.captures_iter(children.as_str()) {
                    let count: usize = c.name("count").unwrap().as_str().parse().unwrap();
                    let child = c.name("child").unwrap().as_str();

                    if contains.contains_key(source) {
                        let mut values = contains.get(source).unwrap().to_owned();
                        values.push((count, child.to_owned()));
                        contains.insert(source.to_owned(), values);
                    } else {
                        contains.insert(source.to_owned(), vec![(count, child.to_owned())]);
                    }

                    let mut v = vec![source.to_owned()];
                    if contained.contains_key(child) {
                        let mut v2 = contained.get(child).unwrap().to_owned();
                        v.append(&mut v2);
                    }
                    contained.insert(child.to_owned(), v);
                }
            }
        }
    }

    (contains, contained)
}

fn walk_contained(map: &ContainedMap, color: &str, out: &mut ContainsSet) {
    if let Some(containers) = map.get(color) {
        for c in containers {
            out.insert(c.to_owned());
            walk_contained(map, c, out)
        };
    }
}

/*
    So, a single shiny gold bag must contain
         1 dark olive bag (and the 7 bags within it)
    plus 2 vibrant plum bags (and the 11 bags within each of those):
    1 + 1*7 + 2 + 2*11 = 32 bags!
    (1 * (7 + 1)) + (2 * (11 + 1))
*/
fn color_cost(map: &ContainsMap, color: &str) -> usize {
    let mut total = 0usize;
    if let Some(children) = map.get(color) {
        for (num, child) in children {
            let nested = color_cost(map, child);
            total += num * (nested + 1);
        }
    }
    total
}
