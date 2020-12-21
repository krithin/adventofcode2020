use std::collections::HashMap;

fn main() {
    let starting_numbers: Vec<u32> = vec![0,14,6,20,1,4];
    let mut last_n: u32;
    let mut turn_num: u32 = 0;
    let mut last_seen: HashMap<u32, u32> = HashMap::new();

    for &s in &starting_numbers[..starting_numbers.len()-1] {
        turn_num = turn_num + 1;
        last_seen.insert(s, turn_num);
    }

    last_n = *starting_numbers.last().unwrap();
    turn_num += 1;

    for turn_num in turn_num..30000000 {
        last_n = match last_seen.insert(last_n, turn_num) {
            None => 0,
            Some(i) => turn_num - i,
        };
    }

    println!("{}", last_n);
}
