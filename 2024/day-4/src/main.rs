use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn main() -> Result<(), Error> {
    let f: File = File::open("./src/input.txt")?;
    let reader = BufReader::new(f);

    let mut word_search: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        let raw_letter_data: Vec<&str> = line.split("").collect();
        let letters: Vec<String> = raw_letter_data
            .iter()
            .filter_map(|s| Some(s.to_string()))
            .collect();
        // println!("{:?}", letters);
        word_search.push(letters);
    }

    let mut total: i32 = 0;

    for (i, line) in word_search.iter().enumerate() {
        // println!("{:?}", line);
        for (j, c) in line.iter().enumerate() {
            // println!("{}", c);
            if c == "X" {
                println!("{}", c);
                println!("{},{}", i, j);
                let num_valid = check_siblings(word_search.clone(), (i, j));
                total += num_valid;
            }
        }
    }

    println!("{}", total);

    Ok(())
}

fn check_siblings(search: Vec<Vec<String>>, loc: (usize, usize)) -> i32 {
    let mut curr_valid: i32 = 0;

    for (dr, dc) in &[
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Top-left
        (-1, 1),  // Top-right
        (1, -1),  // Bottom-left
        (1, 1),   // Bottom-right
    ] {
        let new_row = loc.0 as isize + dr;
        let new_col = loc.1 as isize + dc;
        // println!("{},{},{},{}", loc.0, loc.1, new_row, new_col);
        let r = &search.get(new_row as usize);
        let mut neighbor = None;

        if let Some(vec) = r {
            if let Some(val) = vec.get(new_col as usize) {
                neighbor = Some(val);
            }
        }
        if neighbor.is_some() {
            // println!("{:?}", neighbor.unwrap());
            if neighbor.unwrap() == "M" {
                println!("{:?}", neighbor);
                println!("{},{}", new_row, new_col);
                let a_valid = check_direction(
                    search.clone(),
                    (*dr, *dc),
                    (new_row as usize, new_col as usize),
                    "A",
                );

                if a_valid {
                    let s_valid = check_direction(
                        search.clone(),
                        (*dr, *dc),
                        ((new_row + dr) as usize, (new_col + dc) as usize),
                        "S",
                    );

                    if s_valid {
                        curr_valid += 1;
                    }
                }
            }
        }
    }

    return curr_valid;
}

fn check_direction(
    search: Vec<Vec<String>>,
    dir: (isize, isize),
    loc: (usize, usize),
    letter_to_check: &str,
) -> bool {
    if loc.0 as isize + dir.0 < 0 || loc.1 as isize + dir.1 < 0 {
        return false; // Prevent underflow
    }

    let new_row = (loc.0 as isize + dir.0) as usize;
    let new_col = (loc.1 as isize + dir.1) as usize;

    if new_row >= search.len() || new_col >= search[0].len() {
        return false; // Ensure bounds
    }

    if let Some(neighbor) = search.get(new_row).and_then(|vec| vec.get(new_col)) {
        return neighbor == letter_to_check;
    }
    false
}
