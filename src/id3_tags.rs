/// ID3v2 tag types with their definitions from the ID3v2.3 standard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ID3TagType {
    /// TALB - The 'Album/Movie/Show title' frame is intended for the title of the recording(/source of sound) which the audio in the file is taken from.
    TALB,
    /// TPE1 - The 'Lead artist(s)/Lead performer(s)/Soloist(s)/Performing group' frame is used for the main artist(s). They are separated with the "/" character.
    TPE1,
    /// TPE2 - The 'Band/Orchestra/Accompaniment' frame is used for additional information about the performers in the recording.
    TPE2,
    /// TPE3 - The 'Conductor' frame is used for the name of the conductor.
    TPE3,
    /// TPE4 - The 'Interpreted, remixed, or otherwise modified by' frame contains more information about the people behind a remix and similar interpretations of another existing piece.
    TPE4,
    /// TCOM - The 'Composer(s)' frame is intended for the name of the composer(s). They are separated with the "/" character.
    TCOM,
    /// TEXT - The 'Lyricist(s)/Text writer(s)' frame is intended for the writer(s) of the text or lyrics in the recording. They are separated with the "/" character.
    TEXT,
    /// TOPE - The 'Original artist(s)/performer(s)' frame is intended for the performer(s) of the original recording, if for example the music in the file should be a cover of a previously released song.
    TOPE,
    /// TOLY - The 'Original lyricist(s)/text writer(s)' frame is intended for the text writer(s) of the original recording, if for example the music in the file should be a cover of a previously released song.
    TOLY,
    /// TPUB - The 'Publisher' frame simply contains the name of the label or publisher.
    TPUB,
    /// TIT1 - The 'Content group description' frame is used if the sound belongs to a larger category of sounds/music. For example, classical music is often sorted in different musical sections.
    TIT1,
    /// TIT2 - The 'Title/Songname/Content description' frame is the actual name of the piece (e.g. "Adagio", "Hurricane Donna").
    TIT2,
    /// TIT3 - The 'Subtitle/Description refinement' frame is used for information directly related to the contents title (e.g. "Op. 16" or "Performed live at Wembley").
    TIT3,
    /// TBPM - The 'BPM' frame contains the number of beats per minute in the main part of the audio. The BPM is an integer and represented as a numerical string.
    TBPM,
    /// TKEY - The 'Initial key' frame contains the musical key in which the sound starts. It is represented as a string with a maximum length of three characters.
    TKEY,
    /// TLEN - The 'Length' frame contains the length of the audio file in milliseconds, represented as a numeric string.
    TLEN,
    /// TDAT - The 'Date' frame is a numeric string in the DDMM format containing the date for the recording. This field is always four characters long.
    TDAT,
    /// TIME - The 'Time' frame is a numeric string in the HHMM format containing the time for the recording. This field is always four characters long.
    TIME,
    /// TYER - The 'Year' frame is a numeric string with a year of the recording. This frame is always four characters long.
    TYER,
    /// TRCK - The 'Track number/Position in set' frame is a numeric string containing the order number of the audio file on its original recording.
    TRCK,
    /// TPOS - The 'Part of a set' frame is a numeric string that describes which part of a set the audio came from.
    TPOS,
    /// TFLT - The 'File type' frame indicates which type of audio this tag defines. (MPG, AAC, VQF, PCM, etc.)
    TFLT,
    /// TMED - The 'Media type' frame describes from which media the sound originated. (DIG, ANA, CD, LD, TT, MD, DAT, DCC, DVD, TV, VID, RAD, TEL, MC)
    TMED,
    /// TCON - The 'Content type' frame describes the genre. May use numeric references to ID3v1 genres or custom text definitions.
    TCON,
    /// TCOP - The 'Copyright message' frame, which must begin with a year and a space character, is intended for the copyright holder of the original sound, not the audio file itself.
    TCOP,
    /// TPUB - See TPUB above.
    TPUB_ALT,
    /// TSIZ - The 'Size' frame contains the size of the audio file in bytes, excluding the ID3v2 tag, represented as a numeric string.
    TSIZ,
    /// TSRC - The 'ISRC' frame should contain the International Standard Recording Code (ISRC) (12 characters).
    TSRC,
    /// TENC - The 'Encoded by' frame contains the name of the person or organization that encoded the audio file. This field may contain a copyright message, if the audio file also is copyrighted by the encoder.
    TENC,
    /// TOFN - The 'Original filename' frame contains the preferred filename for the file, since some media doesn't allow the desired length of the filename.
    TOFN,
    /// TOAL - The 'Original album/movie/show title' frame is intended for the title of the original recording if the music in the file should be a cover of a previously released song.
    TOAL,
    /// TORY - The 'Original release year' frame is intended for the year when the original recording was released, if the music in the file should be a cover of a previously released song.
    TORY,
    /// TRDA - The 'Recording dates' frame is intended to be used as complement to the TYER, TDAT and TIME frames.
    TRDA,
    /// TRSN - The 'Internet radio station name' frame contains the name of the internet radio station from which the audio is streamed.
    TRSN,
    /// TRSO - The 'Internet radio station owner' frame contains the name of the owner of the internet radio station from which the audio is streamed.
    TRSO,
    /// TLAN - The 'Language(s)' frame should contain the languages of the text or lyrics spoken or sung in the audio. The language is represented with three characters according to ISO-639-2.
    TLAN,
    /// MCDI - This frame is intended for music that comes from a CD, so that the CD can be identified in databases such as the CDDB.
    MCDI,
    /// COMM - This frame is intended for any kind of full text information that does not fit in any other frame. It consists of a frame header followed by encoding, language and content descriptors and is ended with the actual comment as a text string.
    COMM,
    /// COMR - This frame enables several competing offers in the same tag by bundling all needed information.
    COMR,
    /// ENCR - To identify with which method a frame has been encrypted the encryption method must be registered in the tag with this frame.
    ENCR,
    /// AENC - This frame indicates if the actual audio stream is encrypted, and by whom.
    AENC,
    /// GEOB - In this frame any type of file can be encapsulated. After the header, 'Frame size' and 'Encoding' follows 'MIME type' represented as a terminated string encoded with ISO-8859-1.
    GEOB,
    /// GRID - This frame enables grouping of otherwise unrelated frames. This can be used when some frames are to be signed.
    GRID,
    /// PRIV - This frame is used to contain information from a software producer that its program uses and does not fit into the other frames.
    PRIV,
    /// OWNE - The ownership frame might be used as a reminder of a made transaction or, if signed, as proof.
    OWNE,
    /// APIC - This frame contains a picture directly related to the audio file. Image format is the MIME type and subtype for the image.
    APIC,
    /// PCNT - This is simply a counter of the number of times a file has been played. The value is increased by one every time the file begins to play.
    PCNT,
    /// POPM - The purpose of this frame is to specify how good an audio file is. It contains the email address to the user, one rating byte and a four byte play counter.
    POPM,
    /// POSS - This frame delivers information to the listener of how far into the audio stream he picked up; in effect, it states the time offset of the first frame in the stream.
    POSS,
    /// RVAD - This allows the user to say how much he wants to increase/decrease the volume on each channel while the file is played.
    RVAD,
    /// RVRB - This allows you to adjust echoes of different kinds. Reverb left/right is the delay between every bounce in ms.
    RVRB,
    /// SYLT - This is a way of incorporating the words, said or sung lyrics, in the audio file as text, in sync with the audio.
    SYLT,
    /// SYTC - For a more accurate description of the tempo of a musical piece this frame might be used.
    SYTC,
    /// MLLT - To increase performance and accuracy of jumps within a MPEG audio file, frames with time codes in different locations in the file might be useful.
    MLLT,
    /// TDLY - The 'Playlist delay' defines the numbers of milliseconds of silence between every song in a playlist.
    TDLY,
    /// WOAR - The 'Official artist/performer webpage' frame is a URL pointing at the artists official webpage.
    WOAR,
    /// WOAF - The 'Official audio file webpage' frame is a URL pointing at a file specific webpage.
    WOAF,
    /// WOAS - The 'Official audio source webpage' frame is a URL pointing at the official webpage for the source of the audio file, e.g. a movie.
    WOAS,
    /// WCOM - The 'Commercial information' frame is a URL pointing at a webpage with information such as where the album can be bought.
    WCOM,
    /// WCOP - The 'Copyright/Legal information' frame is a URL pointing at a webpage where the terms of use and ownership of the file is described.
    WCOP,
    /// WPAY - The 'Payment' frame is a URL pointing at a webpage that will handle the process of paying for this file.
    WPAY,
    /// WPUB - The 'Publishers official webpage' frame is a URL pointing at the official webpage for the publisher.
    WPUB,
    /// WORS - The 'Official internet radio station homepage' contains a URL pointing at the homepage of the internet radio station.
    WORS,
    /// TOWN - The 'File owner/licensee' frame contains the name of the owner or licensee of the file and its contents.
    TOWN,
    /// USER - This frame contains a brief description of the terms of use and ownership of the file.
    USER,
}
