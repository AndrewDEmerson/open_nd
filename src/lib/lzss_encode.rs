pub fn encode_lzss(data: &mut [u8]) -> Vec<u8> {
    // The min and max sizes of a pattern in order for it to be valid
    const MIN_MATCH_SIZE: usize = 3;
    const MAX_MATCH_SIZE: usize = 18;

    // The dictionary contains the last DICTIONARY_SIZE elements that have been encoded
    // It is a sliding dictionary, so when a new element is added, the oldest is removed
    const DICTIONARY_SIZE: usize = 4096;
    let mut dictionary: [u8; DICTIONARY_SIZE] = [0x0; DICTIONARY_SIZE];
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

    loop {
        // Take a slice of the input file, starting at the index, and of length max size or E.O.F.
        let read_buffer;
        if read_index + MAX_MATCH_SIZE < data.len() {
            read_buffer = &data[read_index..read_index + MAX_MATCH_SIZE];
        } else {
            read_buffer = &data[read_index..];
        }
        if read_buffer.len() == 0 {
            // There is no more unencoded data, end the loop
            break;
        }
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
                // If the match is larger than the minimum, and it is the largest found, save its index and length
                if length >= MIN_MATCH_SIZE && length > match_length {
                    match_start_index = i;
                    match_length = length;
                }
            }
        }

        // Added a flag for the data
        if match_length != 0 {
            // If the match length is not zero, then it is >= the MIN_MATCH_SIZE
            // A matching pattern is represented by a flag of 0
            flags >>= 1;
            let lsb: u8 = match_start_index as u8;
            let msn: u8 = (match_start_index >> 8) as u8;
            let msb: u8 = (msn << 4) | (match_length as u8 - 3);
            write_buffer.push(lsb);
            write_buffer.push(msb);

            // write the symbols written to the dictionary
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

            dictionary[dict_index as usize] = read_buffer[0];
            dict_index += 1;
            dict_index %= DICTIONARY_SIZE as u16;
            read_index += 1;
        }
        flag_write_index += 1;
        if flag_write_index == 8 {
            out_data.push(flags);
            out_data.append(&mut write_buffer);
            flags = 0;
            flag_write_index = 0;
        }
    }

    while flag_write_index < 8 {
        flags >>= 1;
        flags |= 0b10000000;
        flag_write_index += 1;
    }
    out_data.push(flags);
    out_data.append(&mut write_buffer);

    out_data
}
