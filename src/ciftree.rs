use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Read;
use std::num::Wrapping;
use std::path::PathBuf;
use structopt::StructOpt;
#[path = "lib/lzss.rs"]
mod lzss;
#[path = "lib/byte_reader.rs"]
mod byte_read;

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

    //load the file and check that it is a valid file type
    let mut s = String::new();
    for i in 0..0x14 {
        s.push(data[i] as char);
    }
    assert_eq!(
        s, "CIF TREE WayneSikes\0",
        "Header incorrect for CIFTREE file"
    );

    //the header lists the number of entries inside of the file
    let number_entries: u16 = (data[0x1d] as u16) << 8 | (data[0x1c] as u16);

    /*  this stuct holds the data for a file given in their indexs.
     *   this stuct is compatible with the ciftree format seen in games 3&4 (and others?). this should in theory be compatible with all games, but the filename size and etry size in the index are know to be fiffrent
     *   the CIFTree contains diffrent data types (dat, tga, txt, xs1), not all fields will be relevent dependending on the file
     */
    #[derive(Default)]
    struct Entry {
        filename: String, //The file name without the extension. the length of this varies between games.
        entry_num: u16,   //The chronicallogial placment of this file in the tree
        img_origin_x: u16, //for tga files, in earlier games this is not included in the index and is 0; if not a PLAIN file then it is 0
        img_origin_y: u16, //for tga files, in earlier games this is not included in the index and is 0; if not a PLAIN file then it is 0
        img_width: u16,    //for tga files, the width of the resulting image in pixels
        img_height: u16,   //for tga files, the height of the resulting image in pixels
        data_offset: u32, //this represents the offset in the CIFTREE where the files data can be found
        file_size_decoded: u32, //This is the size of the file after it has been decompressed through LZSS
        file_size_encoded: u32, //This is how many bits inside of the CIFTREE the file takes up
        file_type: u8, //if this is 0x02 then this is a PLAIN(TGA) file, if it is 0x03 then it is a DATA file
    }

    //as the file format is known to change between versions, the game number must be known, with the game number the entry size in the index as well as the filename size limit can be set
    //The index is formated differently between versions and therefore must be read diferently depending on the version.
    let entry_size: usize;
    let filename_size: usize;
    //This vector contains Entry structs for each of the files listed on the CIFTREE.
    let mut entries_info: Vec<Entry> = Vec::with_capacity(number_entries as usize);
    //Contains the extensions for each of the files in the CIFTREE, retrivable with the filename
    let mut extensions: HashMap<String, String> = HashMap::with_capacity(number_entries as usize);

    let game_number = 4;

    //Generate index vector for games 3 to 4
    if (3..=4).contains(&game_number) {
        entry_size = 94;
        filename_size = 33;
        for e in 0..number_entries {
            let index = 0x820 + (entry_size * e as usize);
            let entry = &mut data[index..(index + entry_size)];
            entries_info.push(Entry {
                filename: String::from(
                    std::str::from_utf8(&entry[0..filename_size])
                        .unwrap()
                        .trim_matches('\0'),
                ),
                entry_num: byte_read::read_bytes_le(&entry, 0x21, 2) as u16,
                img_origin_x: byte_read::read_bytes_le(&entry, 0x2B, 2) as u16,
                img_origin_y: byte_read::read_bytes_le(&entry, 0x2f, 2) as u16,
                img_width: byte_read::read_bytes_le(&entry, 0x43, 2) as u16,
                img_height: byte_read::read_bytes_le(&entry, 0x47, 2) as u16,
                data_offset: byte_read::read_bytes_le(&entry, 0x4b, 4) as u32,
                file_size_decoded: byte_read::read_bytes_le(&entry, 0x4f, 4) as u32,
                file_size_encoded: byte_read::read_bytes_le(&entry, 0x57, 4) as u32,
                file_type: byte_read::read_bytes_le(&entry, 0x5b, 1) as u8,
                ..Default::default()
            });
        }
    }

    for f in 0..number_entries as usize {
        //decode the data for the entry
        let slice = &mut data[(entries_info[f as usize].data_offset as usize)
            ..entries_info[f as usize].data_offset as usize
                + entries_info[f as usize].file_size_encoded as usize];
        for n in 0..slice.len() {
            //'unencrypt' the data
            slice[n] = (Wrapping(slice[n] as u8) - Wrapping((n % 256) as u8)).0;
        }
        let file = &mut lzss::decode_lzss(slice)[..];
        assert_eq!(
            file.len(),
            entries_info[f].file_size_decoded as usize,
            "decrypted ciftree file size does not match file size stated by its index"
        );
        if f == 0 {
            extensions = gen_exts(file, number_entries as usize);
        }

        let out = PathBuf::from(&args.output).join(format!("{}.{}", entries_info[f].filename, extensions[&entries_info[f].filename]));
        let mut out = std::fs::File::create(out).unwrap();
        if extensions[&entries_info[f].filename] == "TGA" {
            let tga = gen_tga(
                file,
                entries_info[f].img_width,
                entries_info[f].img_height,
                entries_info[f].img_origin_x,
                entries_info[f].img_origin_y,
            );

            out.write_all(tga.as_slice()).unwrap();
            continue;

        }


        
        out.write_all(file).unwrap();
    }
}

fn gen_exts(file: &[u8], size: usize) -> HashMap<String, String> {
    let mut extensions: HashMap<String, String> = HashMap::with_capacity(size);

    let mut lines = std::str::from_utf8(file).unwrap().lines();
    let mut l: &str;

    for _n in 0..size {
        //this should ideally check for the end of the iterator
        //also should account for CIFLIST not being first in the tree
        loop {
            l = lines.next().unwrap();
            if l.contains('#') {
                continue;
            } else {
                break;
            }
        }
        let mut char_iter = l.chars();
        let mut name = String::new();
        let mut extension = String::with_capacity(3);

        loop {
            let t = char_iter.next();
            if t.is_none() {
                name = String::from("CIFLIST");
                extension = String::from("txt");
                break;
            } else {
                let t = t.unwrap();
                if t == '.' {
                    extension.push(char_iter.next().unwrap());
                    extension.push(char_iter.next().unwrap());
                    extension.push(char_iter.next().unwrap());
                    break;
                } else {
                    name.push(t);
                }
            }
        }
        extensions.entry(name).or_insert(extension);
    }
    extensions
}

fn gen_tga(file: &[u8], width: u16, height: u16, ox: u16, oy: u16) -> Vec<u8> {
    let mut a: Vec<u8> = vec![
        0x00,
        0x00,
        0x02,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        (ox & 0x00FF) as u8,
        ((ox & 0xFF00) >> 8) as u8,
        (oy & 0x00FF) as u8,
        ((oy & 0xFF00) >> 8) as u8, //origin
        (width & 0x00FF) as u8,
        ((width & 0xFF00) >> 8) as u8,
        (height & 0x00FF) as u8,
        ((height & 0xFF00) >> 8) as u8, //width & height
        0x0F,
        0x20, //16 bit pixel size; origin is in upper left hand corner
    ];
    a.extend_from_slice(file);
    a
}
