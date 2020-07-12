use std::io::prelude::*;

pub fn decode_lzss(data: &mut [u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut buffer: [u8; 4096] = [0x0; 4096];
    let mut flags: u8;
    let mut buf_write_index: u16 = 0xFEE;
    let mut buf_read_index: u16;
    let mut index = 0;

    while index < data.len() {
        flags = data[index];
        index += 1;
        for _ in 0..8 {
            if (flags & 1) != 0 {
                output.push(data[index]);
                buffer[buf_write_index as usize] = data[index];
                buf_write_index += 1;
                buf_write_index %= 4096;
                index += 1;
            } else {
                buf_read_index = data[index] as u16;
                index += 1;
                buf_read_index |= ((data[index] & 0xF0) as u16) << 4;
                let mut j = 0;
                while j < (data[index] & 0x0f) + 3 {
                    output.push(buffer[buf_read_index as usize]);
                    buffer[buf_write_index as usize] = buffer[buf_read_index as usize];
                    buf_read_index += 1;
                    buf_read_index %= 4096;
                    buf_write_index += 1;
                    buf_write_index %= 4096;
                    j += 1;
                }
                index += 1;
            }
            flags >>= 1;
            if index >= data.len() {
                break;
            }
        }
    }
    output
}