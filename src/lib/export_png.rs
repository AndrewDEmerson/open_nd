use std::path::PathBuf;
use png;

pub fn encode_png(pixels: &mut Vec<u8>, write:PathBuf, width: u16, height: u16) {
    std::fs::create_dir_all(write.parent().unwrap()).unwrap();
    while pixels.len() < width as usize *height as usize *3{
        pixels.push(0x0);
    }

    println!("Writing file: {}", write.display());
    let w = std::io::BufWriter::new(std::fs::File::create(std::path::Path::new(&write)).unwrap());
    let mut encoder = png::Encoder::new(w, width as u32,height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
}