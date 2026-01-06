use anyhow::Result;
use ffmpeg_next as ffmpeg;
fn main() -> Result<()>{
    ffmpeg::init();

    let path = std::env::args()
    .nth(1)
    .expect("ERROR: correct usage: cargo run -- <file-path>");

    // #[derive(Debug)]
    let mut ictx = ffmpeg::format::input(&path)?;
    let input=  ictx
    .streams()
    .best(ffmpeg::media::Type::Video)
    .ok_or_else(|| anyhow::anyhow!("Unable to find video stream"))?;

    let video_stream_index = input.index();
    
    println!(
        "Video stream index: {}, codec: {:?}",
        video_stream_index,
        input.parameters().id()
    );

    let context = ffmpeg::codec::context::Context::from_parameters(
        input.parameters(),
    )?;

    let mut decoder = context.decoder().video()?;
    let mut frame_count = 0;

    for (stream, packet) in ictx.packets(){
        if stream.index() == video_stream_index{
            decoder.send_packet(&packet);
            let mut frame = ffmpeg::frame::Video::empty();
            while decoder.receive_frame(&mut frame).is_ok(){
                frame_count += 1;
                println!(
                    "Frame {:5} | PTS: {:?} | {}x{} | format: {:?} | stride:{} | data len: {}",
                    frame_count,
                    frame.pts(),
                    frame.width(),
                    frame.height(),
                    frame.format(),
                    frame.stride(1),
                    frame.data(0).len()
                );
            }
        }
    }

    //flush the decoder
    decoder.send_eof()?;
    let mut frame = ffmpeg::frame::Video::empty();
    while decoder.receive_frame(&mut frame).is_ok() {
        frame_count += 1;
        println!(
            "Frame {:5} | PTS: {:?} | {}x{} | format: {:?}",
            frame_count,
            frame.pts(),
            frame.width(),
            frame.height(),
            frame.format()
        );
    }

    println!("Decoded {} frames total", frame_count);
    Ok(())
}
