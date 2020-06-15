use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;
#[path = "../lib/byte_reader.rs"]
mod byte_read;

pub fn his_to_wav(input_file: std::path::PathBuf, output_dir: std::path::PathBuf, bit_depth: u8) {
    let mut file = std::fs::File::open(&input_file).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    assert_eq!(
        String::from(std::str::from_utf8(&data[0..=3]).unwrap()),
        "HIS\0",
        "file appears to be of incorrect type"
    );
    println!("HIS FILE");

    let mut sound: Vec<u8> = Vec::new();
    if bit_depth == 8 {
        let datasize = byte_read::read_bytes_le(&data, 0x18, 4) as u32;
        let filesize = datasize + 34;
        sound.append(&mut String::from("RIFF").into_bytes());
        sound.append(&mut Vec::from(filesize.to_le_bytes()));
        sound.append(&mut String::from("WAVEfmt \x10\0\0\0").into_bytes());
        sound.append(&mut Vec::from(&data[0x08..0x18]));
        sound.append(&mut String::from("data").into_bytes());
        sound.append(&mut Vec::from(datasize.to_le_bytes()));
        sound.append(&mut Vec::from(&data[0x1C..]));
    } else if bit_depth == 16 {
        let datasize = byte_read::read_bytes_le(&data, 0x18, 4) as u32 * 2;
        let filesize = datasize + 36;
        sound.append(&mut String::from("RIFF").into_bytes());
        sound.append(&mut Vec::from(filesize.to_le_bytes()));
        sound.append(&mut String::from("WAVEfmt \x10\0\0\0").into_bytes());
        sound.append(&mut Vec::from(&data[0x08..0x10]));
        sound.append(&mut Vec::from((byte_read::read_bytes_le(&data, 0x10, 4) as u32 * 2).to_le_bytes()));
        sound.append(&mut Vec::from((byte_read::read_bytes_le(&data, 0x14, 2) as u16 *2).to_le_bytes()));
        sound.append(&mut vec![0x10, 0x00]);
        sound.append(&mut String::from("data").into_bytes());
        sound.append(&mut Vec::from((datasize).to_le_bytes()));
        for n in 0x1C..data.len() {
            let val: i16 = ((data[n] as i16) - 128) << 8;
            sound.append(&mut Vec::from(val.to_le_bytes()));
        }
    } else {
        panic!("Invalid Bit Depth");
    }

    let out = PathBuf::from(&output_dir).join(format!(
        "{}.wav",
        std::path::Path::new(&input_file)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    ));
    let mut out = std::fs::File::create(out).unwrap();

    out.write_all(sound.as_slice()).unwrap();
}
