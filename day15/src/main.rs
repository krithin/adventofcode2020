use std::collections::HashMap;

fn main() {
    let starting_numbers: Vec<usize> = vec![0,14,6,20,1,4];
    let mut n: Vec<usize> = Vec::with_capacity(30000000);
    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    for &s in &starting_numbers[..starting_numbers.len()-1] {
        n.push(s);
        last_seen.insert(s, n.len());
    }

    n.push(*starting_numbers.last().unwrap());

    while n.len() < 30000000 {
        n.push(
            match last_seen.insert(*n.last().unwrap(), n.len()) {
                None => 0,
                Some(i) => n.len() - i,
            }
        );
    }

    println!("{}", *n.last().unwrap());
}
