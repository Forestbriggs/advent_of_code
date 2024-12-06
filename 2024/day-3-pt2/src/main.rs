use regex::Regex;
use std::fs::File;
// use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let f: File = File::open("./src/input.txt")?;
    let reader = BufReader::new(f);

    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    let mut matches: Vec<(String, String)> = Vec::new();

    let mut result: i32 = 0;

    for line in reader.lines() {
        let line: String = line?;
        // println!("{}", line);
        for caps in re.captures_iter(&line) {
            // println!("Captures: {:?}", caps);
            // Access capture groups
            let first_number = caps.get(2).map_or("", |m| m.as_str());
            let second_number = caps.get(3).map_or("", |m| m.as_str());
            // println!("First: {}, Second: {}", first_number, second_number);

            if first_number.is_empty() {
                // println!("{}", caps.get(1).unwrap().as_str());
                matches.push((caps.get(1).unwrap().as_str().to_string(), "".to_string()));
            } else {
                matches.push((first_number.to_string(), second_number.to_string()));
            }
        }
    }

    let mut enabled: bool = true;

    for (first, second) in &matches {
        // println!("Match: {}, {}", first, second);
        if first == "don't()" {
            // println!("Don't");
            enabled = false;
        } else if first == "do()" {
            // println!("Do");
            enabled = true;
        } else if enabled {
            // println!("Enabled");
            result += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        }
    }

    println!("{}", result);
    Ok(())
}
