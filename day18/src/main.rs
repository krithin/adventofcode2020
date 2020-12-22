use std::{io, io::prelude::*};

struct StackFrame {
    val: u64,
    next_op: char,
}

fn evaluate_part1(line: &str) -> u64 {
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

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Num(u64),
    Op(char),
}

fn tokenize(line: &str) -> Vec<Token> {
    return line.chars()
    .filter_map(|c| match c {
        ' ' => None,
        '0'..='9' => Some(Token::Num(c.to_digit(10).unwrap() as u64)),
        '+'|'*'|'('|')' => Some(Token::Op(c)),
        _ => panic!("Bad token!"),
    }).collect();
}

fn evaluate_part2(tokens: &[Token]) -> u64 {
    let mut tokens: Vec<Token> = tokens.iter().cloned().collect();

    while let Some(start) = tokens.iter().position(|c| c == &Token::Op('(')) {
        let mut depth = 0;
        let mut i = start;
        let end = loop {
            match tokens.get(i) {
                Some(Token::Op('(')) => depth += 1,
                Some(Token::Op(')')) => {
                    depth -= 1;
                    if depth == 0 { break i; }
                }
                _ => {},
            }
            i += 1;
        };

        let val = evaluate_part2(&tokens[start+1..end]);
        tokens.drain(start..=end);
        tokens.insert(start, Token::Num(val));
    }

    while let Some(i) = tokens.iter().position(|t| t == &Token::Op('+')) {
        let args: Vec<Token> = tokens.drain(i-1..=i+1).collect();
        match (&args[0], &args[2]) {
            (Token::Num(lhs), Token::Num(rhs)) => {
                tokens.insert(i-1, Token::Num(lhs + rhs));
            },
            _ => panic!("Bad value"),
        }
    }

    while let Some(i) = tokens.iter().position(|t| t == &Token::Op('*')) {
        let args: Vec<Token> = tokens.drain(i-1..=i+1).collect();
        match (&args[0], &args[2]) {
            (Token::Num(lhs), Token::Num(rhs)) => {
                tokens.insert(i-1, Token::Num(lhs * rhs));
            },
            _ => panic!("Bad value"),
        }
    }

    assert_eq!(tokens.len(), 1);
    match tokens[0] {
        Token::Num(n) => return n,
        _ => panic!("Bad logic!"),
    }
}

fn main() {
    let stdin = io::stdin();

    let mut values: Vec<u64> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        //let result = evaluate_part1(&line);
        let result = evaluate_part2(&tokenize(&line));
        println!("{}", result);
        values.push(result);
    }

    println!("{}", values.iter().sum::<u64>());
}
