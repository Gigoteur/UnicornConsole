use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

pub struct Editor {
    pub data: String,
}

pub fn load_editor(filename: String) -> String {
    let mut data = "".to_string();

    let f = File::open(filename.clone()).unwrap();
    let buf_reader = BufReader::new(f);

    for line in buf_reader.lines() {
        let l = line.unwrap();

        data = data + "\n" + &l;
    }

    data
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            data: load_editor("./sys/editor/editor.py".to_string()).clone(),
        }
    }
}