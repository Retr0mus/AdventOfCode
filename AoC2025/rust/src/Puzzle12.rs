use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fmt::Display;

#[derive(Debug)]
enum State {
    OpeartorExtraction,
    MatrixBuilding,
    Calculation,
}

struct StateMachine {
    current_state : State,
    operator : char,
    numbers : Vec<u16>,
    accumulator : u64,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: State::OpeartorExtraction,
            operator: '+',
            numbers: Vec::new(),
            accumulator: 0,
        }
    }

    fn transition(&mut self, row: &mut Vec<char>) {
        match self.current_state {
            
            State::OpeartorExtraction => {
                self.operator = row.pop().expect("REASON");

                self.numbers.push(
                    row.iter()
                        .collect::<String>()
                        .trim()
                        .parse::<u16>()
                        .expect("REASON")
                );

                self.current_state = State::MatrixBuilding;
            }

            State::MatrixBuilding => {

                if row.iter().collect::<String>().trim().is_empty() {
                    self.current_state = State::Calculation;
                    self.transition(row);
                    return;
                }

                row.pop();

                self.numbers.push(
                    row.iter()
                        .collect::<String>()
                        .trim()
                        .parse::<u16>()
                        .expect("REASON")
                );
            }

            State::Calculation => {
                let mut r : u64 = 0;
                
                match self.operator {
                    '+' => {r = self.numbers.iter().fold(0, |result, &x| result + x as u64);}
                    _  => {r = self.numbers.iter().fold(1, |result, &x| result * x as u64);}
                }

                self.accumulator += r as u64;
                self.numbers.clear();

                self.current_state = State::OpeartorExtraction;
            }
        }
    }
}

fn transpose<T: Default + Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut transposed = vec![vec![T::default(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j].clone();
        }
    }

    transposed
}


fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day6.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Define the matrix
    let mut matrix : Vec<Vec<char>> = vec![];
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                matrix.push(content.chars().collect());
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    // Transpose the matrix for semplicity
    let mut transposed = transpose(&matrix);
    transposed.push(vec![]);                    // Adds an empty line at the end to trigger last calculation

    // Iterate over rows with the state machine
    let mut calculator = StateMachine::new();

    for mut row in transposed {
        calculator.transition(&mut row);
    }
    
    println!("Result: {}", calculator.accumulator);    // 11371597126232

    Ok(())
}
