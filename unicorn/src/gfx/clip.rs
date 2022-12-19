use std::cmp;

// ClipRect rectangle is exclusive of right and bottom edges
#[derive(Debug)]
pub struct ClipRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl ClipRect {
    pub fn new() -> ClipRect {
        ClipRect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    pub fn intersect(&mut self, other: &ClipRect) {
        self.left = cmp::max(self.left, other.left);
        self.top = cmp::max(self.top, other.top);
        self.right = cmp::min(self.right, other.right);
        self.bottom = cmp::min(self.bottom, other.bottom);
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        (x >= self.left) && (x < self.right) && (y >= self.top) && (y < self.bottom)
    }
}
