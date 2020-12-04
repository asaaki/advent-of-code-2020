use crate::structs::*;
use crate::utils::*;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub(crate) fn run_test(step: Step, input: Vec<String>, expected: String) -> NullResult {
    let actual = run(step, input)?;
    assert_eq!(actual, expected);
    Ok(())
}

// there's probably a crate out there where we could add fancy field validations directly;
// I should search for one just for testing it
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>, // the only optional field!
}

pub(crate) fn run(step: Step, input: Vec<String>) -> CustomResult<String> {
    // clean up input data:
    // 0. re-glue vec of lines into one string blob
    // 1. separate on empty lines
    // 2. merge record data into one string
    // 3. reorganize record data to be valid YAML
    // Sadly this requires some String allocations on the way. :cry:
    let records: Vec<String> = input
        .join("\n")
        .split("\n\n")
        .map(|r| r.split("\n").collect::<Vec<&str>>().join(" "))
        .map(|r| {
            r.split(" ")
                .map(|l| {
                    let items = l.split(":").collect::<Vec<&str>>();
                    let v = if items[1].starts_with('#') {
                        format!("'{}'", items[1])
                    } else {
                        items[1].to_owned()
                    };
                    format!("{}: {}", items[0], v)
                })
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect();

        let passports: Vec<Option<Passport>> = records
        .iter()
        .map(|r| serde_yaml::from_str(r).ok())
        .collect();

    match step {
        Step::One => {
            let step1_valids = passports.iter().filter(|p| p.is_some()).count();
            let result: String = format!("{}", step1_valids);
            println!("Inputs: passports={}", passports.len());
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            let step2_valids = passports
                .iter()
                .filter(|op| match op {
                    Some(p) => step2_validate(p),
                    None => false,
                })
                .count();
            let result: String = format!("{}", step2_valids);
            println!("Inputs: passports={}", passports.len());
            println!("Result = {}", result);
            Ok(result)
        }
    }
}

// no short circuits here yet, could be improved;
// if so, then reorder checks based on performance, so REs last
fn step2_validate(p: &Passport) -> bool {
    // could be its own function, but it was one of the easier checks
    let valid_years =
           p.byr >= 1920
        && p.byr <= 2002
        && p.iyr >= 2010
        && p.iyr <= 2020
        && p.eyr >= 2020
        && p.eyr <= 2030;
    let valid_height = validate_height(&p.hgt);
    let valid_hair = validate_hex(&p.hcl);
    let valid_eyes = validate_eye_color(&p.ecl);
    let valid_pid = validate_pid(&p.pid);

    valid_years && valid_height && valid_hair && valid_eyes && valid_pid
}

static RE_HEIGHT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^((?P<cm>\d{3})cm|(?P<in>\d{2})in)$").unwrap());

// there is probably a nicer way to handle this,
// but I also kinda like this 2 phase approach,
// just not very FP :shrug:
fn validate_height(input: &str) -> bool {
    let mut valid_height = false;

    if let Some(caps) = RE_HEIGHT.captures(input) {
        if let Some(m) = caps.name("cm") {
            let v: u8 = m.as_str().parse().expect("`cm` to be a number");
            valid_height = (v >= 150) && (v <= 193)
        }
        if let Some(m) = caps.name("in") {
            let v: u8 = m.as_str().parse().expect("`in` to be a number");
            valid_height = (v >= 59) && (v <= 76)
        }
    };

    valid_height
}

static RE_HEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[0-9a-f]{6}$").unwrap());

// why this extra length check?
// so we can bail out early and avoid running REs if the exact length requirement is not met anyway
fn validate_hex(input: &str) -> bool {
    input.len() == 7 && RE_HEX.find(input).is_some()
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn validate_eye_color(input: &str) -> bool {
    EYE_COLORS.iter().any(|&e| e.contains(input))
}

static RE_PID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]{9}$").unwrap());

// see above (validate_hex)
fn validate_pid(input: &str) -> bool {
    input.len() == 9 && RE_PID.find(input).is_some()
}
