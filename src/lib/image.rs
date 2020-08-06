use png;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn encode_png(pixels: &mut Vec<u8>, write: PathBuf, width: u16, height: u16) {
    std::fs::create_dir_all(write.parent().unwrap()).unwrap();
    while pixels.len() < width as usize * height as usize * 3 {
        pixels.push(0x0);
    }

    //println!("Writing file: {}", write.display());
    let w = std::io::BufWriter::new(std::fs::File::create(std::path::Path::new(&write)).unwrap());
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap_or_else(|error| {
        println!("{} on file {:?}", error, write);
    });
}

#[allow(dead_code)]
pub fn decode_png(input_file: std::path::PathBuf) -> (u16, u16, Vec<u8>) {
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

#[allow(dead_code)]
pub fn gen_rgb_array(data: &[u8]) -> Vec<u8>{
    //Covert data in form of 0x#RRRRRGG-GGGBBBBB into an vector of form [R1,G1,B1,R2,G2,...]
    let mut pixels = Vec::new();
    let mut p = 0;
    while p < data.len() -1{
        let vals = ((data[p + 1] as u16) << 8) | (data[p] as u16);
        pixels.push((((vals >> 10) & 0x1F) as u8) << 3);
        pixels.push((((vals >> 5) & 0x1F) as u8) << 3);
        pixels.push((((vals >> 0) & 0x1F) as u8) << 3);
        p += 2;
    }
    pixels
}

#[allow(dead_code)]
pub fn gen_rgb5_array(data: &[u8]) -> Vec<u8>{
    //Covert data in vector of form [R1,G1,B1,R2,G2,...] into vector in form of 0x#RRRRRGG-GGGBBBBB
    let mut pixels: Vec<u8> = Vec::with_capacity(data.len()*2/3);
    let mut p = 0;
    while p < data.len(){
        let red:u8 = data[p] >> 3;
        let green:u8 = data[p+1] >> 3;
        let blue:u8 = data[p+2] >> 3;
        let msb: u8 = (red << 2) | (green >> 3);
        let lsb:u8 = (green << 5) | blue;
        pixels.push(lsb);
        pixels.push(msb);
        p += 3;
    }
    pixels
}
