fn next_cup(cups: &[usize], label: usize) -> usize {
    return cups[label];
}

// removes the cup _following_ label, and returns the label of the removed cup
fn remove_next(cups: &mut [usize], label: usize) -> usize {
    let c = next_cup(cups, label);
    cups[label] = next_cup(cups, c);
    cups[c] = 0;
    return c;
}

// inserts a cup after the one with the given label
fn insert_after(cups: &mut [usize], position: usize, new_label: usize) {
    let c = next_cup(cups, position);
    cups[position] = new_label;
    cups[new_label] = c;
}

fn main() {
    let input = "586439172";
    //let input = "389125467";
    let input: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    const NUM_CUPS: usize = 1000000;
    let mut cups: [usize; NUM_CUPS + 1] = [0; NUM_CUPS + 1];

    for i in 0..input.len()-1 {
        cups[input[i]] = input[i+1];
    }
    cups[input[input.len()-1]] = input.len() + 1;
    for i in input.len()+1..=NUM_CUPS-1 {
        cups[i] = i+1;
    }
    cups[NUM_CUPS] = input[0];

    //println!("{:?}", cups);

    let mut curr = input[0];
    let mut target: usize;
    const NUM_MOVES: u32 = 10000000;
    for i in 1..=NUM_MOVES {
        if i % 10000 == 0 { println!("move {}", i)}

        target = curr-1;
 
        let mut removed_cups: Vec<usize> = Vec::new();
        for _j in 1..=3 {
            removed_cups.push(remove_next(&mut cups, curr));
        }

        if target < 1 { target = NUM_CUPS }
        while removed_cups.contains(&target) {
            target -= 1;
            if target < 1 { target = NUM_CUPS }
        }

        for new_label in removed_cups {
            insert_after(&mut cups, target, new_label);
            target = new_label;
        }

        curr = next_cup(&cups, curr);
    }

    let cup1 = next_cup(&cups, 1);
    let cup2 = next_cup(&cups, cup1);
    println!("{} {} {}", cup1, cup2, cup1 * cup2);
}
