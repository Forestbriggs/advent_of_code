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
    let mut total_distance: i32 = 0;

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
        // println!("Left: {:?}", left);
        // println!("Right: {:?}", right);
    }

    // * Sort both the vectors from smallest to largest for processing
    left.sort();
    right.sort();
    println!("Sorted Left: {:?}", left);
    println!("Sorted Right: {:?}", right);

    // * Check that both vectors are the same length
    // * storing left's length for looping later
    // ! With a set input the assertion should never be false,
    // ! if it is that means there was an error within our reader.
    let l: usize = left.len();
    assert_eq!(right.len(), l);

    // * compare the distance between each number from smallest to largest
    // * appending this distance to total distance on each iteration
    for i in 0..l {
        let left_num: i32 = left[i];
        let right_num: i32 = right[i];

        let distance_between: i32 = left_num - right_num;
        total_distance += distance_between;
        // println!("{}", total_distance);
    }

    println!("{}", total_distance);
    Ok(())
}
