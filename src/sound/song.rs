use std::io::Cursor;
use std::io::prelude::*;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use std::fs::File;
use std::io::BufReader;
use std::io;
use std::convert;


#[derive(Debug)]
pub enum Error {
    Err(String),
    IOError(io::Error),
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

pub fn read_string(buf: &mut Cursor<Vec<u8>>, len: usize) -> Result<String, Error> {
    let mut ret = String::new();
    try!(buf.take(len as u64).read_to_string(&mut ret));
    Result::Ok(ret)
}

pub fn read_stringz(buf: &mut Cursor<Vec<u8>>) -> Result<String, Error> {
    let mut ret = String::new();
    let mut buf_ret = Vec::new();
    buf.read_until(0, &mut buf_ret);
    try!(Cursor::new(buf_ret.clone())
             .take(buf_ret.len() as u64)
             .read_to_string(&mut ret));
    Result::Ok(ret)
}

pub fn read_u8(buf: &mut Cursor<Vec<u8>>) -> Result<u8, Error> {
    Result::Ok(try!(buf.read_u8()))
}

pub fn read_u32(buf: &mut Cursor<Vec<u8>>) -> Result<u32, Error> {
    Result::Ok(try!(buf.read_u32::<LittleEndian>()))
}


pub fn skip_bytes(buf: &mut Cursor<Vec<u8>>, len: usize) {
    let position = buf.position();

    buf.set_position(position + 4);
}

#[derive(Clone)]
pub struct SongPattern {}

impl SongPattern {
    pub fn new(buf: &mut Cursor<Vec<u8>>) -> SongPattern {
        SongPattern {}
    }
}

#[derive(Clone)]
pub struct SongSequenceRow {
    pub pattern: Vec<u8>,
}

impl SongSequenceRow {
    pub fn new() -> SongSequenceRow {
        SongSequenceRow { pattern: Vec::new() }
    }
}

#[derive(Clone)]
pub struct SongSequence {
    pub rows: Vec<SongSequenceRow>,
}

impl SongSequence {
    pub fn new(buf: &mut Cursor<Vec<u8>>) -> SongSequence {
        let count = read_u8(buf).unwrap();

        println!("INFO SEQ {:?}", count);

        let mut rows = Vec::new();
        for i in 0..count {
            rows.push(SongSequenceRow::new());
        }


        for track in 0..4 {
            for i in 0..count {
                let track_value = read_u8(buf).unwrap();
                rows[i as usize].pattern.push(track_value);
            }
        }

        SongSequence { rows: rows.clone() }
    }
}
#[derive(Clone)]
pub struct SongSection {
    pub name: String,
    pub size: u32,
    pub version: u8,
    pub section_name: String,
    pub num_pattern_rows: u8,
    pub num_sequence_rows: u8,
    pub sequence: SongSequence,
    pub patterns: Vec<SongPattern>,
}

impl SongSection {
    pub fn new(buf: &mut Cursor<Vec<u8>>) -> SongSection {
        let name = read_string(buf, 4);
        let section_size = read_u32(buf);
        println!("SIZE = {:?}", section_size);


        let version = read_u8(buf);
        println!("VERSION = {:?}", version);

        let section_name = read_stringz(buf);
        println!("SECTION NAME = {:?}", section_name);

        let num_pattern_rows = read_u8(buf);
        let num_sequence_rows = read_u8(buf);


        println!("NUM {:?} {:?}", num_pattern_rows, num_sequence_rows);

        println!("RE {:?}", read_string(buf, 4));
        println!("SIZE {:?}", read_u32(buf));

        let sequence = SongSequence::new(buf);

        println!("RE {:?}", read_string(buf, 4));
        println!("SIZE {:?}", read_u32(buf));

        let mut patterns = Vec::new();
        let count_pattern = read_u8(buf).unwrap();
        println!("COUNT {:?}", count_pattern);

        for _ in 0..count_pattern {
            patterns.push(SongPattern::new(buf));
        }


        SongSection {
            name: name.unwrap().clone(),
            size: section_size.unwrap(),
            version: version.unwrap(),
            section_name: section_name.unwrap().clone(),
            num_pattern_rows: num_pattern_rows.unwrap(),
            num_sequence_rows: num_sequence_rows.unwrap(),
            sequence: sequence,
            patterns: patterns.clone(),
        }
    }
}

#[allow(dead_code)]
pub struct Song {
    pub sections: Vec<SongSection>,
    pub parsing: bool,
}

#[allow(dead_code)]
impl Song {
    pub fn new(buf: &mut io::BufRead) -> Song {
        let mut parsing = false;
        let mut sections = Vec::new();

        let mut buffer = Vec::new();

        buf.read_to_end(&mut buffer).unwrap();

        let mut cur_buf = Cursor::new(buffer);

        sections.push(SongSection::new(&mut cur_buf));

        parsing = true;

        Song {
            sections: sections.clone(),
            parsing: parsing,
        }
    }

    pub fn new_from_file(filename: String) -> Song {
        let f = File::open(filename.clone()).unwrap();
        let mut buf_reader = BufReader::new(f);

        Song::new(&mut buf_reader)
    }

    pub fn valid(&mut self) -> bool {
        self.parsing
    }
}

#[cfg(test)]
mod tests {
    use super::Song;

    #[test]
    fn parse_song_file() {
        let mut s = Song::new_from_file("examples/assets/dub.song".to_string());
        assert_eq!(s.sections.len(), 1);
        assert!(s.valid());
    }
}
