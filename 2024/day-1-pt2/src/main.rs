use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    // * Open input text file and initialize a new BufReader
    let f: File = File::open("./src/input.txt")?;
    let reader: BufReader<File> = BufReader::new(f);

    // * Initialize vector to hold left and right side input from text file
    // * for later comparing
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // * Initialize to keep track of total distance between left and right list
    let mut similarity_score: i32 = 0;

    // * Loop through each line in text file
    // * Split each line and if it has a valid number push the
    // * left and right numbers into their respective vectors
    for line in reader.lines() {
        let line: String = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(left_num) = parts[0].parse::<i32>() {
                left.push(left_num);
            }
            if let Ok(right_num) = parts[1].parse::<i32>() {
                right.push(right_num)
            }
        }
    }

    let l: usize = right.len();

    for left_num in left {
        let mut score: i32 = 0;
        for i in 0..l {
            let right_num = right[i];
            if left_num == right_num {
                score += 1;
            }
        }
        let sim_val = left_num * score;
        similarity_score += sim_val;
    }

    println!("{}", similarity_score);

    Ok(())
}
