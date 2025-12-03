use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

/// Checks if a vector contains always the same value
fn contains_single_value<T: PartialEq>(vec: &[T]) -> bool {
    if vec.is_empty() {
        return false; // Return false for empty vector
    }
    
    // Get the first element and check if all elements are equal to the first
    let first = &vec[0];
    return vec.iter().all(|item| item == first);
}

/// Finds the divisors of a given number
fn find_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as usize;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {  // Avoid adding the square root twice
                divisors.push(n / i);
            }
        }
    }

    divisors.sort();
    return divisors;
}

/// Checks if a number is valid
fn validity_check(number : i64) -> bool {
    let char_vec : Vec<char> = number.to_string().chars().collect();

    let mut divisors = find_divisors(char_vec.len());
    divisors.pop(); // Last divisor is removed to exclude the entire string case.

    for n in divisors{

        let strings_of_length_n: Vec<String> = char_vec
            .chunks(n)
             .map(|chunk| chunk.iter().collect())
             .collect();

        if contains_single_value(&strings_of_length_n) {
            return false;
        }
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

    println!("Result: {}", accumulator);    // 66500947346

    Ok(())
}
