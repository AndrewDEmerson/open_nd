pub fn gen_rgb_array(data: &[u8]) -> Vec<u8>{
    //Covert data in form of 0x#RRRRRGG-GGGBBBBB into an vector of form [R1,G1,B1,R2,G2,...]
    let mut pixels = Vec::new();
    let mut p = 0;
    while p < data.len() -1{
        let vals = ((data[p + 1] as u16) << 8) | (data[p] as u16);
        pixels.push((((vals >> 10) & 0x1F) as u8) << 3);
        pixels.push((((vals >> 5) & 0x1F) as u8) << 3);
        pixels.push((((vals >> 0) & 0x1F) as u8) << 3);
        p += 2;
    }
    pixels
}

pub fn gen_rgb5_array(data: &[u8]) -> Vec<u8>{
    //Covert data in vector of form [R1,G1,B1,R2,G2,...] into vector in form of 0x#RRRRRGG-GGGBBBBB
    let mut pixels: Vec<u8> = Vec::with_capacity(data.len()*2/3);
    let mut p = 0;
    while p < data.len(){
        let red:u8 = data[p] >> 3;
        let green:u8 = data[p+1] >> 3;
        let blue:u8 = data[p+2] >> 3;
        let msb: u8 = (red << 2) | (green >> 3);
        let lsb:u8 = (green << 5) | blue;
        pixels.push(lsb);
        pixels.push(msb);
        p += 3;
    }
    pixels
}