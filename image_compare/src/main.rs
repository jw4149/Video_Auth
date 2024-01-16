use image::{open, GenericImageView};
use image::{DynamicImage, ImageBuffer, Rgba};
use plotters::prelude::*;
use std::error::Error;
use std::process;
use std::fs::File;
use std::io::{Result, Write};


fn main() -> Result<()> {

    for i in 10..20 {
        let original_file_path = format!("output_original_frames/frame{}.png", i);
        let output_file_path = format!("output_frames_from_lossless_encoding/frame{}.png", i);
        let container1 = load_image(&original_file_path);
        let container2 = load_image(&output_file_path);

        // Print the vector
        println!("width, height of the first image {:?}, {:?}", container1.width, container1.height);
        println!("width, height of the second image {:?}, {:?}", container2.width, container2.height);

        if container1.width != container2.width || container1.height != container2.height {
            println!("The dimension of two images are not the same, cannot build historgram");
            process::exit(1);
        }

        let delta_vec = compare_diff_abs(container1.data.as_ref(), container2.data.as_ref());

        // Split the data into four channels
        let mut rgba_channels = (Vec::new(), Vec::new(), Vec::new(), Vec::new());

        let output_vec_file = format!("diff_vecs/diff_{}.txt", i);
        write_pixel_data_to_file(&delta_vec, &output_vec_file);

        for [c1, c2, c3, c4] in delta_vec {
            rgba_channels.0.push(c1);
            rgba_channels.1.push(c2);
            rgba_channels.2.push(c3);
            rgba_channels.3.push(c4);
        }

        let diff_r_max = *rgba_channels.0.iter().max().unwrap() as i32;
        let diff_g_max = *rgba_channels.1.iter().max().unwrap() as i32;
        let diff_b_max = *rgba_channels.2.iter().max().unwrap() as i32;
        let diff_a_max = *rgba_channels.3.iter().max().unwrap() as i32;

        // debug log 
        println!("max diff for r,g,b,a channels {:?}, {:?}, {:?}, {:?}", diff_r_max, diff_g_max, diff_b_max, diff_a_max);
    }

    Ok(())

}

fn compare_diff_abs(a: &Vec<[u8; 4]>, b: &Vec<[u8; 4]>) -> Vec<[i16;4]> {
   a.iter().zip(b.iter()).map(|(a_elem, b_elem)| {
       let mut diff = [0i16; 4];
       for i in 0..4 {
           diff[i] = (a_elem[i] as i16 - b_elem[i] as i16).abs();
       }
       diff
   }).collect()
}

fn load_image(path: &str) -> ImageContainer {
    let img = open(path).unwrap();
    let (width, height) = img.dimensions();

    // Convert image data to a vector of pixels
    // read data column-by-column
    let mut img_vec = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y).0; // Get the pixel value as RGBA
            if x >= 500 && x < 600 && y >= 500 && y < 600 {
                pixel = [0u8; 4];
            }
            img_vec.push(pixel);
        }
    }

    let ret = ImageContainer {
        width: width,
        height: height,
        data: img_vec,
    };
    ret
}

fn write_pixel_data_to_file(data: &Vec<[i16; 4]>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();

    for arr in data {
        let line = format!("{:?}\n", arr);
        file.write_all(line.as_bytes()).unwrap();
    }

    println!("Data has been written to {}", file_path);
}

struct ImageContainer {
    width: u32,
    height: u32,
    data: Vec<[u8; 4]>,
}