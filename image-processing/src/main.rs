use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};

fn main() {
    for i in 263..264 {
        // Load the image
        let original_file_path = format!("./resources/output_original_frames/frame{}.png", i);
        let edited_file_path = format!("./resources/output_edited_frames/frame{}.png", i);
        let img = image::open(original_file_path).expect("Failed to open image");

        // Convert the image to an RGBA image (if not already)
        let img_rgba: RgbaImage = img.to_rgba8();

        // Define the coordinates of the black box
        let x_start = 500;
        let x_end = 600;
        let y_start = 500;
        let y_end = 600;

        // Apply a black box at the specified coordinates
        let mut img_with_black_box = img_rgba.clone();
        for x in x_start..x_end {
            for y in y_start..y_end {
                img_with_black_box.put_pixel(x, y, Rgba([0, 0, 0, 255])); // RGBA for black
            }
        }

        // Save the modified image
        img_with_black_box
            .save_with_format(edited_file_path, ImageFormat::Png)
            .expect("Failed to save image with black box");
    }
}