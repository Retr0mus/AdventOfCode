use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

fn find_max_position(v : Vec<u8>) -> usize {

    let mut max_pos : usize = 0;

    for i in 1..v.len() {
        if v[max_pos] < v[i] {
            max_pos = i;
        }
    }

    return max_pos;
}

fn extract_max_joltage(battery : String) -> i32 {
    
    let mut joltage : i32 = 0;

    let battery_vec : Vec<u8> = battery
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect();

    // Finding first number
    let mut excluded_last_cell = battery_vec.repeat(1);
    excluded_last_cell.pop();
    let first_digit_pos = find_max_position(excluded_last_cell);
    joltage += (battery_vec[first_digit_pos] as i32)*10;

    // Finding second number
    let second_digit_vec = battery_vec[(first_digit_pos+1)..].to_vec();
    let second_digit_pos = find_max_position(second_digit_vec) + first_digit_pos + 1;
    joltage += battery_vec[second_digit_pos] as i32;

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
    let mut accumulator : i32 = 0;
    
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

    println!("Result: {}", accumulator);    // 17031

    Ok(())
}