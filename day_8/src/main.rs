use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
    Err,
}

#[derive(Debug)]
struct Interpreter {
    ip: usize,
    acc: i32,
    program: Vec<Instruction>,
    seen: Vec<bool>,
}

impl Interpreter {
    pub fn new(program: &Vec<String>) -> Self {
        lazy_static! {
            static ref MATCHER: Regex = Regex::new(r"^([a-z]{3}) ((\+|-)[0-9]+)$").unwrap();
        };

        let mut parsed_program = vec![];
        program.iter().for_each(|line| {
            let caps = MATCHER.captures(line).unwrap();
            let val = caps[2].parse().unwrap();
            let instruction = match &caps[1] {
                "nop" => Instruction::Nop(val),
                "acc" => Instruction::Acc(val),
                "jmp" => Instruction::Jmp(val),
                _ => Instruction::Err,
            };
            parsed_program.push(instruction);
        });

        let len = parsed_program.len();
        Interpreter {
            ip: 0,
            acc: 0,
            program: parsed_program,
            seen: vec![false; len],
        }
    }

    fn exec(&mut self) {
        self.seen[self.ip] = true;
        match self.program[self.ip] {
            Instruction::Acc(x) => {
                self.acc += x;
                self.ip += 1
            }
            Instruction::Jmp(x) => self.ip = (self.ip as i32 + x) as usize,
            Instruction::Nop(_) => self.ip += 1,
            _ => panic!("Should never happen"),
        };
    }

    pub fn part1(&mut self) -> i32 {
        while !self.seen[self.ip] {
            self.exec()
        }
        self.acc
    }

    fn terminates(&mut self) -> bool {
        while !self.seen[self.ip] {
            self.exec();
            if self.ip >= self.program.len() {
                return true;
            }
        }
        false
    }

    pub fn part2(&mut self) -> i32 {
        for (idx, instr) in self.program.iter().enumerate() {
            let mut new_program = self.program.clone();
            let new_instr = match instr {
                Instruction::Jmp(x) => Some(Instruction::Nop(*x)),
                Instruction::Nop(x) => Some(Instruction::Jmp(*x)),
                _ => None,
            };
            if new_instr.is_some() {
                new_program[idx] = new_instr.unwrap();
                let mut interpreter = Interpreter {
                    ip: 0,
                    acc: 0,
                    program: new_program,
                    seen: vec![false; self.program.len()],
                };
                if interpreter.terminates() {
                    return interpreter.acc;
                }
            };
        }
        panic!("Did not find a solution");
    }
}

fn parse_input(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file).unwrap();
    contents
        .split('\n')
        .map(|x| x.to_string())
        .filter(|x| x.len() != 0)
        .collect()
}

fn main() {
    let input = parse_input("input.txt");
    let mut interpreter = Interpreter::new(&input);
    dbg!(interpreter.part1());

    let mut interpreter = Interpreter::new(&input);
    dbg!(interpreter.part2());
}
