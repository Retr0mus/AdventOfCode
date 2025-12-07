use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::collections::HashSet;



#[derive(Debug)]
enum State {
    SourceExtraction,
    Refraction,
}

struct StateMachine {
    current_state : State,
    paths : HashSet<Vec<u8>>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: State::SourceExtraction,
            paths: HashSet::new(),
        }
    }

    fn transition(&mut self, line: &Vec<u8>) {
        match self.current_state {
            
            State::SourceExtraction => {
                for i in 0..line.len() {
                    if line[i] == 9 {
                        let mut first_path = Vec::new();
                        first_path.push(i as u8);
                        self.paths.insert(first_path);
                        break;
                    }
                }
                self.current_state = State::Refraction;
            }

            State::Refraction => {
                let mut next_paths : HashSet<Vec<u8>> = HashSet::new();

                for path in self.paths.clone() {
                    if line[path[path.len()-1] as usize] == 1 {
                        let mut new_path_low = path.clone();
                        new_path_low.push(path[path.len() - 1] - 1);
                        next_paths.insert(new_path_low);

                        let mut new_path_high = path.clone();
                        new_path_high.push(path[path.len() - 1] + 1);
                        next_paths.insert(new_path_high);
                    }
                    else {
                        next_paths.insert(path.clone());
                    }
                }
                self.paths = next_paths;
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Specify the path to your file
    let path = Path::new("data/day7.txt"); // Ensure this file exists in your project directory
    
    // Open the file
    let file = File::open(path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Define the state machine
    let mut refractor = StateMachine::new();
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                let parsed_content : Vec<u8> = content
                    .chars()
                    .filter_map(|c| match c {
                        'S' => Some(9 as u8),
                        '^' => Some(1 as u8),
                         _  => Some(0 as u8),
                    })
                .collect();
                refractor.transition(&parsed_content);
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("Result: {}", refractor.paths.len());    // 

    Ok(())
}
