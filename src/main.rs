use std::{io, io::prelude::*};
use std::collections::HashSet;
use regex::Regex;

// min and max are inclusive
fn test_year(input: &str, min: u32, max: u32) -> bool {
    if input.len() !=4 {
        return false;
    } else {
        let year = input.parse::<u32>().unwrap();
        return year >= min && year <= max;
    }
}

fn test_hgt(input: &str) -> bool {
    let suffix = input[input.len()-2..].to_string();
    let value = input[..input.len()-2].parse::<u32>();
    return match suffix.as_str() {
        "cm" => {
            match value {
                Ok(num) => num <= 193 && num >= 150,
                _ => false,
            }
        },
        "in" => {
            match value {
                Ok(num) => num <= 76 && num >= 59,
                _ => false,
            }
        },
        _ => false,
    }
}

fn test_hcl(input: &str, HAIR_COLOUR_REGEX: &regex::Regex) -> bool {
    return HAIR_COLOUR_REGEX.is_match(input);
}

fn test_ecl(input: &str, VALID_EYE_COLOURS: &HashSet::<&str>) -> bool {
    return VALID_EYE_COLOURS.contains(input);
}

fn test_pid(input: &str, PASSPORT_REGEX: &regex::Regex) -> bool {
    return PASSPORT_REGEX.is_match(input);
}

fn main() {
    let stdin = io::stdin();
    let mut valid_passports = 0;

    let HAIR_COLOUR_REGEX: regex::Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let VALID_EYE_COLOURS: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
    let PASSPORT_REGEX: regex::Regex = Regex::new(r"^\d{9}$").unwrap();

    let mut current_lines = Vec::<String>::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "" {
            let passport = current_lines.join(" ");
            current_lines.clear();
            let mut byr_valid = false;
            let mut iyr_valid = false;
            let mut eyr_valid = false;
            let mut hgt_valid = false;
            let mut hcl_valid = false;
            let mut ecl_valid = false;
            let mut pid_valid = false;
            let mut overall_valid = true;

            let fields = passport.split(' ');
            for field in fields {
                if field.chars().nth(3) != Some(':') {
                    overall_valid = false;
                    break;
                }
                match &field[0..3] {
                    "byr" => {
                        byr_valid = !byr_valid && test_year(&field[4..], 1920, 2002);
                    },
                    "iyr" => {
                        iyr_valid = !iyr_valid && test_year(&field[4..], 2010, 2020);
                    },
                    "eyr" => {
                        eyr_valid = !eyr_valid && test_year(&field[4..], 2020, 2030);
                    },
                    "hgt" => {
                        hgt_valid = !hgt_valid && test_hgt(&field[4..]);
                    },
                    "hcl" => {
                        hcl_valid = !hcl_valid && test_hcl(&field[4..], &HAIR_COLOUR_REGEX);
                    },
                    "ecl" => {
                        ecl_valid = !ecl_valid && test_ecl(&field[4..], &VALID_EYE_COLOURS);
                    },
                    "pid" => {
                        pid_valid = !pid_valid && test_pid(&field[4..], &PASSPORT_REGEX);
                    },
                    "cid" => {
                        // Do nothing
                    },
                    _ => {
                        overall_valid = false;
                    }
                }
            }
            let valid = byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid && overall_valid;
            if valid {
                valid_passports += 1;
            } else {
                println!("{}", passport);
                println!("byr: {}, iyr: {}, eyr: {}, hgt: {}, hcl: {}, ecl: {}, pid: {}, overall: {}", byr_valid, iyr_valid, eyr_valid, hgt_valid, hcl_valid, ecl_valid, pid_valid, overall_valid);
                println!("");
            }
        } else {
            current_lines.push(line);
        }
    }

    println!("{}", valid_passports);
}