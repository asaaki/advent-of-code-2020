use crate::structs::*;
use crate::utils::*;

pub(crate) fn run_test(step: Step, input: &Vec<String>, expected: String) -> NullResult {
    let actual = run(step, &input)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}
use Instruction::*;

impl Instruction {
    fn from_tuple(tuple: &Vec<&str>) -> Self {
        let arg: isize = tuple[1].parse().expect("argument must be number");
        match tuple[0] {
            "nop" => Nop(arg),
            "acc" => Acc(arg),
            "jmp" => Jmp(arg),
            _ => panic!("invalid instruction found in code; input: {:?}", tuple),
        }
    }

    fn swap(self) -> Self {
        match self {
            Nop(v) => Jmp(v),
            Jmp(v) => Nop(v),
            _ => self,
        }
    }
}

type Code = Vec<Instruction>;

#[derive(Debug)]
struct Program {
    code: Code,

    pc: usize,         // PC: program counter
    counter: isize,    // accumulator value
    visits: Vec<bool>, // for loop detection
    terminated: bool,  // if program terminated successfully
}

impl Program {
    fn new(code: Code) -> Self {
        let visits = vec![false; code.len()];
        Program {
            code,
            visits,
            counter: 0,
            pc: 0,
            terminated: false,
        }
    }

    fn tick(&mut self) -> bool {
        let pc = self.pc;
        if self.visits[pc] {
            return false;
        };
        self.visits[pc] = true;

        match self.code[pc] {
            Nop(_) => {
                self.pc += 1;
            }
            Acc(v) => {
                self.pc += 1;
                self.counter += v;
            }
            Jmp(steps) => {
                let max = self.code.len() as isize;
                let pos = (pc as isize + steps).rem_euclid(max);
                self.pc = pos as usize;
            }
        }

        // terminate if current PC was the last one in program
        if pc + 1 == self.code.len() {
            self.terminated = true;
            return false;
        };

        true
    }
}

pub(crate) fn run(step: Step, input: &Vec<String>) -> CustomResult<String> {
    let asm: Code = input
        .iter()
        .map(|l| {
            let tuple: Vec<&str> = l.split(" ").collect();
            Instruction::from_tuple(&tuple)
        })
        .collect();
    let mut program = Program::new(asm);

    match step {
        Step::One => {
            while program.tick() { /* no body needed */ }
            let result: String = format!("{}", program.counter);
            println!("Result = {}", result);
            Ok(result)
        }

        Step::Two => {
            // keep a clean copy around
            let original_code = program.code.clone();

            let mut running = true;
            let mut last_swap_idx = 0usize;

            while running {
                running = program.tick();
                if !running {
                    if program.terminated {
                        break;
                    }
                    let mut new_code = original_code.clone();
                    new_code[last_swap_idx] = Instruction::swap(new_code[last_swap_idx]);
                    program = Program::new(new_code);
                    last_swap_idx += 1;
                    // just a better message than index out of bound error
                    if last_swap_idx == program.code.len() {
                        panic!("all mutations tried, no success, abort!");
                    }
                    running = true;
                }
            }
            let result: String = format!("{}", program.counter);
            println!("Result = {}", result);
            Ok(result)
        }
    }
}
