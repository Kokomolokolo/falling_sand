
pub const CELL_SIZE: f32 = 2.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Sand,
    Stone,
    Water,
    Fire,
    Smoke,
}
