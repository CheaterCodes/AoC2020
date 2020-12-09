use std::{collections::{HashSet, VecDeque}, env};
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
    let lines = BufReader::new(file).lines();
    let numbers: Vec<u64> = lines.map(|l| l.unwrap().parse().unwrap()).collect();
    let mut iter = numbers.iter();

    // Get initial preamble
    let mut preamble: VecDeque<u64> = iter.by_ref().take(25).copied().collect();

    // Check all numbers
    let mut invalid = None;
    while let Some(&value) = iter.next() {
        if !is_valid(&preamble, value) {
            println!("Invalid value: {}", value);
            invalid = Some(value);
            break;
        }
        preamble.pop_front();
        preamble.push_back(value);
    }

    // Part two
    // O(n) complexity :)
    if let Some(invalid) = invalid {
        let sums: Vec<u64> = numbers.iter().scan(0, |a, b| { *a += b; Some(*a) }).collect();
        let mut start = 0;
        let mut end = 0;

        loop {
            let span = sums[end] - sums[start];
            if span < invalid {
                end += 1;
            }
            else if span > invalid {
                start += 1;
            }
            else {
                break;
            }
        }

        println!("Span from {} until {}.", start + 1, end);
        let span = &numbers[(start + 1)..(end + 1)];
        println!("{:?}, sum = {}", span, span.iter().sum::<u64>());
        let min = span.iter().min().unwrap();
        let max = span.iter().max().unwrap();
        println!("Minimum: {}, Maximum: {}, Result: {}", min, max, min + max);
    }
}

fn is_valid(preamble: &VecDeque<u64>, value: u64) -> bool {
    let set: HashSet<u64> = preamble.iter().copied().collect();
    for num1 in &set {
        let other = set.iter().find(|&num2| num1 + num2 == value && num1 != num2);
        if other.is_some() {
            return true;
        }
    }
    false
}