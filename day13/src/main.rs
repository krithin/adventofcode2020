//use std::{io, io::prelude::*};

fn main() {
    let earliest_timestamp: u32 = 1014511;
    let bus_ids = "17,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,433,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19";
    let bus_ids: Vec<u32> = bus_ids.split(',').filter(|&s| s != "x").map(|s| s.parse().unwrap()).collect();

    let wait_times: Vec<u32> = bus_ids.iter().map(|&b| (b - (earliest_timestamp % b)) % b).collect();

    let mut shortest_wait = u32::MAX;
    let mut shortest_wait_bus = 0;
    for i in 0..bus_ids.len() {
        if wait_times[i] < shortest_wait {
            shortest_wait = wait_times[i];
            shortest_wait_bus = bus_ids[i];
        }
    }

    println!("{} {} {}", shortest_wait, shortest_wait_bus, shortest_wait * shortest_wait_bus);
}