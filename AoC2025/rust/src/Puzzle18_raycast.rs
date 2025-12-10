use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fmt::Display;
use std::cmp::min;
use std::cmp::max;

const SIZE: usize = 98_416;   // 98_416

fn print_matrix<T: Display>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for value in row {
            print!("{} ", value); // Print each value followed by a space
        }
        println!(); // Move to the next line after printing a row
    }
}

fn area(v1 : (i32, i32), v2 : (i32, i32)) -> i64 {
    let (x1, y1) = v1;
    let (x2, y2) = v2;

    ((x1 as i64 - x2 as i64).abs() + 1)*((y1 as i64 - y2 as i64).abs() + 1)
}

fn generate_line(v1: (i32, i32), v2: (i32, i32)) -> Vec<(i32, i32)> {
    // Check if the first element is the same (x-coordinate)
    if v1.0 == v2.0 {
        // Generate points along the y-axis
        let x = v1.0;
        let y1 = v1.1;
        let y2 = v2.1;
        let start_y = y1.min(y2);
        let end_y = y1.max(y2);

        return (start_y..=end_y).map(|y| (x, y)).collect();
    }
    // The second element must be the same (y-coordinate)
    else {
        // Generate points along the x-axis
        let y = v1.1; // or v2.1
        let x1 = v1.0;
        let x2 = v2.0;
        let start_x = x1.min(x2);
        let end_x = x1.max(x2);

        return (start_x..=end_x).map(|x| (x, y)).collect();
    }
}

// Ray cast implementation
fn ray_cast_inside(figure : &Vec<Vec<i8>>, point : (usize, usize)) -> bool {
    
    let mut collisions = 0;
    let mut is_perimeter = false;

    // Edge checking
    if figure[point.0][point.1] == 1 {
        return true;
    }

    // Inside check
    if point.0 < point.1 {
        for i in 0..=point.0 {
            if !is_perimeter {
                collisions += figure[i][point.1];
                if figure[i][point.1] == 1 {
                    is_perimeter = true;
                }
            }
            else {
                if figure[i][point.1] == 0 {
                    is_perimeter = false;
                }
            }
        }
    }
    else {
        for j in 0..=point.1 {
            if !is_perimeter {
                collisions += figure[point.0][j];
                if figure[point.0][j] == 1 {
                    is_perimeter = true;
                }
            }
            else {
                if figure[point.0][j] == 0 {
                    is_perimeter = false;
                }
            }
        }
    }

    collisions % 2 == 1
}


fn line_inside(figure : &Vec<Vec<i8>>, line : &Vec<(i32,i32)>) -> bool {
    let result : bool = line.into_iter().map(|(x,y)| ray_cast_inside(figure, (*x as usize, *y as usize))).fold(true, |x, acc| x && acc);
    result
}

fn lines_inside(figure : &Vec<Vec<i8>>, v1 : (i32, i32), v2 : (i32, i32)) -> bool {
    let mut lines : Vec<Vec<(i32,i32)>> = Vec::new();
    lines.push(generate_line(v1, (v1.0, v2.1)));
    lines.push(generate_line(v1, (v2.0, v1.1)));
    lines.push(generate_line((v1.0, v2.1), v2));
    lines.push(generate_line((v2.0, v1.1), v2));

    for line in lines {
        if !line_inside(figure, &line) {
            return false;
        }
    }

    true
}

// Cannot flood_fill, Memory error
// fn flood_fill(matrix: &mut Vec<Vec<i8>>, initial_x: usize, initial_y: usize, target_color: i8, replacement_color: i8) {
    
//     let mut stack : Vec<(usize, usize)> = Vec::new();
//     stack.push((initial_x, initial_y));

//     while !stack.is_empty() {
//         // Extract element from stack
//         let Some((x, y)) = stack.pop() else { todo!() };

//         // Check if the current position is valid
//         if x >= matrix.len() || y >= matrix[0].len() {
//             continue; // Out of bounds
//         }
        
//         // Check if the current cell is the target color and not the replacement color
//         if matrix[x][y] != target_color || matrix[x][y] == replacement_color {
//             continue; // Not the target color or already filled
//         }

//         // Replace color
//         //println!("Cell coloured");
//         matrix[x][y] = replacement_color;

//         // Put the other 4 directions in the stack
//         stack.push((x+1, y));                 // Down
//         stack.push((x.wrapping_sub(1), y));   // Up
//         stack.push((x, y + 1));               // Right
//         stack.push((x, y.wrapping_sub(1)));    // Left
//     }
// }


fn draw_figure(vertexes : Vec<(i32, i32)>) -> Vec<Vec<i8>> {
    // Define matrix
    let mut figure: Vec<Vec<i8>> = vec![vec![0; SIZE]; SIZE];

    let mut v = vertexes.clone();
    v.push(v[0]);   // Close figure

    // Draw perimeter
    for i in 0..v.len()-1 {
        let v1 = v[i];
        let v2 = v[i+1];

        if v1.0 == v2.0 {
            let min = min(v1.1, v2.1) as usize;
            let max = max(v1.1, v2.1) as usize;
            for j in min..=max {
                figure[v1.0 as usize][j] = 1;
            }
        }
        else {
            let min = min(v1.0, v2.0) as usize;
            let max = max(v1.0, v2.0) as usize;
            for j in min..=max {
                figure[j][v1.1 as usize] = 1;
            }
        }
    }
    println!("Perimeter drawed!");
    //print_matrix(&figure);

    // Fill the figure's area (inside and outside impossible with my HW)
    //let starting_point : (usize, usize) = (0,0);      // +1,+1 for test input, +1,-1 for real one inside.
    //flood_fill(&mut figure, starting_point.0, starting_point.1, 0, 2);
    //println!("Area drawed!");
    //print_matrix(&figure);

    figure
}

fn find_max_internal_rectangle_area(vertexes : Vec<(i32, i32)>) -> i64 {
    let mut max_area = 0;
    let figure = draw_figure(vertexes.clone());

    for i in 0..vertexes.len()-1 {
        for j in i+1..vertexes.len() {
            let current_area = area(vertexes[i], vertexes[j]);
            //println!("{}", current_area);
            let is_valid = lines_inside(&figure, vertexes[i], vertexes[j]);
            if max_area < current_area && is_valid {
                max_area = current_area;
                //println!("New max_area: {}", max_area);
            }
        }
    }
    max_area
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

    let area = find_max_internal_rectangle_area(red_tiles);

    println!("Result: {}", area);    // Most near result 3001706952

    Ok(())
}
