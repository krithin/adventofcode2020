use std::{io, io::prelude::*};
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    colour_counts: HashMap<String, u32>,
}

fn count_nested_bags(all_bags: &HashMap<String, Bag>, colour: &str) -> u32 {
    match all_bags.get(colour) {
        None => panic!("Impossible colour"),
        Some(bag) => bag.colour_counts.iter().map(|(colour, count)| count * count_nested_bags(all_bags, colour)).sum::<u32>() + 1,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut all_bags = HashMap::<String, Bag>::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let [container_str, containees_str] = line.split("contain").map(|b| b.trim()).collect::<Vec<&str>>()[..2] {
            let container_colour = container_str[..container_str.len() - 5].to_string(); // remove the trailing " bags" 
            assert_eq!(all_bags.contains_key(&container_colour), false);
            all_bags.insert(container_colour.clone(), Bag { colour_counts: HashMap::<String, u32>::new() });

            let containees_str = &containees_str[..containees_str.len() - 1]; // remove the trailing "."
            match containees_str {
                "no other bags" => {},
                _ => {
                    for count_colour_bag in containees_str.split(',').map(|s| s.trim()) {
                        let count_colours = count_colour_bag.split(' ').collect::<Vec<&str>>();
                        assert_eq!(count_colours.len(), 4, "{}", count_colour_bag);

                        let count = count_colours[0].parse::<u32>();
                        let containee_colour = [count_colours[1], count_colours[2]].join(" ");
                        let bags = count_colours[3];

                        match count {
                            Ok(0) => panic!("zero bags?"),
                            Ok(1) => assert_eq!(bags, "bag"),
                            Ok(2..=u32::MAX) => assert_eq!(bags, "bags"),
                            _ => panic!(count),
                        }

                        match all_bags.get_mut(&container_colour) {
                            Some(bag) => {
                                bag.colour_counts.insert(containee_colour, count.unwrap());
                            },
                            None => panic!("no  bag!"),
                        }
                    }
                }
            }
        } else {
            panic!("at the disco");
        }
    }

    println!("{}", count_nested_bags(&all_bags, "shiny gold") - 1);
}