
pub fn read_bytes_le(data: &[u8], start: usize, length: usize) -> usize {
    let mut r: usize = 0;
    for b in 0..length {
        r <<= 8;
        r |= data[start + length - b - 1] as usize;
    }
    r
}

pub fn read_bytes_be(data: &[u8], start: usize, length: usize) -> usize {
    let mut r: usize = 0;
    for b in 0..length {
        r <<= 8;
        r |= data[start + b] as usize;
    }
    r
}