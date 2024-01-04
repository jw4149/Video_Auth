use ffmpeg_next as ffmpeg;

use ffmpeg::format::Pixel;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use video::print_type_of;
use video::file_op::draw_black_box;
use video::file_op::save_file;

fn main() -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();

    // call unwrap can potentially cause a panic if the Result<Input, Error> is an err.
    let mut ictx = ffmpeg::format::input(&"resource/input.mp4").unwrap();
    if let Some(stream) = ictx.streams().best(ffmpeg::media::Type::Video) {
        println!("Best video stream index: {}", stream.index());
    }

    if let Some(stream) = ictx.streams().best(ffmpeg::media::Type::Audio) {
        println!("Best audio stream index: {}", stream.index());
    }

    // for the example mp4, we don't have subtitle stream
    if let Some(stream) = ictx.streams().best(ffmpeg::media::Type::Subtitle) {
        println!("Best subtitle stream index: {}", stream.index());
    }
    
    let video_stream = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = video_stream.index();

    // print out some metadata
    println!("The framerate of video stream {}", video_stream.rate());
    println!("The frames {}", video_stream.frames());
    println!("Metadata {:?}", video_stream.metadata());
    println!("duration {:?}", video_stream.duration() as f64 * f64::from(video_stream.time_base()));

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(video_stream.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut frame_index = 0;

    let mut add_black_box_to_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                draw_black_box(&mut rgb_frame, 500, 500, 100, 100);
                save_file(&rgb_frame, frame_index).unwrap();
                frame_index += 1;
            }
            Ok(())
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            receive_and_process_decoded_frames(&mut decoder)?;
        }
    }
    decoder.send_eof()?;
    add_black_box_to_frames(&mut decoder)?;

    Ok(())
}
