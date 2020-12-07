use std::{collections::{HashMap, HashSet}, env};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

// Build and run with `cargo run -- ./input.txt` from the day1 base dir
fn main() {
    // Get commandline arguments
    let args: Vec<String> = env::args().collect();

    // Check argument count
    if args.len() != 2 {
        println!("Usage: day1 <input file>");
        return;
    }

    // Open file
    let path = args.get(1).unwrap();
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Problem opening file: {:?}", e)
    };
    
    // Read file
    let mut input = String::new();
    let _ = BufReader::new(file).read_to_string(&mut input);

    let color_regex = Regex::new("(\\b\\w+ \\b\\w+)").unwrap();
    let count_regex= Regex::new("(\\d) (\\b\\w+ \\b\\w+)").unwrap();

    // Parse rules
    let rules: HashMap<&str, HashMap<&str, u32>> = input.split("\n").map(|rule| (color_regex.captures(rule).unwrap().get(0).unwrap().as_str(), count_regex.captures_iter(rule).map(|capture| (capture.get(2).unwrap().as_str(), capture.get(1).unwrap().as_str().parse::<u32>().unwrap())).collect())).collect();

    let mut bags_to_check = Vec::<&str>::new();
    bags_to_check.push("shiny gold");

    let mut valid_bags = HashSet::<&str>::new();
    let mut checked_bags = HashSet::<&str>::new();

    while let Some(bag_to_check) = bags_to_check.pop() {
        if checked_bags.insert(bag_to_check) {
            valid_bags.insert(bag_to_check);
            bags_to_check.extend(rules.iter().filter_map(|(bag, contents)| if contents.contains_key(bag_to_check) { Some(bag) } else { None }))
        }
    }

    println!("Valid Bags: {:}", valid_bags.len() - 1);

    let mut bags_to_check = Vec::<&str>::new();
    bags_to_check.push("shiny gold");

    let mut total: usize = 0;
    while let Some(bag_to_check) = bags_to_check.pop() {
        total += 1;
        for (bag, count) in &rules[bag_to_check] {
            for _ in 0..*count {
                bags_to_check.push(*bag);
            }
        }
    }
    
    println!("Total Bags: {:}", total - 1);
}