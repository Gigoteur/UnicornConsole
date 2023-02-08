use log::{warn, info};

use std::io::BufRead;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use std::fs::read;

#[derive(Debug)]
pub struct CartridgeCode {
    pub lines: Vec<String>,
    pub code: String,
    pub version: u8,
    pub code_type: String,
    pub filename: String,
    pub bytes: Vec<u8>,
    pub remote_filename: String,
    pub remote: bool,
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
            bytes: Vec::new(),
            remote_filename: "".to_string(),
            remote: false,
        }
    }

    pub fn new(code_type: String, lines: &[String]) -> CartridgeCode {
        info!("[CARTRIDGE] CartridgeCode");

        let mut data = "".to_string();

        for line in lines {
            data.push_str(line);
            data.push('\n');
        }

        CartridgeCode {
            lines: lines.to_vec(),
            code: data.clone(),
            version: 0,
            code_type: code_type,
            filename: "".to_string(),
            bytes: Vec::new(),
            remote_filename: "".to_string(),
            remote: false,
        }
    }

    pub fn remote(lines: &[String]) -> CartridgeCode {
        let mut code_type = "";
        let mut filename = "";
        let mut code = "".to_string();
        let mut bytes = Vec::new();

        if lines.len() > 0 {           
            filename = lines.get(0).unwrap();
            if filename.contains(".rhai") {
                code_type = "rhai";

                let f1 = File::open(filename).unwrap();
                let buf_reader = BufReader::new(f1);

                for line in buf_reader.lines() {
                    let l = line.unwrap();
                    code.push_str(&l);
                    code.push('\n');
                }
            }
            if filename.contains(".wasm") {
                code_type = "wasm";               
                let path = PathBuf::from_str(filename).unwrap();
                let result = std::fs::read(path).map_err(|e| e.to_string());
                match result {
                    Ok(value) => {
                        bytes.append(&mut value.clone());
                    },
                    Err(_) => {}
                }
            }
        }

        CartridgeCode {
            lines: lines.to_vec(),
            code: code,
            version: 0,
            code_type: code_type.to_string(),
            filename: filename.to_string(),
            bytes: bytes,
            remote_filename: filename.to_string(),
            remote: true,
        }
    }

    pub fn get_code_section(&mut self) -> String {
        if self.remote {
            return "code".to_string();
        }
        return self.code_type.clone();
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

    pub fn get_bytes_data(&mut self) -> Vec<u8> {
        self.bytes.clone()
    }

    pub fn get_data(&mut self) -> String {
        if self.remote {
            let mut s = self.remote_filename.clone();
            s.push('\n');
            return s.clone();
        }
        self.code.clone()
    }

    pub fn set_data(&mut self, data: String) {
        self.code = data;
    }
}