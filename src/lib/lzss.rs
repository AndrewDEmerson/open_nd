pub fn decode_lzss(data: &mut [u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut buffer: [u8; 4096] = [0x20; 4096];
    let mut flags: u8;
    let mut buf_write_index: u16 = 0xFEE;
    let mut buf_read_index: u16;
    let mut index = 0;

    while index < data.len() {
        flags = data[index];
        index += 1;
        for _ in 0..8 {
            if (flags & 1) != 0 {
                if index >= data.len() {
                    return output;
                }
                output.push(data[index]);
                buffer[buf_write_index as usize] = data[index];
                buf_write_index += 1;
                buf_write_index %= 4096;
                index += 1;
            } else {
                if index + 1 >= data.len() {
                    return output;
                }
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

pub fn encode_lzss(data: &mut [u8]) -> Vec<u8> {
    // The min and max sizes of a pattern in order for it to be valid
    const MIN_MATCH_SIZE: usize = 3;
    const MAX_MATCH_SIZE: usize = 18;

    // The dictionary contains the last DICTIONARY_SIZE elements that have been encoded
    // It is a sliding dictionary, so when a new element is added, the oldest is removed
    const DICTIONARY_SIZE: usize = 4096;
    let mut dictionary: [u8; DICTIONARY_SIZE] = [0x20; DICTIONARY_SIZE];
    let mut dict_index: u16 = 0xFEE;

    // A byte of 8 flags is written before 8 bytes of corosponding data
    // Flags are written to the flags variable, before being written to out_data when it is full.
    // Flag_write_index tracks how many bits of the flag byte has been written to the flags var
    let mut flags: u8 = 0;
    let mut flag_write_index: u8 = 0;

    // The write buffer stores that data that corospondes with the flag currently being written
    let mut write_buffer: Vec<u8> = Vec::with_capacity(16);

    // Out_data is a byte vector that is what will be written to the file when it has been fully encoded
    let mut out_data: Vec<u8> = Vec::new();

    // Read index it the byte that the program in now on to encode from the input file
    let mut read_index = 0;

    // The program will loop until we reach the end of the data, where we break
    loop {
        // Take a slice of the input file, starting at the index, and of length max size or E.O.F.
        let read_buffer;
        if read_index + MAX_MATCH_SIZE < data.len() {
            read_buffer = &data[read_index..read_index + MAX_MATCH_SIZE];
        } else {
            read_buffer = &data[read_index..];
        }
        // There is no more data to read in, so we need to break from the loop
        if read_buffer.len() == 0 {
            break;
        }
        // match_start_index will hold the position in the dictionary with the largest match
        // match_length is the length of the match at that index.
        let mut match_start_index = 0;
        let mut match_length = 0;
        for i in 0..DICTIONARY_SIZE {
            // Read through the dictionary looking for a match
            if dictionary[i % DICTIONARY_SIZE] == read_buffer[0] {
                let mut length = 1;
                // Find the length of the match
                for c in 1..MAX_MATCH_SIZE {
                    if c < read_buffer.len()
                        && dictionary[(i + c) % DICTIONARY_SIZE] == read_buffer[c]
                    {
                        length += 1;
                    } else {
                        break;
                    }
                }
                // If the match is greater than or equal to the minimum length,
                // and if it is the largest match found, then record its index and length
                if length >= MIN_MATCH_SIZE && length > match_length {
                    match_start_index = i;
                    match_length = length;
                }
            }
        }

        if match_length != 0 {
            // If the match_length is not zero, then it is >= the MIN_MATCH_SIZE
            // A matching pattern is represented by a flag of 0
            flags >>= 1;
            // The location of the match in the dictionary is written as two bytes
            let lsb: u8 = match_start_index as u8;
            let msn: u8 = (match_start_index >> 8) as u8;
            let msb: u8 = (msn << 4) | (match_length as u8 - 3);
            write_buffer.push(lsb);
            write_buffer.push(msb);

            // Copy the bytes processed to the dictionary
            for i in 0..match_length {
                dictionary[dict_index as usize] = read_buffer[i];
                dict_index += 1;
                dict_index %= DICTIONARY_SIZE as u16;
            }
            read_index += match_length;
        } else {
            // A match was not found, so a literal byte of data will be written
            // This is represented by a flag of 1
            flags >>= 1;
            flags |= 0b10000000;
            write_buffer.push(read_buffer[0]);

            // As this is a single literal byte, only a single byte is written to the dictionary
            dictionary[dict_index as usize] = read_buffer[0];
            dict_index += 1;
            dict_index %= DICTIONARY_SIZE as u16;
            read_index += 1;
        }
        flag_write_index += 1;
        // When the flag buffer is full, The flag buffer is pushed onto the out_data array,
        // The write buffer is then appended to the out_data array after the flags
        if flag_write_index == 8 {
            out_data.push(flags);
            out_data.append(&mut write_buffer);
            flags = 0;
            flag_write_index = 0;
        }
    }
    // Reached the end of the input data, all that is left is to add the last write_buffer data to the output
    // Add literal byte flag bits, until the flag byte is full
    while flag_write_index < 8 {
        flags >>= 1;
        flags |= 0b10000000;
        flag_write_index += 1;
    }
    // Add flags and write buffer to output data
    out_data.push(flags);
    out_data.append(&mut write_buffer);

    out_data
}
