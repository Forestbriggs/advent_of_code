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

        let res: bool = safe_correctable(&num_parts);

        if res {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
    Ok(())
}

fn safe_report(report: &[i32]) -> bool {
    report.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3)
        || report.is_sorted_by(|a, b| a < b && b.abs_diff(*a) <= 3)
}

fn safe_correctable(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut copy = report.to_vec();
        copy.remove(i);
        if safe_report(&copy) {
            return true;
        }
    }

    return false;
}
