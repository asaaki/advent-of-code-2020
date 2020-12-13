use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[derive(Debug)]
enum Action {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}
use Action::*;
type Actions = Vec<Action>;

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let actions: Actions = input.iter().map(|l| parse_action(l)).collect();

    match step {
        Step::One => {
            // (x,y), face/direction in degrees
            let state = ((0isize, 0isize), 90isize);
            let ((x, y), _face) =
                actions
                    .iter()
                    .fold(state, |((x, y), face), action| match action {
                        N(v) => ((x, y + v), face),
                        S(v) => ((x, y - v), face),
                        E(v) => ((x + v, y), face),
                        W(v) => ((x - v, y), face),
                        L(v) => ((x, y), turn(&face, &(-v))),
                        R(v) => ((x, y), turn(&face, v)),
                        F(v) => (move_ship(x, y, v, &face), face),
                    });
            let result: String = format!("{}", x.abs() + y.abs());
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            let state = ((0isize, 0isize), (10isize, 1isize));

            let ((x, y), _wp) =
                actions
                    .iter()
                    .fold(state, |((x, y), (wx, wy)), action| match action {
                        N(v) => ((x, y), (wx, wy + v)),
                        S(v) => ((x, y), (wx, wy - v)),
                        E(v) => ((x, y), (wx + v, wy)),
                        W(v) => ((x, y), (wx - v, wy)),
                        L(v) => ((x, y), turn_wp(wx, wy, &(-v))),
                        R(v) => ((x, y), turn_wp(wx, wy, v)),
                        F(v) => (move_to_wp(x, y, v, wx, wy), (wx, wy)),
                    });
            let result: String = format!("{}", x.abs() + y.abs());
            println!("Result = {}", result);
            Ok(result)
        }
    }
}

fn parse_action(s: &str) -> Action {
    let (a, n_str) = s.split_at(1);
    let n: isize = n_str.parse().expect("argument to be a number");
    match a {
        "N" => N(n),
        "S" => S(n),
        "E" => E(n),
        "W" => W(n),
        "L" => L(n),
        "R" => R(n),
        "F" => F(n),
        _ => panic!("invalid action found"),
    }
}

fn turn(face: &isize, degree: &isize) -> isize {
    (face + degree) % 360
}

fn move_ship(x: isize, y: isize, amount: &isize, face: &isize) -> (isize, isize) {
    match face {
        0 => (x, y + amount),
        90 => (x + amount, y),
        180 => (x, y - amount),
        270 => (x - amount, y),
        _ => (x, y),
    }
}

fn turn_wp(wx: isize, wy: isize, degree: &isize) -> (isize, isize) {
    match degree {
        90 | -270 => (wy, -wx),
        180 | -180 => (-wx, -wy),
        270 | -90 => (-wy, wx),
        _ => (wx, wy),
    }
}

fn move_to_wp(x: isize, y: isize, factor: &isize, wx: isize, wy: isize) -> (isize, isize) {
    (x + (factor * wx), y + (factor * wy))
}
