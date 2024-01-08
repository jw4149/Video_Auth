extern crate ffmpeg_next as ffmpeg;

use std::path::PathBuf;
use ffmpeg::util::frame::video::Video;
use video_rs::{self, Decoder, Locator};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    video_rs::init().unwrap();

    let source: Locator = PathBuf::from("samples/sample.mp4").into();
    
    let mut decoder = Decoder::new(&source).expect("failed to create decoder");
    // let (width, height) = decoder.size();
    let frame_rate = decoder.frame_rate();
    println!("frame rate is: {}", frame_rate);

    let mut i = 0;
    for frame in decoder.decode_raw_iter() {
        if let Ok(frame) = frame {
            println!("Processing frame #{}", i);
            save_file(&frame, i).unwrap();
            i += 1;
        } else {
            break;
        }
    }
}

fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
    let mut file = File::create(format!("sample_frames/frame{}.ppm", index))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?;
    Ok(())
}
