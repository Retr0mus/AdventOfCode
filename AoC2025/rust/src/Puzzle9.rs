use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

/// Defining IO state machine
#[derive(Debug)]
enum State {
    Ranges,
    Identifiers,
}

struct StateMachine {
    current_state: State,
    valid_ranges : Vec<(i64, i64)>,
    ids_to_validate : Vec<i64>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: State::Ranges,
            valid_ranges: Vec::new(),
            ids_to_validate: Vec::new(),
        }
    }

    fn transition(&mut self, input: String) {
        match self.current_state {
            
            State::Ranges => {
                match input.trim() {
                    
                    range if range.contains('-') => {
                        let parts : Vec<i64> = range
                        .split('-')
                        .map(|s| s.parse().expect("Failed to parse integer"))
                        .collect();

                    let (start, end) : (i64, i64) = (parts[0], parts[1]);
                    self.valid_ranges.push((start, end));
                    
                    self.current_state = State::Ranges;
                    }
                    _ => {self.current_state = State::Identifiers;}
                }
            }

            State::Identifiers => {
                let num: i64 = input.parse().expect("Failed to parse integer");
                self.ids_to_validate.push(num);
                self.current_state = State::Identifiers;
            }
        }
    }

    fn current(&self) -> &State {
        &self.current_state
    }
}



fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day5.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Initialize the state machine to read the input file
    let mut sm = StateMachine::new();

    // Define an accumulator
    let mut accumulator = 0;

    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                sm.transition(content);
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    // Counting the valid ids over all the availables
    for id in sm.ids_to_validate {
        for range in &sm.valid_ranges {
            if id >= range.0 && id <= range.1 {
                accumulator += 1;
                break;
            }
        }
    }

    println!("Fresh IDs: {}", accumulator);    // 773

    Ok(())
}
