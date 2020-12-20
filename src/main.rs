use std::{io, io::prelude::*};

fn count_trees(trees: &Vec::<String>, xinc: usize, yinc: usize) -> u64 {
    let mut tree_count: u64 = 0;
    let width = trees[0].len();
    let mut x = 0;
    let mut y = 0;

    while y < trees.len() {
        if trees[y].chars().nth(x).unwrap() == '#' {
            tree_count += 1;
        }
        x = (x + xinc) % width;
        y += yinc;
    }
    return tree_count;
}

fn main() {
    let stdin = io::stdin();

    let trees: Vec::<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let slope11 = count_trees(&trees, 1, 1);
    let slope31 = count_trees(&trees, 3, 1);
    let slope51 = count_trees(&trees, 5, 1);
    let slope71 = count_trees(&trees, 7, 1);
    let slope12 = count_trees(&trees, 1, 2);

    println!("{} {} {} {} {} {}", slope11, slope31, slope51, slope71, slope12, 
                                  slope11 * slope31 * slope51 * slope71 * slope12);
}
