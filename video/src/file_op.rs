use ffmpeg_next as ffmpeg;

use ffmpeg::format::Pixel;
use ffmpeg::util::frame::video::Video;
use std::fs::File;
use std::io::Write;
use std::any::type_name;

pub fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
    let mut file = File::create(format!("./fixtures/overlay/frame{}.ppm", index))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?; 
    //println!("{}", frame.data(0).len());
    Ok(())
}

pub fn draw_black_box(frame: &mut Video, x: usize, y: usize, width: usize, height: usize) {
    let line_stride = frame.stride(0);
    let pixel_format = frame.format();

    if pixel_format != Pixel::RGB24 {
        panic!("Unsupported pixel format. Expected RGB24.");
    }

    for j in y..(y + height) {
        for i in x..(x + width) {
            let pixel_index = j * line_stride + i * 3;
            frame.data_mut(0)[pixel_index] = 0;     // R
            frame.data_mut(0)[pixel_index + 1] = 0; // G
            frame.data_mut(0)[pixel_index + 2] = 0; // B
        }
    }
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}