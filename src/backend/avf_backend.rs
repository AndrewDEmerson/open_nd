use std::io::Read;
use std::num::Wrapping;
#[path = "../lib/byte_reader.rs"]
mod byte_read;
#[path = "../lib/lzss.rs"]
mod lzss;
#[path = "../lib/rgb.rs"]
mod rgb;
#[path = "../lib/export_png.rs"]
mod xpng;

pub fn avf_to_png(input_file: std::path::PathBuf, output_dir: std::path::PathBuf){
    let mut file = std::fs::File::open(&input_file).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    /*
     * Read the header of the file. The header appears to be in big-endian format
     * Important info from the header includes
     * File Decleration, string "AVF WayneSikes" shows this file is the correct format
     * number_frames: the number of entries inside of the frame index
     * width of each frame in pixels (This value appears incorrect <Check Endianness?>) should be 315
     * height of each frame in pixels should be 254
     */

    let mut s = String::new();
    for i in 0..0x0F {
        s.push(data[i] as char);
    }
    assert_eq!(s, "AVF WayneSikes\0", "Header incorrect for AVF file");

    let number_frames: u16 = byte_read::read_bytes_le(&data, 0x15, 2) as u16;
    let frame_width: u16 = byte_read::read_bytes_le(&data, 0x17, 2) as u16;
    let frame_height: u16 = byte_read::read_bytes_le(&data, 0x19, 2) as u16;
    println!("The number of entries in the frame index is {}\nThe frame width in pixels is {}\nthe frame height in pixels is {}", number_frames, frame_width,frame_height);

    struct FrameInfo {
        frame_number: u16,
        frame_offset: u32,
        frame_size: u32,
    }

    /*
     * Read each of the frame indexs to get the needed info to read each frame
     * This data is little endian
     * Information is saved in a vector of type FrameInfo
     */
    let mut hr = 0x21; //a refrence to a hex value to read from that point, initially set to the end of the header
    let mut frames_info = Vec::new();
    for _n in 0..number_frames {
        frames_info.push(FrameInfo {
            frame_number: byte_read::read_bytes_le(&data, hr + 0x00, 2) as u16,
            frame_offset: byte_read::read_bytes_le(&data, hr + 0x02, 4) as u32,
            frame_size: byte_read::read_bytes_le(&data, hr + 0x06, 4) as u32,
        });
        hr += 19; //increase our refrence address to the start of the next frame index
    }

    for f in 0..number_frames {
        //decode the data for the frame
        //let f = 10; // the frame to render

        let slice = &mut data[(frames_info[f as usize].frame_offset as usize)
            ..frames_info[f as usize].frame_offset as usize
                + frames_info[f as usize].frame_size as usize];
        for n in 0..slice.len() {
            //'unencrypt' the data
            slice[n] = (Wrapping(slice[n] as u8) - Wrapping((n % 256) as u8)).0;
        }
        let s = &mut lzss::decode_lzss(slice)[..];
        let mut s = rgb::gen_rgb_array(s);
        let path_o = std::path::PathBuf::from(&output_dir).join(format!(
            "{}_{}.png",
            std::path::Path::new(&input_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap(),
            f
        ));
        xpng::encode_png(&mut s, path_o, frame_width, frame_height);
    }
}
