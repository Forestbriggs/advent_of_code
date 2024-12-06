use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let f: File = File::open("./src/input.txt")?;
    let reader = BufReader::new(f);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut matches: Vec<(String, String)> = Vec::new();

    let mut result: i32 = 0;

    for line in reader.lines() {
        let line: String = line?;
        // println!("{}", line);
        for caps in re.captures_iter(&line) {
            // Access capture groups
            let first_number = caps.get(1).map_or("", |m| m.as_str());
            let second_number = caps.get(2).map_or("", |m| m.as_str());

            matches.push((first_number.to_string(), second_number.to_string()));
        }
    }

    for (first, second) in &matches {
        println!("Match: mul({}, {})", first, second);
        result += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }

    println!("{}", result);
    Ok(())
}
