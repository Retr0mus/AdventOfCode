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


fn get_value(matrix: &Vec<Vec<u8>>, row: isize, col: isize) -> u8 {
    if row < 0 || col < 0 || (row as usize) >= matrix.len() || (col as usize) >= matrix[row as usize].len() {
        0
    } else {
        matrix[row as usize][col as usize]
    }
}


fn adjacency_counter(matrix : &Vec<Vec<u8>>, row : usize, column : usize) -> u8 {
    
    let mut adjacency : u8 = 0;

    for i in -1isize..=1 {
        for j in -1isize..=1 {
            adjacency += get_value(&matrix, row as isize + i, column as isize + j);
        }
    }

    adjacency-1     // Removing same element, considering that it is always called upon a @
}

fn remove_rolls(matrix: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 1 {
                // Check condition on adjacency_counter
                if adjacency_counter(matrix, i, j) < 4 {
                    // Removing roll of paper
                    matrix[i][j] = 0;
                }
            }
        }
    }
    matrix.clone()
}

fn total_matrix_sum(matrix : &Vec<Vec<u8>>) -> i32 {

    let mut accumulator : i32 = 0;
    
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            accumulator += matrix[i][j] as i32;
        }
    }

    accumulator
}

fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day4.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Define the matrix and utility variables
    let mut matrix : Vec<Vec<u8>> = vec![];
    let mut removed_rolls : i32 = 0;
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                matrix.push(content
                    .chars()
                    .map(|c| if c == '@' { 1 as u8 } else { 0 as u8 })
                    .collect());
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let initial_rolls :i32 = total_matrix_sum(&matrix);

    loop {
        let matrix = remove_rolls(&mut matrix);
        let current_rolls : i32 = total_matrix_sum(&matrix);

        if initial_rolls - removed_rolls == current_rolls {
            break;
        }

        removed_rolls = initial_rolls - current_rolls;
    }

    println!("Result: {}", removed_rolls);    // 9000

    Ok(())
}
