use png;
use std::path::PathBuf;

pub fn encode_png(pixels: &mut Vec<u8>, write: PathBuf, width: u16, height: u16) {
    std::fs::create_dir_all(write.parent().unwrap()).unwrap();
    while pixels.len() < width as usize * height as usize * 3 {
        pixels.push(0x0);
    }

    println!("Writing file: {}", write.display());
    let w = std::io::BufWriter::new(std::fs::File::create(std::path::Path::new(&write)).unwrap());
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
}

pub fn decode_png(input_file: std::path::PathBuf) -> (u16, u16, Vec<u8>){
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.
    let decoder = png::Decoder::new(std::fs::File::open(input_file).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();
    (info.width as u16, info.height as u16, buf)
}
