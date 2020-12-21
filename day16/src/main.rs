use std::{io, io::prelude::*};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut fields: Vec<String> = Vec::new();
    let mut your_ticket: String = String::new();
    let mut nearby_tickets: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.len() {
            0 => break,
            _ => fields.push(line)
        }
    }

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        your_ticket = match line.len() {
            0 => break,
            _ => line,
        }
    }

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.len() {
            0 => break,
            15 => {},  // "nearby tickets:" - this is a lame way to skip the header row.
            _ => nearby_tickets.push(line)
        }
    }

    let fields: HashMap<String, ((u16, u16), (u16, u16))> = 
        fields
            .iter()
            .map(
                |s| 
                {
                    let mut parts = s.split(":");
                    let name = parts.next().unwrap();
                    let values = parts.next().unwrap()
                        .split("or")
                        .map(|r| r
                            .split('-')
                            .map(|n| n
                                .trim()
                                .parse::<u16>()
                                .unwrap()
                            ).collect::<Vec<u16>>()
                        ).collect::<Vec<Vec<u16>>>();
                    return (name.to_string(), ((values[0][0], values[0][1]), (values[1][0], values[1][1])));
                }
            ).collect();
    
    let your_ticket: Vec<u16> = your_ticket.split(',').map(|n| n.parse().unwrap()).collect();

    let nearby_tickets: Vec<Vec<u16>> = nearby_tickets.iter().map(|s| s.split(',').map(|n| n.parse().unwrap()).collect()).collect();

    println!("fields:\n{:?}", fields);
    println!("your_ticket:\n{:?}", your_ticket);
    println!("nearby_tickets:\n{:?}", nearby_tickets);

    // Part 1
    let error_rate: u32 = nearby_tickets.iter().map(
        |ticket|
        ticket.iter().map(
            |value| {
                for (_name, ((min1, max1), (min2, max2))) in &fields {
                    if (min1 <= value && value <= max1) || (min2 <= value && value <= max2) {
                        return 0;
                    }
                }
                return *value as u32;
            }
        ).sum::<u32>()
    ).sum();

    println!("Error rate: {}", error_rate);

    // Part 2
    let valid_tickets: Vec<Vec<u16>> = nearby_tickets.iter().filter(
        |&ticket|
        ticket.iter().all(
            |value| {
                for (_name, ((min1, max1), (min2, max2))) in &fields {
                    if (min1 <= value && value <= max1) || (min2 <= value && value <= max2) {
                        return true;
                    }
                }
                return false;
            }
        )
    ).cloned().collect();


}
