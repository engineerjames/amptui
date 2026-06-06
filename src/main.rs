use std::{
    fs::{self, DirEntry},
    io::Result,
    path::{Path, PathBuf},
    process::exit,
};

fn parse_metadata(data: &[u8]) {
    println!("Parsing metadata");
    let mut s = 0;
    loop {
        println!("s={}", s);
        let frame_id = &data[s..s + 4];
        println!("Frame id: {}", str::from_utf8(frame_id).unwrap());

        if frame_id == [0, 0, 0, 0] {
            println!("Exiting metadata loop due to empty frame id");
            break;
        }

        let frame_size = u32::from_be_bytes([data[s + 4], data[s + 5], data[s + 6], data[s + 7]]);
        println!("Frame size: {}", frame_size);

        if data[s + 8] != 0 || data[s + 9] != 0 {
            println!("Flags: {} and {}", data[s + 8], data[s + 9]);
        }

        let payload_start = s + 10;
        let payload_end = payload_start + frame_size as usize;
        if payload_end <= data.len() && frame_size > 0 {
            // First byte is text encoding
            let encoding = data[payload_start];

            // For text frames (TIT2 is title), skip encoding byte
            if encoding == 1 || encoding == 2 {
                // UTF-16 encoded
                let text_bytes = &data[payload_start + 1..payload_end];
                let utf16_values: Vec<u16> = text_bytes
                    .chunks_exact(2)
                    .map(|chunk| {
                        let array: [u8; 2] = chunk.try_into().unwrap();
                        // Of course this data is LE not BE.. FFS
                        u16::from_le_bytes(array)
                    })
                    .skip(1)
                    .collect();
                println!("Text: {:?}", String::from_utf16_lossy(&utf16_values));
            }
        }

        s += 10 + frame_size as usize;

        if s >= data.len() {
            break;
        }
    }
}

fn main() -> Result<()> {
    let mp3_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources");

    let mut mp3_file: Option<DirEntry> = None;
    for entry in fs::read_dir(&mp3_path)? {
        let possible_mp3_file = entry.unwrap();

        if possible_mp3_file
            .file_name()
            .into_string()
            .unwrap_or("".to_string())
            .ends_with("mp3")
        {
            mp3_file = Some(possible_mp3_file);
        }
    }

    if mp3_file.is_none() {
        println!("Error finding mp3 file!");
        exit(1);
    }

    println!("MP3 path is: {}", mp3_path.display());

    let mp3_data = fs::read(mp3_path)?;

    println!("Bytes read: {}", mp3_data.iter().len());

    // First three bytes should be ID3?
    // If ID3v1 the tag is at the end of the file, but
    // ID3v2 is at the beginning and is marked by ID3 in the first three bytes
    if mp3_data[0..3] == *b"ID3" {
        println!("ID3v2 tags detected");
    }

    // ID3v1 has 'TAG' 128 bytes from the end of the file
    //

    // For ID3v2, the header is 10 bytes of data that follows the ID3 tag
    // so it will be bytes
    // ID3 tags

    // Bytes 3 - 4 are the version (0 - 1 in header_data)
    // Byte 5 is the flags
    // Bytes 6 - 9 is the 32-bit synch-safe integer, of which bit 7 is always 0
    println!("Version bytes: {} and {}", mp3_data[3], mp3_data[4]);
    println!("Flags: {}", mp3_data[5]);
    println!(
        "Synch-safe integer bytes: {} {} {} {}",
        mp3_data[6], mp3_data[7], mp3_data[8], mp3_data[9]
    );

    let total_size: u32 = (((mp3_data[6] as u32) << 21)
        | ((mp3_data[7] as u32) << 14)
        | ((mp3_data[8] as u32) << 7)
        | mp3_data[9] as u32)
        .into();

    println!("Total size file is {} bytes", total_size);

    let begin: usize = 10;
    let end: usize = 10 + total_size as usize;
    parse_metadata(&mp3_data[begin..end]);

    Ok(())
}
