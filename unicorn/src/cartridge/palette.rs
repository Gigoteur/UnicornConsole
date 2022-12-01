use gfx::palette::RGB;

use std::collections::HashMap;

#[derive(Debug)]
pub struct CartridgePalette {
    pub colors: HashMap<u32, RGB>
}

impl CartridgePalette {
    pub fn empty() -> CartridgePalette {
        CartridgePalette { colors: HashMap::new() }
    }

    pub fn new(lines: &[String]) -> CartridgePalette {
        let mut colors = HashMap::new();

        for line in lines {
            let split_line = line.split(" ");
            let vec: Vec<&str> = split_line.collect();

            if vec.len() == 4 {
                let color = vec[0].parse::<u32>().unwrap();
                let r = vec[1].parse::<u8>().unwrap();
                let g = vec[2].parse::<u8>().unwrap();
                let b = vec[3].parse::<u8>().unwrap();

                colors.insert(color, RGB::new(r, g, b));
            }
        }

        CartridgePalette { colors: colors }
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for (color, rgb) in &self.colors {
            data.push_str(&format!("{:?} {:?} {:?} {:?}\n", color, rgb.r, rgb.g, rgb.b));
        }

        data
    }

    pub fn set_colors(&mut self, colors: HashMap<u32, RGB>) {
        self.colors.clear();
        self.colors.extend(colors);
    }

}