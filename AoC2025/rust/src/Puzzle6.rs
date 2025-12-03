use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

/// Finds the position of the highest element in the vector
fn find_max_position(v : Vec<u8>) -> usize {

    let mut max_pos : usize = 0;

    for i in 1..v.len() {
        if v[max_pos] < v[i] {
            max_pos = i;
        }
    }

    return max_pos;
}

/// Calculates maximum joltage for the line
fn extract_max_joltage(battery : String) -> i64 {
    
    let digits = 12;
    let mut joltage : i64 = 0;
    let mut digit_pos : usize = 0;

    let battery_vec : Vec<u8> = battery
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    for i in 1..=digits {
        let feasible_region = battery_vec[digit_pos..(battery_vec.len()-digits+i)].to_vec();
        digit_pos += find_max_position(feasible_region);
        joltage += (battery_vec[digit_pos] as i64) * 10_i64.pow((digits-i) as u32);
        digit_pos += 1;
    }

    return joltage;
}

fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day3.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Create accumulator and counter
    let mut accumulator : i64 = 0;
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                accumulator += extract_max_joltage(content);
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("Result: {}", accumulator);    // 168575096286051

    Ok(())
}