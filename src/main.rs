use std::{
    fs::{self, DirEntry},
    io::Result,
    path::PathBuf,
    process::exit,
};

use crate::id3_tags::ID3Version;

mod id3_tags;

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
