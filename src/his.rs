use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
#[path = "lib/byte_reader.rs"]
mod byte_read;

const SOUND_DEPTH: u8 = 16; //set to 8 for 8bit or 16 for 16bit

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    #[structopt(short = "i", long = "input")]
    input: std::path::PathBuf,
    // The path to the write directory
    #[structopt(short = "o", long = "output")]
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    println!("Reading from: {}", args.input.display());

    let mut file = std::fs::File::open(&args.input).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    assert_eq!(
        String::from(std::str::from_utf8(&data[0..=3]).unwrap()),
        "HIS\0",
        "file appears to be of incorrect type"
    );
    println!("HIS FILE");

    let mut sound: Vec<u8> = Vec::new();
    if SOUND_DEPTH == 8 {
        let datasize = byte_read::read_bytes_le(&data, 0x18, 4) as u32;
        let filesize = datasize + 34;
        sound.append(&mut String::from("RIFF").into_bytes());
        sound.append(&mut Vec::from(filesize.to_le_bytes()));
        sound.append(&mut String::from("WAVEfmt \x10\0\0\0").into_bytes());
        sound.append(&mut Vec::from(&data[0x08..0x18]));
        sound.append(&mut String::from("data").into_bytes());
        sound.append(&mut Vec::from(datasize.to_le_bytes()));
        sound.append(&mut Vec::from(&data[0x1C..]));
    } else if SOUND_DEPTH == 16 {
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

    let out = PathBuf::from(&args.output).join(format!(
        "{}.wav",
        std::path::Path::new(&args.input)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    ));
    let mut out = std::fs::File::create(out).unwrap();

    out.write_all(sound.as_slice()).unwrap();
}
