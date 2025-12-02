use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/input.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let mut file = File::open(path)?;

    // Create a string to hold the contents
    let mut contents = String::new();

    // Read the file contents into the string
    file.read_to_string(&mut contents)?;

    // Print the contents
    println!("File Contents:\n{}", contents);

    Ok(())
}
