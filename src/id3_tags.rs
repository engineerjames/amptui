use std::collections::HashMap;

#[allow(unused)]
const MIN_MP3_SIZE_BYTES: usize = 128;

#[allow(unused)]
const ID3V2_HEADER_SIZE_BYTES: usize = 10;

#[allow(unused)]
const ID3V1_OFFSET_FROM_END_BYTES: usize = 128;

trait ID3Parser {
    fn parse(&self, data: &[u8], version: ID3Version) -> HashMap<ID3TagType, ID3TagData>;
}

pub enum ID3TagData {
    UTF8Text(String),
    UTF16Text(Vec<u16>),
    Binary(Vec<u8>),
}

struct ID3V1Parser;
struct ID3V2Parser;

impl ID3Parser for ID3V1Parser {
    fn parse(&self, data: &[u8], version: ID3Version) -> HashMap<ID3TagType, ID3TagData> {
        HashMap::new()
    }
}

impl ID3Parser for ID3V2Parser {
    fn parse(&self, data: &[u8], version: ID3Version) -> HashMap<ID3TagType, ID3TagData> {
        HashMap::new()
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// ID3v2 tag types with their definitions from the ID3v2.3 standard
pub enum ID3TagType {
    /// AENC - This frame indicates if the actual audio stream is encrypted, and by whom.
    AENC,
    /// APIC - This frame contains a picture directly related to the audio file. Image format is the MIME type and subtype for the image.
    APIC,
    /// COMM - This frame is intended for any kind of full text information that does not fit in any other frame. It consists of a frame header followed by encoding, language and content descriptors and is ended with the actual comment as a text string.
    COMM,
    /// COMR - This frame enables several competing offers in the same tag by bundling all needed information.
    COMR,
    /// ENCR - To identify with which method a frame has been encrypted the encryption method must be registered in the tag with this frame.
    ENCR,
    /// GEOB - In this frame any type of file can be encapsulated. After the header, 'Frame size' and 'Encoding' follows 'MIME type' represented as a terminated string encoded with ISO-8859-1.
    GEOB,
    /// GRID - This frame enables grouping of otherwise unrelated frames. This can be used when some frames are to be signed.
    GRID,
    /// MCDI - This frame is intended for music that comes from a CD, so that the CD can be identified in databases such as the CDDB.
    MCDI,
    /// MLLT - To increase performance and accuracy of jumps within a MPEG audio file, frames with time codes in different locations in the file might be useful.
    MLLT,
    /// OWNE - The ownership frame might be used as a reminder of a made transaction or, if signed, as proof.
    OWNE,
    /// PCNT - This is simply a counter of the number of times a file has been played. The value is increased by one every time the file begins to play.
    PCNT,
    /// POPM - The purpose of this frame is to specify how good an audio file is. It contains the email address to the user, one rating byte and a four byte play counter.
    POPM,
    /// POSS - This frame delivers information to the listener of how far into the audio stream he picked up; in effect, it states the time offset of the first frame in the stream.
    POSS,
    /// PRIV - This frame is used to contain information from a software producer that its program uses and does not fit into the other frames.
    PRIV,
    /// RVAD - This allows the user to say how much he wants to increase/decrease the volume on each channel while the file is played.
    RVAD,
    /// RVRB - This allows you to adjust echoes of different kinds. Reverb left/right is the delay between every bounce in ms.
    RVRB,
    /// SYLT - This is a way of incorporating the words, said or sung lyrics, in the audio file as text, in sync with the audio.
    SYLT,
    /// SYTC - For a more accurate description of the tempo of a musical piece this frame might be used.
    SYTC,
    /// TALB - The 'Album/Movie/Show title' frame is intended for the title of the recording(/source of sound) which the audio in the file is taken from.
    TALB,
    /// TBPM - The 'BPM' frame contains the number of beats per minute in the main part of the audio. The BPM is an integer and represented as a numerical string.
    TBPM,
    /// TCOP - The 'Copyright message' frame, which must begin with a year and a space character, is intended for the copyright holder of the original sound, not the audio file itself.
    TCOP,
    /// TCOM - The 'Composer(s)' frame is intended for the name of the composer(s). They are separated with the "/" character.
    TCOM,
    /// TCON - The 'Content type' frame describes the genre. May use numeric references to ID3v1 genres or custom text definitions.
    TCON,
    /// TDAT - The 'Date' frame is a numeric string in the DDMM format containing the date for the recording. This field is always four characters long.
    TDAT,
    /// TDLY - The 'Playlist delay' defines the numbers of milliseconds of silence between every song in a playlist.
    TDLY,
    /// TENC - The 'Encoded by' frame contains the name of the person or organization that encoded the audio file. This field may contain a copyright message, if the audio file also is copyrighted by the encoder.
    TENC,
    /// TEXT - The 'Lyricist(s)/Text writer(s)' frame is intended for the writer(s) of the text or lyrics in the recording. They are separated with the "/" character.
    TEXT,
    /// TFLT - The 'File type' frame indicates which type of audio this tag defines. (MPG, AAC, VQF, PCM, etc.)
    TFLT,
    /// TIME - The 'Time' frame is a numeric string in the HHMM format containing the time for the recording. This field is always four characters long.
    TIME,
    /// TIT1 - The 'Content group description' frame is used if the sound belongs to a larger category of sounds/music. For example, classical music is often sorted in different musical sections.
    TIT1,
    /// TIT2 - The 'Title/Songname/Content description' frame is the actual name of the piece (e.g. "Adagio", "Hurricane Donna").
    TIT2,
    /// TIT3 - The 'Subtitle/Description refinement' frame is used for information directly related to the contents title (e.g. "Op. 16" or "Performed live at Wembley").
    TIT3,
    /// TKEY - The 'Initial key' frame contains the musical key in which the sound starts. It is represented as a string with a maximum length of three characters.
    TKEY,
    /// TLAN - The 'Language(s)' frame should contain the languages of the text or lyrics spoken or sung in the audio. The language is represented with three characters according to ISO-639-2.
    TLAN,
    /// TLEN - The 'Length' frame contains the length of the audio file in milliseconds, represented as a numeric string.
    TLEN,
    /// TMED - The 'Media type' frame describes from which media the sound originated. (DIG, ANA, CD, LD, TT, MD, DAT, DCC, DVD, TV, VID, RAD, TEL, MC)
    TMED,
    /// TOAL - The 'Original album/movie/show title' frame is intended for the title of the original recording if the music in the file should be a cover of a previously released song.
    TOAL,
    /// TOFN - The 'Original filename' frame contains the preferred filename for the file, since some media doesn't allow the desired length of the filename.
    TOFN,
    /// TOLY - The 'Original lyricist(s)/text writer(s)' frame is intended for the text writer(s) of the original recording, if for example the music in the file should be a cover of a previously released song.
    TOLY,
    /// TOPE - The 'Original artist(s)/performer(s)' frame is intended for the performer(s) of the original recording, if for example the music in the file should be a cover of a previously released song.
    TOPE,
    /// TORY - The 'Original release year' frame is intended for the year when the original recording was released, if the music in the file should be a cover of a previously released song.
    TORY,
    /// TOWN - The 'File owner/licensee' frame contains the name of the owner or licensee of the file and its contents.
    TOWN,
    /// TPE1 - The 'Lead artist(s)/Lead performer(s)/Soloist(s)/Performing group' frame is used for the main artist(s). They are separated with the "/" character.
    TPE1,
    /// TPE2 - The 'Band/Orchestra/Accompaniment' frame is used for additional information about the performers in the recording.
    TPE2,
    /// TPE3 - The 'Conductor' frame is used for the name of the conductor.
    TPE3,
    /// TPE4 - The 'Interpreted, remixed, or otherwise modified by' frame contains more information about the people behind a remix and similar interpretations of another existing piece.
    TPE4,
    /// TPOS - The 'Part of a set' frame is a numeric string that describes which part of a set the audio came from.
    TPOS,
    /// TPUB - The 'Publisher' frame simply contains the name of the label or publisher.
    TPUB,
    /// TRCK - The 'Track number/Position in set' frame is a numeric string containing the order number of the audio file on its original recording.
    TRCK,
    /// TRDA - The 'Recording dates' frame is intended to be used as complement to the TYER, TDAT and TIME frames.
    TRDA,
    /// TRSN - The 'Internet radio station name' frame contains the name of the internet radio station from which the audio is streamed.
    TRSN,
    /// TRSO - The 'Internet radio station owner' frame contains the name of the owner of the internet radio station from which the audio is streamed.
    TRSO,
    /// TSIZ - The 'Size' frame contains the size of the audio file in bytes, excluding the ID3v2 tag, represented as a numeric string.
    TSIZ,
    /// TSRC - The 'ISRC' frame should contain the International Standard Recording Code (ISRC) (12 characters).
    TSRC,
    /// TYER - The 'Year' frame is a numeric string with a year of the recording. This frame is always four characters long.
    TYER,
    /// USER - This frame contains a brief description of the terms of use and ownership of the file.
    USER,
    /// WCOM - The 'Commercial information' frame is a URL pointing at a webpage with information such as where the album can be bought.
    WCOM,
    /// WCOP - The 'Copyright/Legal information' frame is a URL pointing at a webpage where the terms of use and ownership of the file is described.
    WCOP,
    /// WOAF - The 'Official audio file webpage' frame is a URL pointing at a file specific webpage.
    WOAF,
    /// WOAR - The 'Official artist/performer webpage' frame is a URL pointing at the artists official webpage.
    WOAR,
    /// WOAS - The 'Official audio source webpage' frame is a URL pointing at the official webpage for the source of the audio file, e.g. a movie.
    WOAS,
    /// WORS - The 'Official internet radio station homepage' contains a URL pointing at the homepage of the internet radio station.
    WORS,
    /// WPAY - The 'Payment' frame is a URL pointing at a webpage that will handle the process of paying for this file.
    WPAY,
    /// WPUB - The 'Publishers official webpage' frame is a URL pointing at the official webpage for the publisher.
    WPUB,
}

#[allow(unused)]
const TAG_DESCRIPTIONS: &[(ID3TagType, &str)] = &[
    (
        ID3TagType::AENC,
        "Indicates if the actual audio stream is encrypted and by whom",
    ),
    (
        ID3TagType::APIC,
        "Contains a picture directly related to the audio file",
    ),
    (
        ID3TagType::COMM,
        "Full text information that does not fit in any other frame",
    ),
    (
        ID3TagType::COMR,
        "Enables several competing offers in the same tag",
    ),
    (
        ID3TagType::ENCR,
        "Identifies the encryption method of a frame",
    ),
    (ID3TagType::GEOB, "Encapsulates any type of file"),
    (ID3TagType::GRID, "Groups otherwise unrelated frames"),
    (
        ID3TagType::MCDI,
        "Identifies music from a CD in databases like CDDB",
    ),
    (
        ID3TagType::MLLT,
        "Delivers lookup table for jumps within a MPEG audio file",
    ),
    (
        ID3TagType::OWNE,
        "Records ownership information and transaction details",
    ),
    (
        ID3TagType::PCNT,
        "Counter of the number of times a file has been played",
    ),
    (
        ID3TagType::POPM,
        "Specifies how good an audio file is with a rating",
    ),
    (
        ID3TagType::POSS,
        "Delivers time offset information within the audio stream",
    ),
    (
        ID3TagType::PRIV,
        "Contains information from a software producer",
    ),
    (
        ID3TagType::RVAD,
        "Specifies volume increase/decrease on each channel",
    ),
    (ID3TagType::RVRB, "Adjusts echoes of different kinds"),
    (
        ID3TagType::SYLT,
        "Synchronized lyrics or description of events in sync with audio",
    ),
    (
        ID3TagType::SYTC,
        "Provides accurate description of the tempo of a musical piece",
    ),
    (ID3TagType::TALB, "Album/Movie/Show title of the recording"),
    (
        ID3TagType::TBPM,
        "Number of beats per minute in the main part of the audio",
    ),
    (
        ID3TagType::TCOP,
        "Copyright message with copyright holder information",
    ),
    (ID3TagType::TCOM, "Name of the composer(s)"),
    (ID3TagType::TCON, "Genre or content type of the audio"),
    (ID3TagType::TDAT, "Date of the recording in DDMM format"),
    (
        ID3TagType::TDLY,
        "Milliseconds of silence between songs in a playlist",
    ),
    (
        ID3TagType::TENC,
        "Name of the person or organization that encoded the audio",
    ),
    (ID3TagType::TEXT, "Writer(s) of the text or lyrics"),
    (
        ID3TagType::TFLT,
        "Type of audio this tag defines (MPG, AAC, VQF, PCM, etc.)",
    ),
    (ID3TagType::TIME, "Time of the recording in HHMM format"),
    (ID3TagType::TIT1, "Content group description"),
    (ID3TagType::TIT2, "Title/Songname/Content description"),
    (ID3TagType::TIT3, "Subtitle/Description refinement"),
    (ID3TagType::TKEY, "Musical key in which the sound starts"),
    (
        ID3TagType::TLAN,
        "Languages of the text or lyrics spoken/sung",
    ),
    (ID3TagType::TLEN, "Length of the audio file in milliseconds"),
    (
        ID3TagType::TMED,
        "Media type from which the sound originated",
    ),
    (ID3TagType::TOAL, "Original album/movie/show title"),
    (ID3TagType::TOFN, "Original filename of the file"),
    (ID3TagType::TOLY, "Original lyricist(s)/text writer(s)"),
    (ID3TagType::TOPE, "Original artist(s)/performer(s)"),
    (ID3TagType::TORY, "Original release year"),
    (ID3TagType::TOWN, "File owner/licensee name"),
    (ID3TagType::TPE1, "Lead artist(s)/Lead performer(s)"),
    (ID3TagType::TPE2, "Band/Orchestra/Accompaniment"),
    (ID3TagType::TPE3, "Conductor name"),
    (
        ID3TagType::TPE4,
        "Interpreted, remixed, or otherwise modified by",
    ),
    (ID3TagType::TPOS, "Part of a set (disc number in a set)"),
    (ID3TagType::TPUB, "Label or publisher name"),
    (ID3TagType::TRCK, "Track number/Position in set"),
    (ID3TagType::TRDA, "Recording dates"),
    (ID3TagType::TRSN, "Internet radio station name"),
    (ID3TagType::TRSO, "Internet radio station owner"),
    (ID3TagType::TSIZ, "Size of the audio file in bytes"),
    (
        ID3TagType::TSRC,
        "International Standard Recording Code (ISRC)",
    ),
    (ID3TagType::TYER, "Year of the recording"),
    (ID3TagType::USER, "Terms of use and ownership of the file"),
    (ID3TagType::WCOM, "Commercial information webpage"),
    (ID3TagType::WCOP, "Copyright/Legal information webpage"),
    (ID3TagType::WOAF, "Official audio file webpage"),
    (ID3TagType::WOAR, "Official artist/performer webpage"),
    (ID3TagType::WOAS, "Official audio source webpage"),
    (ID3TagType::WORS, "Official internet radio station homepage"),
    (ID3TagType::WPAY, "Payment webpage"),
    (ID3TagType::WPUB, "Publishers official webpage"),
];

impl ID3TagType {
    #[allow(unused)]
    pub fn get_description(&self) -> Option<&'static str> {
        let tag_to_return = TAG_DESCRIPTIONS
            .iter()
            .find(|(tag, _)| tag == self)
            .map(|(_, desc)| &**desc);

        tag_to_return
    }
}

#[allow(unused)]
#[derive(PartialEq, Debug)]
/// Version of the ID3 tag standard, which controls how to decode the tag data.
pub enum ID3Version {
    /// The v1 tag allows 30 bytes each for the title, artist, album, and a "comment",
    /// 4 bytes for the year, and 1 byte to identify the genre of the song from a predefined
    /// list of values: https://en.wikipedia.org/wiki/List_of_ID3v1_genres
    V3_1_0,

    /// In 1997, a modification to ID3v1 was proposed in which two bytes formerly allocated
    /// to the comment field were used instead to store a track number so that albums stored
    /// across multiple files could be correctly ordered. The modified format became known as ID3v1.1
    V3_1_1,

    /// BirdCage Software proposed ID3v1.2 in the early 2000s, which enlarged many
    /// of the fields from 30 to 60 bytes and added a sub-genre field while retaining
    /// backward compatibility with v1.1 by placing its new "enhanced" tag in front of a
    /// standard v1.1 tag.
    V3_1_2,

    /// ID3v2 tags are of variable size and are usually placed at the start of the file,
    /// which enables metadata to load immediately, even when the file as a whole is loading
    /// incrementally during streaming. A ID3v2 tag consists of a number of optional frames,
    /// each of which contains a piece of metadata up to 16 MB in size. For example, a TT2 frame
    /// may be included to contain a title. The entire tag may be as large as 256 MB, and strings
    /// may be encoded in Unicode.
    V3_2_0,

    /// The first public variant of v2, ID3v2.2, used three character frame identifiers rather than
    /// four (TT2 for the title instead of TIT2). It is considered obsolete.
    V3_2_2,

    /// ID3v2.3 is the most widely used version of ID3v2 tags and is widely supported by Windows
    /// Explorer and Windows Media Player. Notably it introduced the ability to embed an image
    /// such as an album cover.
    V3_2_3,

    /// ID3v2.4 was published on November 1, 2000. It defines 83 frame types, allows text frames to
    /// contain multiple values separated with a null byte, and permits the tag to be stored at either
    /// the beginning or the end of the file.
    V3_2_4,

    /// If the tag doesn't match anything we expect, we can use this to signal that we are going
    /// to have to do our best given the data we have with no guarantee of success.
    Unknown,
}

// TODO: Look into APE tags as well?

#[allow(unused)]
/// Retrieves the ID3 version from the given (full) MP3 binary data.
pub fn get_id3_version(data: &[u8]) -> ID3Version {
    if data.len() < MIN_MP3_SIZE_BYTES {
        return ID3Version::Unknown;
    }

    // ID3v2 tags are at the very beginning of the file and start with "ID3"
    if data[0..3] == *b"ID3" {
        let minor_version = data[3];

        match minor_version {
            2 => ID3Version::V3_2_2,
            3 => ID3Version::V3_2_3,
            4 => ID3Version::V3_2_4,
            _ => ID3Version::Unknown,
        }
    } else {
        // If the tag is not at the beginning, it must be at the end of the file and is v1.1 or v1.2
        let tag_start = data.len() - ID3V1_OFFSET_FROM_END_BYTES;
        if data[tag_start..tag_start + 3] == *b"TAG" {
            ID3Version::V3_1_0
        } else {
            ID3Version::Unknown
        }
    }
}

pub fn get_id3_tags(data: &[u8]) -> HashMap<ID3TagType, ID3TagData> {
    let version = get_id3_version(data);

    let parser: Box<dyn ID3Parser> = match version {
        ID3Version::V3_1_0 | ID3Version::V3_1_1 | ID3Version::V3_1_2 => Box::new(ID3V1Parser),
        ID3Version::V3_2_0 | ID3Version::V3_2_2 | ID3Version::V3_2_3 | ID3Version::V3_2_4 => {
            Box::new(ID3V2Parser)
        }
        ID3Version::Unknown => {
            println!("Unknown ID3 version, cannot parse tags");
            return HashMap::new();
        }
    };

    parser.parse(data)
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_get_description_correct_value() {
        assert_ne!(ID3TagType::AENC.get_description(), None);
    }

    #[test]
    fn test_get_id3_version_v3_2() {
        let mut data: Vec<u8> = vec![0; MIN_MP3_SIZE_BYTES];
        data[0..10].copy_from_slice(b"ID3\x03\x00\x00\x00\x00\x00\x0F");
        assert_eq!(get_id3_version(&data), ID3Version::V3_2_3);
    }

    #[test]
    fn test_get_id3_version_v3_1() {
        let mut data: Vec<u8> = vec![0; ID3V1_OFFSET_FROM_END_BYTES];
        data[0..3].copy_from_slice(b"TAG");
        assert_eq!(get_id3_version(&data), ID3Version::V3_1_0);
    }

    #[test]
    fn test_get_id3_version_unknown() {
        let mut data: Vec<u8> = vec![0; MIN_MP3_SIZE_BYTES];
        data[0..10].copy_from_slice(b"NOTID3DATA");
        assert_eq!(get_id3_version(&data), ID3Version::Unknown);
    }
}
