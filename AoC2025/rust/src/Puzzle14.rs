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
    positions : HashSet<u8>,
    tachyons : Vec<u64>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: State::SourceExtraction,
            positions: HashSet::new(),
            tachyons: Vec::new(),
        }
    }

    fn transition(&mut self, line: &Vec<u8>) {
        match self.current_state {
            
            State::SourceExtraction => {
                for i in 0..line.len() {
                    if line[i] == 9 {
                        self.positions.insert(i as u8);
                        self.tachyons = vec![0; line.len()];
                        self.tachyons[i] = 1;
                        break;
                    }
                }
                self.current_state = State::Refraction;
            }

            State::Refraction => {
                let mut next_positions : HashSet<u8> = HashSet::new();
                let mut next_tachyons : Vec<u64> = vec![0; self.tachyons.len()];

                for beam in self.positions.clone() {
                    let position = beam as usize;

                    if line[position] == 1 {
                        // Update positions to check
                        next_positions.insert(beam-1);
                        next_positions.insert(beam+1);
                        
                        // Update path vector
                        next_tachyons[position-1] += self.tachyons[position];
                        next_tachyons[position+1] += self.tachyons[position];
                    }
                    else {
                        next_positions.insert(beam);
                        next_tachyons[position] += self.tachyons[position];
                    }
                }
                self.positions = next_positions;
                self.tachyons = next_tachyons;
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

    println!("Result: {}", refractor.tachyons.iter().fold(0, |acc, &x| acc + x));    // 

    Ok(())
}
