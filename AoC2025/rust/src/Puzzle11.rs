use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fmt::Display;

fn print_matrix<T: Display>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for value in row {
            print!("{} ", value); // Print each value followed by a space
        }
        println!(); // Move to the next line after printing a row
    }
}

fn extract_column(matrix: &[Vec<String>], col_index: usize) -> Vec<String> {
    let mut column: Vec<String> = matrix
        .iter()
        .filter_map(|row| row.get(col_index).map(|s| s.to_owned()))
        .collect();

    // If you want to remove the last element (the operator) before returning
    if column.len() > 0 {
        column.pop(); // Removes last element
    }
    
    column // Return the modified column
}


fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day6.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Define the matrix and utility variables
    let mut matrix : Vec<Vec<String>> = vec![];
    let mut accumulator : i64 = 0;
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                matrix.push(content.split_whitespace().map(|s| s.to_string()).collect());
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let row = &matrix[0];

    for j in 0..row.len() {
        let operator = match matrix[matrix.len() - 1][j].chars().next() {
            Some(c) => c,
            None => ' ',
        };
        let column: Vec<i64> = extract_column(&matrix, j)
            .iter() 
            .map(|s| s.parse::<i64>())
            .filter_map(Result::ok)
            .collect();
        let mut column_result : i64 = 0;

        match operator {
            '+' => {column_result = column.iter().fold(0, |result, &x| result + x);}
             _  => {column_result = column.iter().fold(1, |result, &x| result * x);}
        }

        accumulator += column_result;
    }

    println!("Result: {}", accumulator);    // 5552221122013

    Ok(())
}
