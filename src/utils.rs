use std::fmt;

#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    pub vec: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, rows: usize, cols: usize) -> Self {
        assert!(vec.len() == rows * cols);
        Self { vec, rows, cols }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.cols * row;
        &self.vec[i..(i + self.cols)]
    }

    pub fn index(&self, col: usize, row: usize) -> &T {
        let i = self.cols * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, col: usize, row: usize) -> &mut T {
        let i = self.cols * row;
        &mut self.vec[i + col]
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.rows {
            if i != 0 {
                str.push_str(", ");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
        }
        write!(f, "[{}]", str)
    }
}
