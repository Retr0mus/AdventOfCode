use image::{GrayImage, Luma, Rgb, RgbImage};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::cmp::min;
use std::cmp::max;

const SIZE: usize = 100_000;

/// Maps matrix values to RGB colors
fn value_to_color(value: u8) -> [u8; 3] {
    match value {
        0 => [255, 255, 255], // white
        1 => [0, 0, 0],       // black
        2 => [0, 0, 255],     // blue
        3 => [0, 255, 0],     // green
        _ => [128, 128, 128], // gray for unknown values
    }
}

/// Priority for color selection when scaling (higher = more important)
/// Ensures colored pixels aren't lost when scaling down
fn value_priority(value: u8) -> u8 {
    match value {
        1 => 4, // black  - highest priority
        2 => 3, // blue
        3 => 2, // green
        0 => 1, // white  - lowest priority
        _ => 0,
    }
}

/// Creates a scaled RGB image from a multi-value matrix.
/// When scaling, the highest priority color in each block wins.
fn create_scaled_image(matrix: &[Vec<u8>], scale_factor: usize) -> RgbImage {
    let height = matrix.len();
    let width = matrix.first().map_or(0, |row| row.len());

    let new_height = height.div_ceil(scale_factor);
    let new_width = width.div_ceil(scale_factor);

    // 3 bytes per pixel (RGB), initialized to white
    let mut pixels = vec![255u8; new_height * new_width * 3];

    pixels
        .par_chunks_mut(new_width * 3)
        .enumerate()
        .for_each(|(new_y, row)| {
            let y_start = new_y * scale_factor;
            let y_end = (y_start + scale_factor).min(height);

            for new_x in 0..new_width {
                let x_start = new_x * scale_factor;
                let x_end = (x_start + scale_factor).min(width);

                // Find highest priority value in the block
                let mut best_value = 0u8;
                let mut best_priority = 0u8;

                'block: for y in y_start..y_end {
                    for &val in &matrix[y][x_start..x_end] {
                        let pri = value_priority(val);
                        if pri > best_priority {
                            best_priority = pri;
                            best_value = val;
                            // Early exit if we found black (highest priority)
                            if val == 1 {
                                break 'block;
                            }
                        }
                    }
                }

                let color = value_to_color(best_value);
                let idx = new_x * 3;
                row[idx..idx + 3].copy_from_slice(&color);
            }
        });

    RgbImage::from_raw(new_width as u32, new_height as u32, pixels)
        .expect("Buffer size mismatch")
}


fn partial_flood_fill(matrix: &mut Vec<Vec<u8>>, initial_x: usize, initial_y: usize, target_colors: Vec<u8>, replacement_color: u8, start : bool) {
    
    let mut stack : Vec<(usize, usize)> = Vec::new();
    stack.push((initial_x, initial_y));

    while !stack.is_empty() {
        // Extract element from stack
        let Some((x, y)) = stack.pop() else { todo!() };

        // Check if the current position is valid
        if x >= matrix.len() || y >= matrix[0].len() {
            continue; // Out of bounds
        }

        let mut is_target : bool = false;

        for color in &target_colors {
            if matrix[x][y] == *color {
                is_target = true;
                break;
            }
        }
        
        // Check if the current cell is the target color and not the replacement color
        if !is_target || matrix[x][y] == replacement_color {
            continue; // Not the target color or already filled
        }

        // Replace color
        //println!("Cell coloured");
        matrix[x][y] = replacement_color;

        // Put the other directions in the stack
        if start {
            stack.push((x+1, y));                 // Down
            stack.push((x, y + 1));               // Right
        }
        else {
            stack.push((x.wrapping_sub(1), y));   // Up
            stack.push((x, y.wrapping_sub(1)));   // Left
        }
    }
}

fn draw_figure(vertexes : Vec<(i32, i32)>) -> Vec<Vec<u8>> {
    // Define matrix
    let mut figure: Vec<Vec<u8>> = vec![vec![0; SIZE]; SIZE];

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
    partial_flood_fill(&mut figure, 0, 0, [0].to_vec(), 2, true);
    partial_flood_fill(&mut figure, SIZE-1, SIZE-1, [0,2].to_vec(), 3, false);
    println!("Area drawed!");
    //print_matrix(&figure);

    figure
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let matrix = draw_figure(red_tiles.clone());


    // Scale down by 100x → 1000×1000 output image
    let scale_factor = 100;
    
    println!("Creating scaled image...");
    let img = create_scaled_image(&matrix, scale_factor);
    
    println!("Saving image...");
    img.save("output.png")?;
    
    println!("Done! Output size: {}x{}", img.width(), img.height());
    Ok(())
}


// claude-opus-4-5-20251101-thinking-32k

// The user has a very large binary matrix (100000x100000) in Rust and wants to:

//     Create an image from it where 1 = black, 0 = white
//     Scale the image down without losing black pixels (meaning if any pixel in a region is black, the scaled-down pixel should remain black)

// Let me think about the challenges:

//     Memory: A 100000x100000 matrix of u8 is 10 billion bytes = ~10 GB. Even storing it as bits would be ~1.25 GB. Creating an image of this size is not practical.

//     Scaling without losing black pixels: This is essentially a "max pooling" or "any black in region" approach. When downscaling, if any source pixel in a block is black (1), the destination pixel should be black.

// Let me write a solution:

//     First, I'll show how to handle the scaling with the "preserve black" logic
//     Use the image crate for image generation
//     Consider memory efficiency

// For the scaling, if we want to scale by factor N (meaning output is 100000/N x 100000/N), for each output pixel we check if any of the NxN input pixels is 1.

// Let me write the code:
// rust

// use image::{GrayImage, Luma};

// fn create_scaled_image(matrix: &Vec<Vec<u8>>, scale_factor: usize) -> GrayImage {
//     let height = matrix.len();
//     let width = if height > 0 { matrix[0].len() } else { 0 };
    
//     let new_height = (height + scale_factor - 1) / scale_factor; // ceiling division
//     let new_width = (width + scale_factor - 1) / scale_factor;
    
//     let mut img = GrayImage::new(new_width as u32, new_height as u32);
    
//     for new_y in 0..new_height {
//         for new_x in 0..new_width {
//             // Check if any pixel in the block is 1 (black)
//             let mut is_black = false;
            
//             for dy in 0..scale_factor {
//                 let y = new_y * scale_factor + dy;
//                 if y >= height { break; }
                
//                 for dx in 0..scale_factor {
//                     let x = new_x * scale_factor + dx;
//                     if x >= width { break; }
                    
//                     if matrix[y][x] == 1 {
//                         is_black = true;
//                         break;
//                     }
//                 }
//                 if is_black { break; }
//             }
            
//             let pixel_value = if is_black { 0 } else { 255 };
//             img.put_pixel(new_x as u32, new_y as u32, Luma([pixel_value]));
//         }
//     }
    
//     img
// }

// For each output pixel, I iterate through the corresponding block in the input matrix and check if any value is 1. If found, the output pixel becomes black (0); otherwise it's white (255). The nested loops break early once a black pixel is detected to optimize performance.

// However, with a 100000x100000 matrix, this approach will still be slow. I could leverage parallel processing with rayon to distribute the work across multiple threads, or explore SIMD operations to check multiple values simultaneously.

// For the output, PNG supports 1-bit images which would significantly reduce file size. Even after scaling down by 100x, a 1000x1000 image is manageable.

// Let me implement a parallel version using rayon to handle the scaling more efficiently:
// rust

// use image::{GrayImage, Luma};
// use rayon::prelude::*;

// fn create_scaled_image_parallel(matrix: &[Vec<u8>], scale_factor: usize) -> GrayImage {
//     let height = matrix.len();
//     let width = if height > 0 { matrix[0].len() } else { 0 };
    
//     let new_height = height.div_ceil(scale_factor);
//     let new_width = width.div_ceil(scale_factor);

// The approach processes each scaled pixel in parallel by checking if any pixel within the corresponding block is black (value 1), then mapping that to the output image. .len() } else { 0 };

// let new_height = (height + scale_factor - 1) / scale_factor; let new_width = (width + scale_factor - 1) / scale_factor;

// // Create pixel data in parallel by row
// let pixels: Vec<u8> = (0..new_height)
//     .into_par_iter()
//     .flat_map_iter(|new_y| {
//         (0..new_width).map(move |new_x| {
//             let y_start = new_y * scale_factor;
//             let y_end = (y_start + scale_factor).min(height);
//             let x_start = new_x * scale_factor;
//             let x_end = (x_start + scale_factor).min(width);
            
//             // Scan the block for any black pixels
//             (y_start..y_end).any(|y| {
//                 matrix[y][x_start..x_end].iter().any(|&p| p == 1)
//             }).then_some(0u8).unwrap_or(255u8)
//         })
//     })
//     .collect();

// GrayImage::from_raw(new_width as u32, new_height as u32, pixels)
//     .expect("Failed to create image")

// }


// I should refactor this with rayon more carefully to avoid unnecessary complexity. Let me simplify the approach:

// ```rust
// use image::GrayImage;
// use rayon::prelude::*;

// fn create_scaled_image(matrix: &[Vec<u8>], scale_factor: usize) -> GrayImage {
//     let height = matrix.len();
//     let width = if height > 0 { matrix[0].len() } else { 0 };
    
//     let new_height = (height + scale_factor - 1) / scale_factor;
//     let new_width = (width + scale_factor - 1) / scale_factor;
    
//     let pixels: Vec<u8> = (0..new_height)
//         .into_par_iter()
//         .flat_map_iter(|new_y| {
//             (0..new_width).map(move |new_x| {
//                 let y_range = (new_y * scale_factor)..((new_y + 1) * scale_factor).min(height);
//                 let x_range = (new_x * scale_factor)..((new_x + 1) * scale_factor).min(width);
                
//                 y_range.any(|y| matrix[y][x_range.clone()].iter().any(|&p| p == 1))
//                     .then_some(0u8)
//                     .unwrap_or(255u8)
//             })
//         })
//         .collect();
    
//     GrayImage::from_raw(new_width as u32, new_height as u32, pixels)
//         .expect("Failed to create image")
// }
// ``` the block is 1, then set the downsampled pixel to black (0), otherwise white (255). This uses `any()` to efficiently check if any pixel in the scale_factor × scale_factor block equals 1, avoiding unnecessary iterations.

// For very large datasets, I could explore memory-mapped files to avoid loading everything at once, or use a streaming approach that processes data incrementally. The `bitvec` crate might also provide more efficient storage than standard vectors. When saving the result, using PNG with 1-bit color depth would minimize file size.*

