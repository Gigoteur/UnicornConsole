use std::fs::File;
use std::io::BufReader;
use std::io;

pub struct Song {
    pub parsing: bool
}

impl Song {
    pub fn new(buf: &mut io::BufRead) -> Song {
        let mut buffer = Vec::new();

        buf.read_to_end(&mut buffer).unwrap();

        Song {
            parsing: false,
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
        let mut s = Song::new_from_file("demos/assets/dub.song".to_string());
        assert!(s.valid())
    }
}