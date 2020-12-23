use std::collections::VecDeque;

fn main() {

    let input = "586439172";
    //let input = "389125467";

    let mut seq: VecDeque<u32> = input.chars().map(|c| c.to_digit(10).unwrap() as u32).collect();
    // for i in seq.len()+1..=1000000 {
    //     seq.push_back(i as u32);
    // }
    let mut dst: usize;
    let num_moves = 100;
    let max_target = seq.len() as u32;

    for move_num in 1..=num_moves {
        if move_num % 1000 == 0 { println!("{}", move_num);}

        let current: u32 = seq.pop_front().unwrap();
        let mut target = current - 1;
        seq.push_back(current);

        let three: VecDeque<u32> = seq.drain(0..3).collect();

        println!("target: {}, seq: {:?}, three: {:?}", target, seq, three);
        if target < 1 { target = max_target; }
        while three.contains(&target) {
            target -= 1;
            if target < 1 { target = max_target; }
        }

        dst = 0;
        for (i, &val) in seq.iter().enumerate() {
            if val == target {
                dst = i + 1;
            }
        }
        println!("final target: {}, seq: {:?}, dst: {}", target, seq, dst);

        for &i in three.iter().rev() {
            seq.insert(dst, i);
        }

        println!("{:?}", seq);
    }

    println!("{:?}", seq);
}
