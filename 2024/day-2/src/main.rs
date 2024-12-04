use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    // * Open input text file and initialize a new BufReader
    let f: File = File::open("./src/input.txt")?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut safe_reports: i32 = 0;

    for line in reader.lines() {
        let line: String = line?;
        let string_parts: Vec<&str> = line.split_whitespace().collect();
        let num_parts: Vec<i32> = string_parts
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let res: bool;

        if num_parts[0] > num_parts[1] {
            res = validate_decreasing_level(num_parts);
        } else if num_parts[0] < num_parts[1] {
            res = validate_increasing_level(num_parts);
        } else {
            res = false;
        }

        if res {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
    Ok(())
}

fn validate_increasing_level(num_parts: Vec<i32>) -> bool {
    for i in 0..num_parts.len() - 1 {
        let l: i32 = num_parts[i];
        let r: i32 = num_parts[i + 1];

        if r - l < 1 || r - l > 3 {
            return false;
        }
    }

    return true;
}

fn validate_decreasing_level(num_parts: Vec<i32>) -> bool {
    for i in 0..num_parts.len() - 1 {
        let l: i32 = num_parts[i];
        let r: i32 = num_parts[i + 1];

        if l - r < 1 || l - r > 3 {
            return false;
        }
    }

    return true;
}
