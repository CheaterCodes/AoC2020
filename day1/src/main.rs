use std::env;
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
    
    // Parse file into numbers
    let numbers: Vec<u32> = BufReader::new(file).lines().map(|l| l.unwrap()).map(|s| s.parse::<u32>().unwrap()).collect();

    // Simply iterate twice. Lazy solution with O(n²).
    for number1 in &numbers {
        for number2 in &numbers {
            if number1 + number2 == 2020 {
                println!("{} + {} = 2020", number1, number2);
                println!("{} x {} = {}", number1, number2, number1 * number2)
            }
        }
    }

    // Simply iterate thrice. Lazy solution with O(n3).
    for number1 in &numbers {
        for number2 in &numbers {
            for number3 in &numbers {
                if number1 + number2 + number3 == 2020 {
                    println!("{} + {} + {} = 2020", number1, number2, number3);
                    println!("{} x {} x {} = {}", number1, number2, number3, number1 * number2 * number3);
                }
            }
        }
    }
}
