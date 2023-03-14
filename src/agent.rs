use crate::color::Color;

#[derive(Debug)]
pub struct Agent {
    pub row: usize,
    pub col: usize,
    pub color: Color,
}
