use std::path::PathBuf;

use video_rs::{self, Decoder, Encoder, EncoderSettings, Locator, Time};

fn main() {
    video_rs::init().unwrap();

    let source: Locator = PathBuf::from("resources/sample.mp4").into();
    let destination: Locator = PathBuf::from("resources/output.mp4").into();
    
    let mut decoder = Decoder::new(&source).expect("failed to create decoder");
    let (width, height) = decoder.size();
    let frame_rate = decoder.frame_rate();

    let settings = EncoderSettings::for_h264_yuv420p(width as usize, height as usize, false);
    let mut encoder = Encoder::new(&destination, settings).expect("failed to create encoder");
    
    let duration: Time = Time::from_nth_of_a_second(frame_rate as usize);
    let mut position = Time::zero();
    let mut i = 0;
    for frame in decoder.decode_iter() {
        println!("Processing frame #{}", i);
        if let Ok((_, mut frame)) = frame {
            frame.slice_mut(ndarray::s![500..600, 500..600, ..]).fill(0);
            encoder.encode(&frame, &position).expect("failed to encode frame");
            position = position.aligned_with(&duration).add();
        } else {
            break;
        }
        i += 1;
    }

    encoder.finish().expect("failed to finish encoder");
}
