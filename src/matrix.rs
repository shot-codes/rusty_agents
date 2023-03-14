use std::fmt::Display;

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Copy + Display> Matrix<T> {
    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        Matrix {
            data: vec![default; rows * cols],
            rows,
            cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        assert!(row < self.rows);
        assert!(col < self.cols);
        self.data[row * self.cols + col] = value;
    }

    #[allow(dead_code)]
    pub fn print_grid(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                eprint!("{} ", self.get(row, col));
            }
            eprintln!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolen() {
        let mut m = Matrix::new(6, 3, false);
        m.set(0, 0, true);
        m.set(1, 1, true);
        m.set(2, 2, true);
        assert_eq!(m.get(0, 1), false);
        assert_eq!(m.get(0, 0), true);
        assert_eq!(m.get(1, 1), true);
        assert_eq!(m.get(2, 2), true);
    }

    #[test]
    fn character() {
        let mut m = Matrix::new(6, 3, 'a');
        m.set(0, 0, 'b');
        m.set(1, 1, 'c');
        m.set(2, 2, 'd');
        assert_eq!(m.get(0, 1), 'a');
        assert_eq!(m.get(0, 0), 'b');
        assert_eq!(m.get(1, 1), 'c');
        assert_eq!(m.get(2, 2), 'd');
        m.print_grid();
    }
}
