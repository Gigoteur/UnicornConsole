use log::{warn, info};

use std::io::BufRead;

use std::fs::File;
use std::io::BufReader;


#[derive(Debug)]
pub struct CartridgeCode {
    pub lines: Vec<String>,
    pub code: String,
    pub version: u8,
    pub code_type: String,
    pub filename: String,
}

impl CartridgeCode {
    pub fn empty() -> CartridgeCode {
        info!("[CARTRIDGE] CartridgeCode");

        CartridgeCode {
            lines: Vec::new(),
            code: "".to_string(),
            version: 0,
            code_type: "".to_string(),
            filename: "".to_string(),
        }
    }

    pub fn new(code_type: String, lines: &[String]) -> CartridgeCode {
        info!("[CARTRIDGE] CartridgeCode");

        let mut data = "".to_string();

        for line in lines {
            data = data + line;
            data.push('\n');
        }

        CartridgeCode {
            lines: lines.to_vec(),
            code: data.clone(),
            version: 0,
            code_type: code_type,
            filename: "".to_string(),
        }
    }

    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    pub fn get_name(&mut self) -> String {
        self.code_type.clone()
    }

    pub fn reload(&mut self) {
        let f1 = File::open(self.filename.as_str());
        match f1 {
            Ok(f1) => {
                let buf_reader = BufReader::new(f1);

                let mut code_section = Vec::new();

                for line in buf_reader.lines() {
                    let l = line.unwrap();
                    code_section.push(l);
                }

                self.lines = code_section;
            }
            Err(e) => {
                warn!("[CARTRIDGE] Error to reload the file {:?} -> {:?}", self.filename, e);
            }
        }

    }

    pub fn get_data(&mut self) -> String {
        self.code.clone()
    }

    pub fn set_data(&mut self, data: String) {
        self.code = data;
    }
}