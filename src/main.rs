use std::{
    fs::{self, DirEntry},
    io::Result,
    path::PathBuf,
    process::exit,
};

use crate::id3_tags::ID3Version;

mod id3_tags;

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
            .unwrap_or("".to_owned())
            .starts_with("23")
        // TODO: Remove this
        {
            mp3_file = Some(possible_mp3_file);
            break;
        }
    }

    if mp3_file.is_none() {
        println!("Error finding mp3 file!");
        exit(1);
    }

    let mp3_file = mp3_file.unwrap();

    println!("MP3 path is: {:?}", mp3_file);

    let mp3_data = fs::read(mp3_file.path())?;

    println!("Bytes read: {}", mp3_data.iter().len());

    let version = id3_tags::get_id3_version(&mp3_data);
    println!("Version: {:?}", version);

    if version != ID3Version::Unknown {
        let tags = id3_tags::get_id3_tags(&mp3_data);
        println!("Number of tags: {:?}", tags.len());
    }

    Ok(())
}
