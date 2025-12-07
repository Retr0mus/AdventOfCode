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
    tachyons : HashSet<u8>,
    splits : u32,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: State::SourceExtraction,
            tachyons: HashSet::new(),
            splits: 0,
        }
    }

    fn transition(&mut self, line: &Vec<u8>) {
        match self.current_state {
            
            State::SourceExtraction => {
                for i in 0..line.len() {
                    if line[i] == 9 {
                        self.tachyons.insert(i as u8);
                        break;
                    }
                }
                self.current_state = State::Refraction;
            }

            State::Refraction => {
                let mut next_tachyons : HashSet<u8> = HashSet::new();
                for beam in self.tachyons.clone() {
                    if line[beam as usize] == 1 {
                        next_tachyons.insert(beam-1);
                        next_tachyons.insert(beam+1);
                        self.splits += 1;
                    }
                    else {
                        next_tachyons.insert(beam);
                    }
                }
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

    println!("Result: {}", refractor.splits);    // 1585

    Ok(())
}
