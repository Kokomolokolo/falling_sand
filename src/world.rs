

use crate::{cell::{CELL_SIZE, Cell}, physics::{update_sand, update_water}};
use macroquad::prelude::*;

pub const GRID_WIDTH: usize = 200;
pub const GRID_HEIGHT: usize = 150;

pub struct World {
    pub grid: Vec<Vec<Cell>>
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![Cell::Empty; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.grid.get(y)?.get(x).copied() // das muss wohl so, ich verstehe nur so halb. Geht auch unsicherer einfacher.
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if y < GRID_HEIGHT && x < GRID_WIDTH {
            self.grid[y][x] = cell;
        }
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        let cell = self.get(x, y);
        if cell == Some(Cell::Empty) {
            return true
        }
        return false
    }

    pub fn draw(&self) {
        // Drawing the World
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let cell = self.grid[y][x];
                // Farbe malen, je nachdem was für eine Pixel
                let color =  match cell {
                    Cell::Empty => continue,
                    Cell::Sand => YELLOW,
                    Cell::Stone => GRAY,
                    Cell::Water => BLUE,
                    Cell::Fire => ORANGE,
                };

                // RECTAGLE, gemalt an x position * cell_size
                draw_rectangle(
                    x as f32 * CELL_SIZE, 
                    y as f32 * CELL_SIZE, 
                    CELL_SIZE,
                    CELL_SIZE,
                    color
                );
            }
        }
    }
    pub fn update(&mut self) {
        for y in (0..GRID_HEIGHT).rev() {
            for x in 0..GRID_WIDTH {
                let cell = self.grid[y][x];
                match cell {
                    Cell::Sand => update_sand(self, x, y),
                    Cell::Water => update_water(self, x, y),
                    Cell::Stone | Cell::Empty | _ => {}
                }
            }
        }
    }

    pub fn handle_sendung_mit_der_maus(&mut self, cell: Cell) {
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            
            let grid_x = (mouse_x / CELL_SIZE) as usize;
            let grid_y = (mouse_y / CELL_SIZE) as usize;
            
            self.set(grid_x, grid_y, cell);
        }
    }

    pub fn handle_keyboard(& self, current: Cell) -> Cell {
        if is_key_pressed(KeyCode::Key1) {
            return Cell::Sand;
        }
        if is_key_pressed(KeyCode::Key2) {
            return Cell::Stone;
        }
        if is_key_pressed(KeyCode::Key3) {
            return Cell::Water;
        }
        current
    }
}