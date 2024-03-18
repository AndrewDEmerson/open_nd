use std::io::Write;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
#[path = "lib/byte_reader.rs"]
mod byte_read;

#[derive(StructOpt)]
#[structopt(name = "HIS Encoder")]
/// Convert an unsinged 8bit wav into a HIS audio file.
struct Cli {
    /// Path to the input WAV file
    input_file: std::path::PathBuf,
    /// Path to a directory where output is saved
    output_dir: std::path::PathBuf,
    /// What version of the HIS file is to be written
    /// 
    /// (1) Used in first 2 games [NOT SUPPORTED].
    /// (2) Used in games after first two.
    /// (3) Based of OGG files [NOT SUPPORTED].
    #[structopt(short = "v", long = "his-ver", default_value = "2")]
    his_ver: u8,
}

fn main() {
    let args = Cli::from_args();
    wav_to_his(args.input_file, args.output_dir, args.his_ver);
}

pub struct Header {
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    subchunk2_size: u32,
}

pub fn wav_to_his(input_file: std::path::PathBuf, output_dir: std::path::PathBuf, his_ver: u8){
    assert_eq!(his_ver, 2, "This file version is not supported!");
    let mut file = std::fs::File::open(&input_file).unwrap();
    let mut wav = Vec::new();
    file.read_to_end(&mut wav).unwrap();

    let head = Header {
        audio_format: byte_read::read_bytes_le(&wav, 0x14, 2) as u16,
        num_channels: byte_read::read_bytes_le(&wav, 0x16, 2) as u16,
        sample_rate: byte_read::read_bytes_le(&wav, 0x18, 4) as u32,
        byte_rate: byte_read::read_bytes_le(&wav, 0x1C, 4) as u32,
        block_align: byte_read::read_bytes_le(&wav, 0x20, 2) as u16,
        bits_per_sample: byte_read::read_bytes_le(&wav, 0x22, 2) as u16,
        subchunk2_size: (wav.len() - 0x2B) as u32,
    };

    let mut data = "HIS\0".as_bytes().to_vec();
    data.append(&mut vec![0x01, 0, 0, 0]);
    data.append(&mut Vec::from((head.audio_format).to_le_bytes()));
    data.append(&mut Vec::from((head.num_channels).to_le_bytes()));
    data.append(&mut Vec::from((head.sample_rate).to_le_bytes()));
    data.append(&mut Vec::from((head.byte_rate).to_le_bytes()));
    data.append(&mut Vec::from((head.block_align).to_le_bytes()));
    data.append(&mut Vec::from((head.bits_per_sample).to_le_bytes()));
    data.append(&mut Vec::from((head.subchunk2_size).to_le_bytes()));
    data.append(&mut wav[0x2C..].to_vec());

    let out = PathBuf::from(&output_dir).join(format!(
        "{}.his",
        std::path::Path::new(&input_file)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    ));
    let mut out = std::fs::File::create(out).unwrap();
    out.write_all(data.as_slice()).unwrap();

}