use std::{io, io::prelude::*};

struct StackFrame {
    val: u64,
    next_op: char,
}

fn evaluate(line: &str) -> u64 {
    let mut curr: u64 = 0;
    let mut last_op = '+';
    let mut stack: Vec<StackFrame> = Vec::new();

    for c in line.chars() {
        // The numbers in the sample input are all single digits!
        match c {
            '(' => {
                stack.push(StackFrame {val: curr, next_op: last_op});
                curr = 0;
                last_op = '+';
            },
            ')' => {
                let StackFrame { val, next_op } = stack.pop().unwrap();
                if next_op == '+' {
                    curr += val;
                } else {
                    curr *= val;
                }
            },
            '+' | '*' => { last_op = c; }
            '0'..='9' => {
                let n = c.to_digit(10).unwrap();
                if last_op == '+' {
                    curr += n as u64;
                } else {
                    curr *= n as u64;
                }
            }
            ' ' => {}, // ignore spaces
            _ => panic!("bad character!"),
        }
    }
    return curr;
}

fn main() {
    let stdin = io::stdin();

    let mut values: Vec<u64> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let result = evaluate(&line);
        println!("{}", result);
        values.push(result);
    }

    println!("{}", values.iter().sum::<u64>());
}
