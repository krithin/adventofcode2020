use std::{io, io::prelude::*};
use std::collections::HashSet;

#[derive(Debug)]
#[derive(PartialEq)]
enum Operation {
    acc,
    jmp,
    nop,
}

#[derive(Debug)]
struct OperationParseError {}

impl std::str::FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Operation, OperationParseError> {
        match s {
            "acc" => Ok(Operation::acc),
            "jmp" => Ok(Operation::jmp),
            "nop" => Ok(Operation::nop),
            _ => Err(OperationParseError {}),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    arg: i32,
}

#[derive(Debug)]
struct SimResult {
    acc: i32,
    next_inst_offset: i32,
}

fn simulate(instruction: &Instruction, acc: i32) -> SimResult {
    match instruction.op {
        Operation::acc => SimResult { acc: acc + instruction.arg, next_inst_offset: 1 },
        Operation::jmp => SimResult { acc: acc, next_inst_offset: instruction.arg },
        Operation::nop => SimResult { acc: acc, next_inst_offset: 1 },
    }
}

#[derive(Debug)]
struct CompletionResult {
    acc: i32,
    completes: bool,
}

fn test_completion(instructions: &Vec<Instruction>) -> CompletionResult {
    let mut acc: i32 = 0;
    let mut ip: i32 = 0;
    let mut seen_instructions = HashSet::<i32>::new();
    seen_instructions.insert(0);

    return loop {
        let result = simulate(&instructions[ip as usize], acc);
        acc = result.acc;
        ip = ip + result.next_inst_offset;
        println!("{:?} next_ip {}", result, ip);
        if !seen_instructions.insert(ip) {
            break CompletionResult { acc: acc, completes: false };
        } else if ip == instructions.len() as i32 {
            break CompletionResult { acc: acc, completes: true };
        }
    };
}

fn toggle_instruction(instruction: &Instruction) -> Instruction {
    match instruction.op {
        Operation::jmp => Instruction { op: Operation::nop, arg: instruction.arg },
        Operation::nop => Instruction { op: Operation::jmp, arg: instruction.arg },
        Operation::acc => Instruction { op: Operation::acc, arg: instruction.arg },
    }
}

fn main() {
    let stdin = io::stdin();
    let mut instructions = Vec::<Instruction>::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let [op, arg] = line.split_whitespace().collect::<Vec<&str>>()[..2] {
            instructions.push(Instruction {
                op: op.parse::<Operation>().unwrap(),
                arg: arg.parse::<i32>().unwrap(),
            });
        }
    }

    for i in 0..instructions.len() {
        println!("Mutating {}", i);
        if instructions[i].op == Operation::acc {
            continue;
        }
        instructions[i] = toggle_instruction(&instructions[i]);
        println!("{:?}", instructions);
        let result = test_completion(&instructions);
        if result.completes {
            println!("Completed! {}", result.acc);
            break;
        } else {
            instructions[i] = toggle_instruction(&instructions[i]);
        }
    }
}                                                    