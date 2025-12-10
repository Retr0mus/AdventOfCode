use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

fn area(v1 : (i32, i32), v2 : (i32, i32)) -> i64 {
    let (x1, y1) = v1;
    let (x2, y2) = v2;

    ((x1 as i64 - x2 as i64).abs() + 1)*((y1 as i64 - y2 as i64).abs() + 1)
}

fn find_max_area(vertexes : Vec<(i32, i32)>) -> ((i32,i32),(i32,i32)) {
    let mut max_area = 0;
    let mut vs : ((i32,i32),(i32,i32)) = ((0,0),(0,0));
    for i in 0..vertexes.len()-1 {
        for j in i+1..vertexes.len() {
            let current_area = area(vertexes[i], vertexes[j]);
            if max_area < current_area {
                max_area = current_area;
                vs = (vertexes[i],vertexes[j]);
            }
        }
    }
    vs
}

fn main() -> io::Result<()> {
    let path = Path::new("data/day9.txt"); // Ensure this file exists in your project directory
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut red_tiles : Vec<(i32, i32)> = Vec::new();
    
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line
        match line {
            Ok(content) => {
                let pair : Vec<i32> = content
                    .split(',')
                    .map(|x| x.parse::<i32>().expect("invalid number"))
                    .collect();
                red_tiles.push((pair[0], pair[1]));
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let greatest_rectangle = find_max_area(red_tiles);

    println!("Rectangle: ({},{}) - ({},{})", greatest_rectangle.0.0, greatest_rectangle.0.1, greatest_rectangle.1.0, greatest_rectangle.1.1);
    println!("Area: {}", area(greatest_rectangle.0, greatest_rectangle.1));    // 4760959496

    Ok(())
}
