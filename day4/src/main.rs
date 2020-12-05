use std::{collections::HashMap, env};
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

    // Parse passports
    let passports: Vec<HashMap<&str, &str>> = input.split("\n\n").map(|s| s.split_whitespace().map(|s| s.splitn(2, ':')).map(|mut split| (split.next().unwrap(), split.next().unwrap())).collect()).collect();
    
    // Part one
    let mut valid_count = 0;
    for passport in &passports {
        if passport.get("byr") == None {
            continue;
        }
        if passport.get("iyr") == None {
            continue;
        }
        if passport.get("eyr") == None {
            continue;
        }
        if passport.get("hgt") == None {
            continue;
        }
        if passport.get("hcl") == None {
            continue;
        }
        if passport.get("ecl") == None {
            continue;
        }
        if passport.get("pid") == None {
            continue;
        }
        valid_count += 1;
    }
    println!("Valid Passports: {}", valid_count);

    // Part two
    // This is a mess
    let mut valid_count = 0;
    for passport in &passports {
        // Birth Year
        if let Some(byr) = passport.get("byr") {
            if let Ok(value) = byr.parse::<u32>() {
                if value < 1920 || value > 2002 {
                    continue;
                }
            }
            else { 
                continue;
            }
        }
        else {
            continue;
        }

        // Issue Year
        if let Some(iyr) = passport.get("iyr") {
            if let Ok(value) = iyr.parse::<u32>() {
                if value < 2010 || value > 2020 {
                    continue;
                }
            }
            else { 
                continue;
            }
        }
        else {
            continue;
        }

        // Expiration Year
        if let Some(eyr) = passport.get("eyr") {
            if let Ok(value) = eyr.parse::<u32>() {
                if value < 2020 || value > 2030 {
                    continue;
                }
            }
            else { 
                continue;
            }
        }
        else {
            continue;
        }

        // Height
        if let Some(hgt) = passport.get("hgt") {
            if hgt.len() < 2 {
                continue;
            }
            let (number, unit) = hgt.split_at(hgt.len() - 2);
            if let Ok(value) = number.parse::<u32>() {
                if unit == "cm" {
                    if value < 150 || value > 193 {
                        continue;
                    }
                }
                else if unit == "in" {
                    if value < 59 || value > 76 {
                        continue;
                    }
                }
                else {
                    continue;
                }
            }
            else { 
                continue;
            }
        }
        else {
            continue;
        }

        // Hair color
        if let Some(hcl) = passport.get("hcl") {
            let re = Regex::new("#[0-9a-f]{6}").unwrap();
            if !re.is_match(hcl) {
                continue;
            }
        }
        else {
            continue;
        }

        // Eye Color
        if let Some(ecl) = passport.get("ecl") {
            let re = Regex::new("(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)").unwrap();
            if !re.is_match(ecl) {
                continue;
            }
        }
        else {
            continue;
        }

        // Passport ID
        if let Some(pid) = passport.get("pid") {
            let re = Regex::new("[0-9]{9}").unwrap();
            if !re.is_match(pid) {
                continue;
            }
        }
        else {
            continue;
        }
        valid_count += 1;
    }
    println!("Valid Passports: {}", valid_count);
}
