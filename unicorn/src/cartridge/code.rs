use log::{debug, error, log_enabled, info, Level};

use std::io::BufRead;

use std::fs::File;
use std::io::BufReader;

pub struct CartridgeJavascript {
    pub data: String,
}

impl CartridgeJavascript {
    pub fn new(lines: &[String]) -> CartridgeJavascript {
        let mut data = "".to_string();

        for line in lines {
            data = data + line;
            data.push('\n');
        }

        CartridgeJavascript { data: data }
    }
}


pub struct CartridgePython {
    pub data: String,
}

impl CartridgePython {
    pub fn new(lines: &[String]) -> CartridgePython {
        let mut data = "".to_string();

        for line in lines {
            data = data + line;
            data.push('\n');
        }

        CartridgePython { data: data }
    }
}

pub struct CartridgeLua {
    pub data: String,
}

impl CartridgeLua {
    pub fn new(lines: &[String]) -> CartridgeLua {
        info!("[CARTRIDGE] [CartridgeLua]");

        let mut data = "".to_string();

        for line in lines {
            data = data + line;
            data.push('\n');
        }

        CartridgeLua { data: data }
    }
}

#[derive(Debug)]
pub struct CartridgeCode {
    pub lines: Vec<String>,
    pub data: Vec<u8>,
    pub version: u8,
    pub code_type: String,
    pub filename: String,
}

impl CartridgeCode {
    pub fn empty() -> CartridgeCode {
        info!("[CARTRIDGE] CartridgeCode");

        CartridgeCode {
            lines: Vec::new(),
            data: Vec::new(),
            version: 0,
            code_type: "".to_string(),
            filename: "".to_string(),
        }
    }

    pub fn new(code_type: String, lines: &[String]) -> CartridgeCode {
        info!("[CARTRIDGE] CartridgeCode");

        CartridgeCode {
            lines: lines.to_vec(),
            data: Vec::new(),
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
        if self.code_type == "lua" {
            let cart = CartridgeLua::new(&self.lines);
            return cart.data;
        }

        if self.code_type == "python" {
            let cart = CartridgePython::new(&self.lines);
            return cart.data;
        }

        if self.code_type == "javascript" {
            let cart = CartridgeJavascript::new(&self.lines);
            return cart.data;
        }

        "".to_string()
    }

    pub fn set_data(&mut self, lines: Vec<String>) {
        self.lines.clear();
        for mut line in lines {
            if line.len() > 0 {
                let len = line.len();
                line.truncate(len - 1);
            }
            self.lines.push(line);
        }
    }
}