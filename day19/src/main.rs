use std::{io, io::prelude::*};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Rule {
    rule_refs: Vec<(u8, Option<u8>)>,
    lit: Option<char>,
}

fn match_dfs(rules: &HashMap<u8, Rule>, line: &str, target_rule: u8, dfs_cache: &mut HashMap<(String, u8), bool>) -> bool {
    match dfs_cache.get(&(line.to_string(), target_rule)) {
        Some(cached_result) => *cached_result,
        None => {
            let rule = &rules.get(&target_rule).unwrap();
            match rule.lit {
                Some(c) => return line.len() == 1 && line.contains(c),
                None => rule.rule_refs.iter().any(
                    |&r| match r {
                        (ref1, Some(ref2)) => {
                            if !line.is_empty() {
                                for divider in 0..=line.len()-1 {
                                    if match_dfs(rules, &line[0..divider], ref1, dfs_cache)
                                        && match_dfs(rules, &line[divider..], ref2, dfs_cache) {
                                        dfs_cache.insert((line.to_string(), target_rule), true);
                                        return true;
                                    }
                                }
                            }
                            dfs_cache.insert((line.to_string(), target_rule), false);
                            return false;
                        },
                        (ref1, None) => match_dfs(rules, line, ref1, dfs_cache),
                    }
                )
                
            }
        }
    }
}

fn parse_rule(rules: &mut HashMap<u8, Rule>, line: &str) {
    if let [rule_num, subrules] = line.split(':').collect::<Vec<&str>>()[0..=1] {
        let rule_num: u8 = rule_num.parse().unwrap();
        // A literal does not appear in the same rule as anything else
        rules.insert(rule_num, 
            if subrules.contains('"') {
                Rule { rule_refs: Vec::new(), lit: Some(subrules.trim()[1..=1].chars().next().unwrap()) }
            } else {
                let subrules: Vec<Vec<u8>> = subrules
                    .split('|')
                    .map(|s| s
                        .trim()
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect()
                    ).collect();
                let mut rule_refs: Vec<(u8, Option<u8>)> = Vec::new();
                for subrule in subrules {
                    rule_refs.push(
                        match subrule.len() {
                            1 => (subrule[0], None),
                            2 => (subrule[0], Some(subrule[1])),
                            _ => panic!("Unexpected number of subrules"),
                        }
                    )
                }
                Rule { rule_refs: rule_refs, lit: None }
            }
        );
    } else {
        panic!("Unable to parse rule");
    }
}

fn main() {
    let stdin = io::stdin();

    let mut rules: HashMap<u8, Rule> = HashMap::with_capacity(135);

    let mut parsing_rules: bool = true;
    let mut dfs_cache: HashMap<(String, u8), bool> = HashMap::new();
    let mut match_count = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.len() == 0 { parsing_rules = false; continue; }

        if parsing_rules {
            parse_rule(&mut rules, &line);
        } else {
            let matched = match_dfs(&rules, &line, 0, &mut dfs_cache);
            if matched { match_count += 1 };
        }
    }
    println!("Match count: {}", match_count);
}
