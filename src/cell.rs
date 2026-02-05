
pub const CELL_SIZE: f32 = 4.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Sand,
    Stone,
    Water,
    Fire,
}
