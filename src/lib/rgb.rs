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