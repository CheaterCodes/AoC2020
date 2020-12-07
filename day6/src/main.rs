use std::{collections::HashSet, env};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

    // Parse answers
    let groups: Vec<Vec<HashSet<char>>> = input.split("\n\n").map(|group| group.split("\n").map(|person| person.chars().collect()).collect()).collect();
    let part1: usize = groups.iter().map(|group| group.iter().fold(group.first().unwrap().clone(), |first, second| first.union(second).copied().collect()).len()).sum();

    println!("{:?}", part1);

    let part2: usize = groups.iter().map(|group| group.iter().fold(group.first().unwrap().clone(), |first, second| first.intersection(second).copied().collect()).len()).sum();
    println!("{:?}", part2);
}
