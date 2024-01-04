### How to run
 - make sure ffmpeg is installed. `brew install ffmpeg`
 - Run `cargo run`

### Input path
 - <ROOT>/video/resource/input.mp4 (this path is gitignored to make git faster)

### Output path
 - <ROOT>/video/fixtures/blackbox/frame{}.mp4 (fixtures are gitignored to make git faster)


### Useful links
 - https://docs.rs/ffmpeg-next/6.1.0/ffmpeg_next/format/stream/struct.Stream.html#method.frames for rust.doc of stream 

### Dan's question
 - when add overlay, does mp4 file have different video streams or just a (combined) stream? 
    - A combined stream

