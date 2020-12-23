fn next_cup(cups: &[u32], label: u32) -> u32 {
    return cups[label as usize];
}

// removes the cup _following_ label, and returns the label of the removed cup
fn remove_next(cups: &mut [u32], label: u32) -> u32 {
    let c = next_cup(cups, label);
    cups[label as usize] = next_cup(cups, c);
    cups[c as usize] = 0;
    return c;
}

// inserts a cup after the one with the given label
fn insert_after(cups: &mut [u32], position: u32, new_label: u32) {
    let c = next_cup(cups, position);
    cups[position as usize] = new_label;
    cups[new_label as usize] = c;
}

fn main() {
    let input = "586439172";
    //let input = "389125467";
    let input: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap() as u32).collect();

    const NUM_CUPS: usize = 9;
    let mut cups: [u32; NUM_CUPS + 1] = [0; NUM_CUPS + 1];

    for i in 0..input.len()-1 {
        cups[input[i] as usize] = input[i+1];
    }
    cups[input[input.len() - 1] as usize] = input[0];

    let mut curr = input[0];
    let mut target: u32;
    const NUM_MOVES: u32 = 100;
    for _i in 1..=NUM_MOVES {
        target = curr-1;
 
        let mut removed_cups: Vec<u32> = Vec::new();
        for _j in 1..=3 {
            removed_cups.push(remove_next(&mut cups, curr));
        }

        if target < 1 { target = NUM_CUPS as u32 }
        while removed_cups.contains(&target) {
            target -= 1;
            if target < 1 { target = NUM_CUPS as u32 }
        }

        for new_label in removed_cups {
            insert_after(&mut cups, target, new_label);
            target = new_label;
        }

        curr = next_cup(&cups, curr);
    }

    let mut i = next_cup(&cups, 1);
    while i != 1 {
        print!("{}", i);
        i = next_cup(&cups, i);
    }
    println!("");
}
