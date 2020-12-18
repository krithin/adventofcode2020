use std::{io, io::prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut valid_passports = 0;

    let mut current_lines = Vec::<String>::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "" {
            let passport = current_lines.join(" ");
            current_lines.clear();

            let fields: Vec::<&str> = passport.split(" ").collect();
            if fields.len() == 7 {
                if !fields.iter().any(|&f| f.starts_with("cid:")) {
                    valid_passports += 1;
                }
            } else if fields.len() == 8 {
                valid_passports += 1;
            }
        } else {
            current_lines.push(line);
        }
    }

    println!("{}", valid_passports);
}