use std::{io, io::prelude::*};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut ingredient_lines: Vec<HashSet<String>> = Vec::new();
    let mut allergen_lines: Vec<HashSet<String>> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let [ingredients, allergens] = line.split(" (contains ").collect::<Vec<_>>()[0..2] {
            ingredient_lines.push(ingredients.split_whitespace().map(|s| s.to_string()).collect::<HashSet<String>>());
            allergen_lines.push(allergens[..allergens.len()-1].split(',').map(|s| s.trim().to_string()).collect::<HashSet<String>>());
        } else {
            panic!("Bad split pattern!")
        }
    }
    // let ingredients: HashSet<String> = ingredient_lines.iter().flatten().cloned().collect();
    // let allergens: HashSet<String> = allergen_lines.iter().flatten().cloned().collect();
    // println!("{:?}\n{:?}\n{:?}\n{:?}\n", ingredient_lines, ingredients, allergen_lines, allergens);

    let mut possible_allergens: HashMap<&str, HashSet<String>> = HashMap::new();
    for (allergen_line, ingredient_line) in allergen_lines.iter().zip(ingredient_lines.iter()) {
        for allergen in allergen_line {
            match possible_allergens.get_mut(allergen.as_str()) {
                Some(x) => { x.retain(|s| ingredient_line.contains(s)); },
                None => { possible_allergens.insert(allergen, ingredient_line.clone()); },
            }
        }
    }

    println!("possible_allergens: {:?}", possible_allergens);
    let possible_allergenic_ingredients: HashSet<String> = possible_allergens.iter().flat_map(|(_k, v)| v.iter().cloned()).collect();
    println!("possible_allergenic_ingredients: {:?}", possible_allergenic_ingredients);

    let mut non_allergen_count: u32 = 0;
    for ingredient_line in ingredient_lines.iter() {
        for ingredient in ingredient_line {
            if !possible_allergenic_ingredients.contains(ingredient) {
                non_allergen_count += 1;
            }
        }
    }
    println!("non-allergen count: {}", non_allergen_count);

    let mut ingredient_allergen_assignments: HashMap<String, String> = HashMap::with_capacity(possible_allergenic_ingredients.len());
    while ingredient_allergen_assignments.len() < possible_allergenic_ingredients.len() {
        let mut ingredient: String = String::new();
        let mut found_allergen: &str = "";
        for (allergen, ingredients) in possible_allergens.iter_mut() {
            if ingredients.len() == 1 {
                ingredient = ingredients.iter().next().cloned().unwrap();
                found_allergen = allergen;
                ingredient_allergen_assignments.insert(ingredient.clone(), allergen.to_string());
                break;
            }
        }
        possible_allergens.remove(found_allergen);
        for (_allergen, ingredients) in possible_allergens.iter_mut() {
            ingredients.remove(ingredient.as_str());
        }
    }
    println!("{:?}", ingredient_allergen_assignments);

    let mut ingredient_allergen_assignments: Vec<(String, String)> = 
        ingredient_allergen_assignments
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
    ingredient_allergen_assignments.sort_by(|a, b| a.1.cmp(&b.1));
    let part2_answer: String = ingredient_allergen_assignments.iter().map(|(i, _a)| i.clone()).collect::<Vec<_>>().join(",");
    println!("{}", part2_answer);
}
