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
    'lazy2: for number1 in &numbers {
        for number2 in &numbers {
            if number1 + number2 == 2020 {
                println!("Lazy: {} + {} = 2020", number1, number2);
                println!("Lazy: {} x {} = {}", number1, number2, number1 * number2);
                break 'lazy2;
            }
        }
    }

    // Fast solution. Array size could be reduced by any constant factor using a hash table. O(n) in any case.
    // This assumes that all numbers are positive
    let mut seen = [false; 2020 + 1];
    for number in &numbers {
        // Skip numbers out of range
        if *number > 2020 {
            continue;
        }

        // Check if complementary number was seen already
        if seen[2020 - *number as usize] {
            println!("Fast: {} + {} = 2020", number, 2020 - number);
            println!("Fast: {} x {} = {}", number, 2020 - number, number * (2020 - number));
            break;
        }

        seen[*number as usize] = true;
    }

    // Simply iterate thrice. Lazy solution with O(n3).
    'lazy3: for number1 in &numbers {
        for number2 in &numbers {
            for number3 in &numbers {
                if number1 + number2 + number3 == 2020 {
                    println!("Lazy: {} + {} + {} = 2020", number1, number2, number3);
                    println!("Lazy: {} x {} x {} = {}", number1, number2, number3, number1 * number2 * number3);
                    break 'lazy3;
                }
            }
        }
    }
}
