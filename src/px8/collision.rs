pub struct Rect {

}

impl Rect {
    pub fn new() -> Rect {
        Rect {

        }
    }
}

pub struct Collision {
    cell_size: u32,
    rects: HashMap<string, Rect>,
}

impl Collision {
    pub fn new(cell_size: u32) -> Collision {
        Collision {
            cell_size: cell_size,
            rects: HashMap::new(),
        }
    }

    pub fn add(&mut self, x: i32 ,y: i32 ,w: i32 ,h: i32) {

    }
}