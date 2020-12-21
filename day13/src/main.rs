struct BusPosition {
    bus_id: u64,
    position: u64,
}

fn main() {
    let bus_ids = "17,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,23,x,x,x,x,13,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,433,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19";

    let bus_positions: Vec<BusPosition> =
        bus_ids.split(',')
            .enumerate()
            .filter(|(_i, bus_id)| *bus_id != "x")
            .map(|(i, bus_id)| BusPosition { bus_id: bus_id.parse().unwrap(), position: i as u64 })
            .collect();

    let mut increment: u64 = 1;
    let mut start = 0;  // Earliest possible start identified so far.

    for bus in bus_positions {
        while (start + bus.position) % bus.bus_id != 0 {
            start += increment;
        }
        increment *= bus.bus_id;  // All the input bus IDs are coprime!
    }

    println!("{}", start);
}
