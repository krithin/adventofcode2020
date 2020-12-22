use std::{io, io::prelude::*};
use std::collections::HashMap;
use std::collections::HashSet;

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

    //println!("fields:\n{:?}", fields);
    //println!("your_ticket:\n{:?}", your_ticket);
    //println!("nearby_tickets:\n{:?}", nearby_tickets);

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

    //println!("valid_tickets:\n{:?}", nearby_tickets);

    let mut field_cols: HashMap<String, HashSet<usize>> = fields.iter()
        .map(|(name, ((min1, max1),(min2,max2)))|
            (
                name.clone(),
                (0..valid_tickets[0].len())
                    .filter( |col|
                        valid_tickets.iter()
                            .map(|ticket| ticket.get(*col).unwrap())
                            .all( |v|  (min1 <= v && v <= max1) || (min2 <= v && v<= max2))
                    ).collect::<HashSet<usize>>()
            )
        ).collect();

    //println!("field_cols:\n{:?}", field_cols);

    let mut field_col_assignments: HashMap<String, usize> = HashMap::new();
    while !(&field_cols.is_empty()) {
        // greedily pick columns, because that's good enough for this dataset
        let mut field_to_remove = String::new();
        for (name, cols) in &field_cols {
            if cols.len() == 1 {
                field_to_remove = name.clone();
            }
        }
        if field_to_remove.len() > 0 {
            let cols = field_cols.remove(&field_to_remove).unwrap();
            assert_eq!(cols.len(), 1);
            let col = *cols.iter().next().unwrap();
            field_col_assignments.insert(field_to_remove, col);
            for (_name, set) in &mut field_cols {
                set.remove(&col);
            }
        } else {
            panic!("Greedy deletion failure!")
        }
    }

    //println!("field_col_assignments: {:?}", field_col_assignments);

    let your_departure_product = field_col_assignments.iter()
        .filter(|(name, _col)| name.starts_with("departure "))
        .map(|(_name, &col)| your_ticket.get(col).unwrap())
        .fold(1 as u64, |acc, val| acc * (*val as u64));

    println!("your_departure_product: {}", your_departure_product);
}
