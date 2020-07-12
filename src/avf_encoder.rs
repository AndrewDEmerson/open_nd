use std::io::Read;
use std::num::Wrapping;
use std::io::prelude::*;

#[path = "lib/byte_reader.rs"]
mod byte_read;
#[path = "lib/lzss_encode.rs"]
mod lzss;
#[path = "lib/rgb.rs"]
mod rgb;
#[path = "lib/export_png.rs"]
mod xpng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    input_file: std::path::PathBuf,
    // The path to the write directory
    output_dir: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    png_to_avf(args.input_file, args.output_dir);
}

pub fn png_to_avf(input_file: std::path::PathBuf, output_dir: std::path::PathBuf) {
    let number_frames: u16 = 1;
    let frame_width: u16;
    let frame_height: u16;

    let frame_number: u16 = 0;
    let frame_offset: u32;
    let frame_size: u32;

    let rgb_arr: Vec<u8> = Vec::new();
    let (frame_width, frame_height, rgb_arr) = xpng::decode_png(input_file);
    println!("width: {}, height: {}, len: {}", frame_width,frame_height, rgb_arr.len());
    let mut pixels: Vec<u8> = rgb::gen_rgb5_array(& rgb_arr[..]);
    let mut encoded_frame = &mut lzss::encode_lzss(&mut pixels[..]);
    for n in 0..encoded_frame.len() {
        //'encrypt' the data
        encoded_frame[n] = (Wrapping(encoded_frame[n] as u8) + Wrapping((n % 256) as u8)).0;
    }

    let mut head = "AVF WayneSikes\0".as_bytes().to_vec();
    head.append(&mut vec![0; 6]); // Six bytes of unknown data
    head.append(&mut Vec::from((number_frames).to_le_bytes()));
    head.append(&mut Vec::from((frame_width).to_le_bytes()));
    head.append(&mut Vec::from((frame_height).to_le_bytes()));
    head.append(&mut vec![0; 6]); // Six bytes of unknown data

    //Frame index
    head.append(&mut Vec::from((frame_number).to_le_bytes()));
    head.append(&mut Vec::from((head.len() as u32 + 17).to_le_bytes()));    // Offset into file
    head.append(&mut Vec::from((encoded_frame.len() as u32).to_le_bytes()));    // length of image data
    head.append(&mut vec![0; 9]); // Nine bytes of unknown data
    head.append(&mut encoded_frame);

    //let output_file = std::path::PathBuf::from(&String::from("avf.avf"));
    let mut out = std::fs::File::create(output_dir).unwrap();
    out.write_all(head.as_slice()).unwrap();


}
