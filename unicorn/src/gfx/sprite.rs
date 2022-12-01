use num_traits::pow;

#[derive(Clone)]
pub struct DynamicSprite {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub flags: u8,
}

impl DynamicSprite {
    pub fn new(d: Vec<u8>, width: u32, height: u32) -> DynamicSprite {
        DynamicSprite { data: d, width: width, height: height, flags: 0 }
    }

}

#[derive(Copy, Debug)]
pub struct Sprite {
    pub data: [u8; 64],
    pub flags: u8,
}

impl Clone for Sprite {
    fn clone(&self) -> Sprite {
        *self
    }
}

impl Sprite {
    pub fn new(d: [u8; 64]) -> Sprite {
        Sprite { data: d, flags: 0 }
    }

    pub fn is_flags_set(&self, value: u8) -> bool {
        (self.flags & pow(2, value as usize)) != 0
    }

    pub fn is_bit_flags_set(&self, value: u8) -> bool {
        (self.flags & value) != 0
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= pow(2, flag as usize);
        } else {
            self.flags &= !(1 << flag);
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    pub fn set_data(&mut self, idx: usize, col: u8) {
        self.data[idx] = col;
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for (_, elem) in self.data.iter_mut().enumerate() {
            data.push_str(&format!("{:?}", elem));
        }

        data
    }

    pub fn get_line(&mut self, line: u32) -> String {
        let mut v = Vec::new();
        v.extend(self.data.iter().cloned());

        let mut data = String::new();

        let mut data_clone = v.clone();

        let data_line: Vec<_> = data_clone
            .drain((line * 8) as usize..(line * 8 + 8) as usize)
            .collect();

        for c in data_line.clone() {
            data.push_str(&format!("{:03x}", c));
        }

        data
    }

    pub fn horizontal_reflection(&self) -> [u8; 64] {
        let mut ret: [u8; 64] = self.to_u8_64_array();

        for i in 0..4 {
            for j in 0..8 {
                ret.swap((i + j * 8) as usize, ((8 - (i + 1)) + j * 8) as usize);
            }
        }

        ret
    }

    pub fn vertical_reflection(&self) -> [u8; 64] {
        let mut ret: [u8; 64] = self.to_u8_64_array();

        for i in 0..4 {
            for j in 0..8 {
                ret.swap((j + i * 8) as usize, (j + (8 - (i + 1)) * 8) as usize);
            }
        }

        ret
    }

    pub fn flip_x(&self) -> Sprite {
        Sprite::new(self.horizontal_reflection())
    }

    pub fn flip_y(&self) -> Sprite {
        Sprite::new(self.vertical_reflection())
    }

    pub fn to_u8_64_array(&self) -> [u8; 64] {
        let mut arr = [0u8; 64];
        for (place, element) in arr.iter_mut().zip(self.data.iter()) {
            *place = *element;
        }
        arr
    }
}

/* 
impl fmt::Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data_matrix = String::new();
        data_matrix.push('\n');

        for i in 0..8 {
            data_matrix.push_str(format!("{:?}", &self.data[i * 8..i * 8 + 8]).as_str());
            data_matrix.push('\n');
        }

        write!(f, "{}", data_matrix)
    }
}*/

#[cfg(test)]
mod tests {
    use super::Sprite;

    #[test]
    fn test_sprite_flags() {
        let mut s = Sprite::new([0; 64]);
        s.set_flag(0, true);
        assert_eq!(s.is_flags_set(0), true);

        s.set_flag(7, true);
        assert_eq!(s.is_flags_set(7), true);

        s.set_flag(7, false);
        assert_eq!(s.is_flags_set(7), false);
    }

    #[test]
    fn test_sprite_flags2() {
        let mut s = Sprite::new([0; 64]);
        s.set_flags(131);
        assert_eq!(s.is_flags_set(0), true);
        assert_eq!(s.is_flags_set(1), true);
        assert_eq!(s.is_flags_set(2), false);
        assert_eq!(s.is_flags_set(3), false);
        assert_eq!(s.is_flags_set(4), false);
        assert_eq!(s.is_flags_set(5), false);
        assert_eq!(s.is_flags_set(6), false);
        assert_eq!(s.is_flags_set(7), true);
    }
}
