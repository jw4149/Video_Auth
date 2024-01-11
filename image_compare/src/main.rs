use image::{open, GenericImageView};
use plotters::prelude::*;
use std::error::Error;
use std::process;


fn main() -> Result<(), Box<dyn Error>> {
    let container1 = load_image("original.ppm");
    let container2 = load_image("edited.ppm");

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


    // Define dimensions for the histograms
    //let dimensions = (container1.width, container1.height);

    // // Create histograms for each channel
    // for (i, channel_data) in rgba_channels.into_iter().enumerate() {
    //     let filename = format!("histogram_channel_{}.png", i + 1);
    //     create_histogram(&filename, &channel_data, dimensions)?;
    // }

    // let filename = format!("histogram_channel_1.png");
    // create_histogram(&filename, &rgba_channels.0)?;

    Ok(())

}

// fn create_histogram(filename: &str, data: &Vec<i16>) -> Result<(), Box<dyn std::error::Error>> {
//     let root = BitMapBackend::new(filename, (3840, 2160)).into_drawing_area();
//     root.fill(&WHITE)?;

//     let max_y = *data.iter().max().unwrap() as i32;
//     println!("max_y = {}", max_y);
    
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Line Plot", ("sans-serif", 50))
//         .x_label_area_size(3840)
//         .y_label_area_size(2160)
//         .build_cartesian_2d(0..data.len() as i32, 0..max_y)?;

//     chart.configure_mesh().draw()?;

//     chart.draw_series(LineSeries::new(
//         data.iter().enumerate().map(|(x, &y)| (x as i32, y as i32)), 
//         &RED,
//     ))?;

//     root.present()?;

//     Ok(())
// }

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
            if (x >= 500 && x < 600 && y >= 500 && y < 600) {
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

struct ImageContainer {
    width: u32,
    height: u32,
    data: Vec<[u8; 4]>,
}