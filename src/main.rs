use std::{io, io::prelude::*};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Bag {
    contained_by: HashSet<String>,
}

fn main() {
    let stdin = io::stdin();
    let mut all_bags = HashMap::<String, Bag>::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let [container_str, containees_str] = line.split("contain").map(|b| b.trim()).collect::<Vec<&str>>()[..2] {
            let container_colour = container_str[..container_str.len() - 5].to_string(); // remove the trailing " bags" 
            let containees_str = &containees_str[..containees_str.len() - 1]; // remove the trailing "."
            match containees_str {
                "no other bags" => {},
                _ => {
                    for count_colour_bag in containees_str.split(',').map(|s| s.trim()) {
                        let count_colours = count_colour_bag.split(' ').collect::<Vec<&str>>();
                        assert_eq!(count_colours.len(), 4, "{}", count_colour_bag);

                        let count = count_colours[0].parse::<u8>();
                        let containee_colour = [count_colours[1], count_colours[2]].join(" ");
                        let bags = count_colours[3];

                        match count {
                            Ok(0) => panic!("zero bags?"),
                            Ok(1) => assert_eq!(bags, "bag"),
                            Ok(2..=u8::MAX) => assert_eq!(bags, "bags"),
                            _ => panic!(count),
                        }

                        match all_bags.get_mut(&containee_colour) {
                            Some(bag) => {
                                bag.contained_by.insert(container_colour.clone());
                            },
                            None => {
                                let mut contained_by = HashSet::<String>::new();
                                contained_by.insert(container_colour.clone());
                                all_bags.insert(containee_colour.clone(), Bag {
                                    contained_by: contained_by,
                                });
                            },
                        }
                    }
                }
            }
        } else {
            panic!("at the disco");
        }
    }
    
    let mut bag_queue = VecDeque::<&str>::new();
    let mut seen_bag_set = HashSet::<&str>::new();
    bag_queue.push_back("shiny gold");
    while !bag_queue.is_empty() {
        let colour = bag_queue.pop_front().unwrap();
        seen_bag_set.insert(colour);
        match all_bags.get(colour) {
            Some(bag) => {
                for containing_colour in bag.contained_by.iter() {
                    if !seen_bag_set.contains::<str>(containing_colour) {
                        bag_queue.push_back(containing_colour);
                    }
                }
            },
            None => {},
        }
    }
    seen_bag_set.remove("shiny gold");
    println!("{:?}", seen_bag_set.len());
}