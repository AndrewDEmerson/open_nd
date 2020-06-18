use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;
#[path = "lib/byte_reader.rs"]
mod byte_read;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    input_file: std::path::PathBuf,
    // The path to the write directory
    output_dir: std::path::PathBuf,
    //The bit depth of the resulting wav file
    #[structopt(short, long, default_value = "8")]
    bit_depth: u8,
}

fn main() {
    let args = Cli::from_args();

    let mut file = std::fs::File::open(&args.input_file).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    his_to_wav(args.input_file, args.output_dir, args.bit_depth);
}

pub struct Header {
    chunk_id: String,
    file_size: u32,
    format_id: String,
    subchunk1_id: String,
    subchunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    subchunk2_id: String,
    subchunk2_size: u32,
}

pub fn his_to_wav(input_file: std::path::PathBuf, output_dir: std::path::PathBuf, bit_depth: u8) {
    let mut file = std::fs::File::open(&input_file).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let mut head;
    let dat_start: usize;
    if String::from(std::str::from_utf8(&data[0..=3]).unwrap()) == "HIS\0" {
        if data[0x1E] == 0x4F{ //This checks for the 'O' in the OggS identifier
            //ogg vorbis file used in newer games
            let out = PathBuf::from(&output_dir).join(format!(
                "{}.ogg",
                std::path::Path::new(&input_file)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ));
            let mut out = std::fs::File::create(out).unwrap();
            out.write_all(&data[0x1E..]).unwrap();
            return;
        } else {
            //File type used in games 3 and later
            head = Header {
                chunk_id: String::from("RIFF"),
                file_size: byte_read::read_bytes_le(&data, 0x18, 4) as u32 + 34,
                format_id: String::from("WAVE"),
                subchunk1_id: String::from("fmt "),
                subchunk1_size: 16,
                audio_format: byte_read::read_bytes_le(&data, 0x08, 2) as u16,
                num_channels: byte_read::read_bytes_le(&data, 0x0A, 2) as u16,
                sample_rate: byte_read::read_bytes_le(&data, 0x0C, 4) as u32,
                byte_rate: byte_read::read_bytes_le(&data, 0x10, 4) as u32,
                block_align: byte_read::read_bytes_le(&data, 0x14, 2) as u16,
                bits_per_sample: byte_read::read_bytes_le(&data, 0x16, 2) as u16,
                subchunk2_id: String::from("data"),
                subchunk2_size: byte_read::read_bytes_le(&data, 0x18, 4) as u32,
            };
            dat_start = 0x1C;
        }
    } else if String::from(std::str::from_utf8(&data[0..0x16]).unwrap())
        == "Her Interactive Sound\x1A"
    {
        //File type used in games 1 and 2
        println!("Old file type! {:?}", input_file);
        head = Header {
            chunk_id: String::from("RIFF"),
            file_size: byte_read::read_bytes_le(&data, 0x1C, 4) as u32 + 34,
            format_id: String::from("WAVE"),
            subchunk1_id: String::from("fmt "),
            subchunk1_size: 16,
            audio_format: byte_read::read_bytes_le(&data, 0x16, 2) as u16,
            num_channels: byte_read::read_bytes_le(&data, 0x20, 2) as u16,
            sample_rate: byte_read::read_bytes_le(&data, 0x18, 4) as u32
                / byte_read::read_bytes_le(&data, 0x20, 2) as u32,
            byte_rate: byte_read::read_bytes_le(&data, 0x22, 2) as u32 / 8
                * byte_read::read_bytes_le(&data, 0x20, 2) as u32
                * byte_read::read_bytes_le(&data, 0x18, 4) as u32
                / byte_read::read_bytes_le(&data, 0x20, 2) as u32,
            block_align: byte_read::read_bytes_le(&data, 0x22, 2) as u16 / 8
                * byte_read::read_bytes_le(&data, 0x20, 2) as u16,
            bits_per_sample: byte_read::read_bytes_le(&data, 0x16, 2) as u16,
            subchunk2_id: String::from("data"),
            subchunk2_size: byte_read::read_bytes_le(&data, 0x28, 4) as u32,
        };
        dat_start = 0x2C;
    } else {
        eprintln!(
            "The file {:?} does not match any currently known HIS sound formats",
            input_file
        );
        return;
    }
    if bit_depth == 16 {
        head.file_size = (head.file_size - 34) * 2 + 36;
        head.byte_rate *= 2;
        head.block_align *= 2;
        head.bits_per_sample = 16;
        head.subchunk2_size *= 2;
    }

    let mut wav: Vec<u8> = Vec::new();
    wav.append(&mut head.chunk_id.into_bytes());
    wav.append(&mut Vec::from(head.file_size.to_le_bytes()));
    wav.append(&mut head.format_id.into_bytes());
    wav.append(&mut head.subchunk1_id.into_bytes());
    wav.append(&mut Vec::from(head.subchunk1_size.to_le_bytes()));
    wav.append(&mut Vec::from(head.audio_format.to_le_bytes()));
    wav.append(&mut Vec::from(head.num_channels.to_le_bytes()));
    wav.append(&mut Vec::from(head.sample_rate.to_le_bytes()));
    wav.append(&mut Vec::from(head.byte_rate.to_le_bytes()));
    wav.append(&mut Vec::from(head.block_align.to_le_bytes()));
    wav.append(&mut Vec::from(head.bits_per_sample.to_le_bytes()));
    wav.append(&mut head.subchunk2_id.into_bytes());
    wav.append(&mut Vec::from(head.subchunk2_size.to_le_bytes()));
    if bit_depth == 8 {
        wav.append(&mut Vec::from(&data[dat_start..]));
    } else {
        for n in dat_start..data.len() {
            //Sound Data Converted from u8 to s16 data
            let val: i16 = ((data[n] as i16) - 128) << 8;
            wav.append(&mut Vec::from(val.to_le_bytes()));
        }
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
    out.write_all(wav.as_slice()).unwrap();
}
