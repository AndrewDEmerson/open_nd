use std::io::Read;
use structopt::StructOpt;
#[path = "lib/byte_reader.rs"]
mod byte_read;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    #[structopt(short = "i", long = "input")]
    input: std::path::PathBuf,
    // The path to the write directory
    //#[structopt(short = "o", long = "output")]
    //output: std::path::PathBuf,
}

//NOTE testing done mostly on S1900 in TRT

fn main() {
    let args = Cli::from_args();
    println!("Reading from: {}", args.input.display());

    let mut file = std::fs::File::open(&args.input).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    assert_eq!(
        String::from(
            std::str::from_utf8(&data[0..=4])
                .unwrap()
                .trim_matches('\0'),
        ),
        "DATA",
        "file appears to be of incorrect type"
    );
    let filesize = byte_read::read_bytes_be(&data, 0x05, 4) as u32;
    assert_eq!(
        std::str::from_utf8(&data[0x08..=0x0F])
            .unwrap()
            .trim_matches('\0'),
        "SCENSSUM",
        "This appears to be a currently unknow version"
    );

    let scene_description = std::str::from_utf8(&data[0x14..=0x45])
        .unwrap()
        .trim_matches('\0');

    let image_file = std::str::from_utf8(&data[0x46..=0x66])
        .unwrap()
        .trim_matches('\0');

    //data in fields 67 to 6A is unknown

    let audio_file = std::str::from_utf8(&data[0x6B..=0x8B])
        .unwrap()
        .trim_matches('\0');

    println!(
        "This scene is: {}\n and uses the image {:?}\n while playing the audio {}",
        scene_description, image_file, audio_file
    );

    //data in fields 0x8b to 0xb9 is currently unknown

    struct ActRecord<'a> {
        size: u32,
        description: String,
        record_type: u8,
        record_type_variation: u8,
        rest: &'a [u8],
    }

    let mut acts: Vec<ActRecord> = Vec::new();

    let mut hr = 0xba;

    loop {
        while data[hr] != 0x41 {hr+=1}  //some acts appear to have extra zeros out of their file size, this makes sure we point to 'A' in ACT

        acts.push(ActRecord {
            size: byte_read::read_bytes_be(&data, hr + 0x04, 4) as u32,
            
            description: String::from(
                std::str::from_utf8(&data[hr + 0x08..=hr + 0x37])
                    .unwrap()
                    .trim_matches('\0'),
            ),
            record_type: byte_read::read_bytes_le(&data, hr + 0x38, 1) as u8,
            record_type_variation: byte_read::read_bytes_le(&data, hr + 0x39, 1) as u8,
            rest: &data[hr + 0x3a..hr + 8 + byte_read::read_bytes_be(&data, hr + 0x04, 4) as usize],
        });
        //println!("{:?}", acts.last().unwrap().rest);
        //println!();
        hr += acts.last().unwrap().size as usize + 8;
        if hr >= data.len()-1 {break};
    }

    /*
     * 0x0C01 = automatic scene change
     * 0x4B01 = text for voiceover
     */
    for a in acts{
        if a.record_type == 0x0C{
            //this is a scene change
            println!("The next scene is {}", byte_read::read_bytes_le(a.rest, 0, 2));
        }else if a.record_type ==0x96{
            println!("Calling audio file {}.wav", std::str::from_utf8(&a.rest[0..=32]).unwrap().trim_matches('\0'));
        }
    }
}
