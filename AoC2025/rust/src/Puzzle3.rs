use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

fn validity_check(number : i64) -> bool{
    let lex = number.to_string();

    let (first_half, second_half) = lex.split_at(lex.len() / 2);
    if first_half == second_half {
        return false;
    }

    return true;
}

/// Adds the invalid numbers in a range.
fn invalidity_check_range(start : i64, end : i64) -> i64 {
    let mut accumulator : i64 = 0;

    for number in start..(end+1) {
        if !validity_check(number)  {
            accumulator += number;
        }
    }

    return accumulator;
}

fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day2.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Define an accumulator
    let mut accumulator = 0;
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                for range in content.split(',') {
                    let parts : Vec<i64> = range
                        .split('-')
                        .map(|s| s.parse().expect("Failed to parse integer"))
                        .collect();
                    
                    let (start, end) : (i64, i64) = (parts[0], parts[1]);

                    accumulator += invalidity_check_range(start, end);
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("Result: {}", accumulator);    // 41294979841

    Ok(())
}
