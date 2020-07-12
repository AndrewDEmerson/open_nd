use std::io::Write;
use std::num::Wrapping;

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

struct Frame {
    width: u16,
    height: u16,
    data: Vec<u8>,
}

pub fn png_to_avf(input_file: std::path::PathBuf, output_dir: std::path::PathBuf) {
    let mut frame_paths: Vec<std::path::PathBuf> = Vec::new();
    if input_file.is_dir() {
        for i in 0..std::fs::read_dir(&input_file).unwrap().count() {
            // Taking a directory as an input means that their are multiple frames to write to a file
            frame_paths.push(input_file.join(format!(
            "{}_{}.png",
            std::path::Path::new(&input_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
            i
            )));
        }
    } else {
        frame_paths.push(input_file.clone());
    }

    let mut frames: Vec<Frame> = Vec::new();
    for i in 0..frame_paths.len() {
        let (frame_width, frame_height, rgb_arr) = xpng::decode_png(frame_paths[i].clone());
        let mut pixels: Vec<u8> = rgb::gen_rgb5_array(&rgb_arr[..]);
        let encoded_frame = &mut lzss::encode_lzss(&mut pixels[..]);
        for n in 0..encoded_frame.len() {
            // Encrypt the data
            encoded_frame[n] = (Wrapping(encoded_frame[n] as u8) + Wrapping((n % 256) as u8)).0;
        }
        frames.push(Frame {
            width: frame_width,
            height: frame_height,
            data: encoded_frame.to_vec(),
        });
    }

    let mut head = "AVF WayneSikes\0".as_bytes().to_vec();
    head.append(&mut vec![0, 0x02, 0, 0, 0, 0]); // Six bytes of unknown data
    head.append(&mut Vec::from((frame_paths.len() as u16).to_le_bytes()));
    head.append(&mut Vec::from((frames[0].width).to_le_bytes()));
    head.append(&mut Vec::from((frames[0].height).to_le_bytes()));
    head.append(&mut vec![0x10, 0x42, 0, 0, 0, 0x02]);

    // Write the frame indexs
    let mut data_dist: u32 = 0;
    for i in 0..frame_paths.len() {
        head.append(&mut Vec::from((i as u16).to_le_bytes()));
        let index_dist:u32 = (19 * (frames.len() as u32 - i as u32 - 1)) + 17;
        head.append(&mut Vec::from(
            (head.len() as u32 + index_dist + data_dist).to_le_bytes(),
        )); // Offset of data into file in bytes
        head.append(&mut Vec::from((frames[i].data.len() as u32).to_le_bytes())); // Length of image data in bytes
        head.append(&mut Vec::from((frames[i].width as u32 * frames[i].height as u32 * 2).to_le_bytes())); // Decomp image size (assuming no compression)
        head.append(&mut vec![0; 5]); // Nine bytes of unknown data
        data_dist += frames[i].data.len() as u32;
    }
    for i in 0..frame_paths.len() {
        head.append(&mut frames[i].data);
    }

    let file_out = std::path::PathBuf::from(&output_dir).join(format!(
        "{}.avf",
        std::path::Path::new(&input_file)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    ));
    let mut out = std::fs::File::create(file_out).unwrap();
    out.write_all(head.as_slice()).unwrap();
}
