use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day1.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Create accumulator and counter
    let mut acc : i16 = 50;
    let mut cnt : i16 = 0;
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                let direction : char = content[0..1].parse().expect("Failed to parse value");
                let mut value : i16 = content[1..].parse().expect("Failed to parse value");

                match direction {
                    'L' => {value = -value;},
                    _ => {},
                }
                
                acc = (acc + value).rem_euclid(100);

                if acc == 0 {
                    cnt = cnt + 1;
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("Result: {}", cnt);    // 1066

    Ok(())
}
