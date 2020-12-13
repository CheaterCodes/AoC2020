use std::{env, error::Error};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Convenience macro for generating errors
macro_rules! error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        Box::<dyn Error>::from(res)
    }};
}

// Build and run with `cargo run -- ./input.txt` from the day1 base dir
fn main() -> Result<(), Box<dyn Error>> {
    // Get commandline arguments
    let args: Vec<String> = env::args().collect();

    // Open file
    let path = args.get(1).ok_or(error!("Usage: cargo run -- <input file>"))?;
    let file = File::open(path)?;
    
    // Read file
    let mut lines = BufReader::new(file).lines();

    let start_time: i32 = lines.next().ok_or(error!("Missing input!"))??.parse()?;
    let intervals: Vec<Option<i32>> = lines.next().ok_or(error!("Missing input!"))??.split(',').map(|s| s.parse().ok()).collect();

    let existing_intervals: Vec<i32> = intervals.iter().filter_map(|&i| i).collect();
    let wait_time: Vec<i32> = existing_intervals.iter().map(|&i| i - (start_time - (start_time - 1) / i * i)).collect();

    let earliest = wait_time.iter().zip(existing_intervals.iter()).min_by_key(|&(&wait, &_interval)| wait).unwrap();
    println!("Bus line {} departs after {} minutes.", earliest.1, earliest.0);
    println!("Result: {}", earliest.0 * earliest.1);

    /* Part Two
        Given a list of numbers 13,x,x,41,x,x,x,37,...
        Find a number t so that
            13  * k13  - 0  = t
            41  * k41  - 3  = t
            37  * k37  - 7  = t
            ...
        Or as modulo arithmetic:
            t = 0       mod 1
            t = 13 - 0  mod 13
            t = 41 - 3  mod 41
            t = 37 - 7  mod 37
            ...
        Note: The numbers appear to all be prime!

        Idea:
        - Increment until first equation is satisfied
        - Now increment by 'line 1' until second equation is satisfied.
          - Incrementing this way makes sure equation one stays satisfied
          - This way you need to invrement at most "line 2' times"
        - Now increment by 'line 1' * 'line 2' until third equation is satisfied.
          - Incrementing this way makes sure equation one and two stay satisfied
          - This way you need to invrement at most "line 3' times"
        - Rinse and repeat

        Complexity: O(n*m) where
            - n is the number of lines
            - m is the value of the biggest/average line
    */

    let line_and_offset: Vec<(usize, usize)> = intervals.iter().enumerate().filter_map(|(i, &line)| line.map(|line| (line as usize, ((line - (i as i32 % line)) % line) as usize))).collect();

    let mut t: u128 = 1;
    let mut increment: u128 = 1;
    for &(line, offset) in &line_and_offset {
        while t % line as u128 != offset as u128 {
            t += increment;
        }
        increment *= line as u128;
        println!("Checked line {}\n- Current t = {}\n- Current increment = {}", line, t, increment);
    }

    println!("Solution: {}", t);
    // Everything ok!
    Ok(())
}
