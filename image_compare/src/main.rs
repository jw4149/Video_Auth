use image::{open, GenericImageView};
use plotters::prelude::*;
use std::error::Error;
use std::process;


fn main() -> Result<(), Box<dyn Error>> {
    let container1 = load_image("test.png");
    let container2 = load_image("test_compare.png");

    // Print the vector
    println!("width, height of the first image {:?}, {:?}", container1.width, container1.height);
    println!("width, height of the second image {:?}, {:?}", container2.width, container2.height);

    if container1.width != container2.width || container1.height != container2.height {
        println!("The dimension of two images are not the same, cannot build historgram");
        process::exit(1);
    }

    // Split the data into four channels
    /**
    let mut channels = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for [c1, c2, c3, c4] in data {
        channels.0.push(c1);
        channels.1.push(c2);
        channels.2.push(c3);
        channels.3.push(c4);
    }

    // Define dimensions for the histograms
    let dimensions = (container1.width, container1.height);

    // Create histograms for each channel
    for (i, channel_data) in channels.into_iter().enumerate() {
        let filename = format!("histogram_channel_{}.png", i + 1);
        create_histogram(&filename, &channel_data, dimensions)?;
    }
    */

    Ok(())

}

fn create_histogram(filename: &str, data: &[i16], dimensions: (u32, u32)) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(filename, dimensions).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram", ("sans-serif", 40))
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(-10..10, 0..10)?; // Adjust range according to your data

    chart.configure_mesh().draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.filled())
            .data(data.iter().map(|&x| (x as i32, 1))),
    )?;

    root.present()?;

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
            let pixel = img.get_pixel(x, y).0; // Get the pixel value as RGBA
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