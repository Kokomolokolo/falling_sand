use std::io::Empty;

use macroquad::prelude::*;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 150;
const CELL_SIZE: f32 = 4.0;
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Sand,
    Stone,
    Water,
}

struct World {
    grid: Vec<Vec<Cell>>
}

impl World {
    fn new() -> Self {
        Self {
            grid: vec![vec![Cell::Empty; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.grid.get(y)?.get(x).copied() // das muss wohl so, ich verstehe nur so halb. Geht auch unsicherer einfacher.
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if y < GRID_HEIGHT && x < GRID_WIDTH {
            self.grid[y][x] = cell;
        }
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        let cell = self.get(x, y);
        if cell == Some(Cell::Empty) {
            return true
        }
        return false
    }

    fn draw(&self) {
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

    fn update(&mut self) {
        // Von unten nach oben durchgehen (rev) damit partikel fallen können.
        for y in (0..GRID_HEIGHT).rev() {
            for x in 0..GRID_WIDTH {
                let cell = self.grid[y][x];
                // jede cell hat eigene Sachen dir mit ihr passieren.
                match cell {
                    Cell::Sand => {
                        if y + 1 < GRID_HEIGHT && self.is_empty(x, y + 1) {
                            self.set(x, y, Cell::Empty);
                            self.set(x, y +  1, Cell::Sand);
                        } else {
                            // Diagonale Bewegung. Ist rechts oder links frei?
                            let left_free = x > 0 && self.is_empty(x - 1, y + 1);
                            let right_free: bool = x + 1 < GRID_WIDTH && self.is_empty(x + 1, y + 1);

                            if left_free && right_free {
                                // go left hat 50% true zu sein
                                let go_left = rand::gen_range(0, 2) == 0; // 2 ist exkludiert.
                                let new_x = if go_left { x - 1 } else { x + 1};
                                // Sand auf die zufällig gewählte Stelle gesetzt.
                                self.set(new_x, y + 1, Cell::Sand);
                                self.set(x, y, Cell::Empty);
                            } else if left_free {
                                self.set(x - 1, y + 1, Cell::Sand);
                                self.set(x, y, Cell::Empty);
                            } else if right_free {
                                self.set(x + 1, y + 1, Cell::Sand);
                                self.set(x, y, Cell::Empty);
                            }
                        }
                    }
                    Cell::Water => {
                        // Nach unten Fallen
                        if y + 1 < GRID_HEIGHT && self.is_empty(x, y + 1) {
                            self.set(x, y, Cell::Empty);
                            self.set(x, y +  1, Cell::Water);
                        } 
                        // Diagonale Bewegung. Ist rechts oder links frei?
                        else {
                            let left_free = x > 0 && self.is_empty(x - 1, y + 1);
                            let right_free: bool = x + 1 < GRID_WIDTH && self.is_empty(x + 1, y + 1);

                            if left_free && right_free {
                                // go left hat 50% true zu sein
                                let go_left = rand::gen_range(0, 2) == 0; // 2 ist exkludiert.
                                let new_x = if go_left { x - 1 } else { x + 1};
                                // Sand auf die zufällig gewählte Stelle gesetzt.
                                self.set(new_x, y + 1, Cell::Water);
                                self.set(x, y, Cell::Empty);
                            } else if left_free {
                                self.set(x - 1, y + 1, Cell::Water);
                                self.set(x, y, Cell::Empty);
                            } else if right_free {
                                self.set(x + 1, y + 1, Cell::Water);
                                self.set(x, y, Cell::Empty);
                            }
                            // Schräg nach rechts/links
                            else {
                                let left_free: bool = x > 0 && self.is_empty(x - 1, y);
                                let right_free: bool = x + 1 < GRID_WIDTH && self.is_empty(x + 1, y);

                                if left_free && right_free {
                                    // go left hat 50% true zu sein
                                    let go_left = rand::gen_range(0, 2) == 0; // 2 ist exkludiert.
                                    let new_x = if go_left { x - 1 } else { x + 1};
                                    // Sand auf die zufällig gewählte Stelle gesetzt.
                                    self.set(new_x, y, Cell::Water);
                                    self.set(x, y, Cell::Empty);
                                } else if left_free {
                                    self.set(x - 1, y, Cell::Water);
                                    self.set(x, y, Cell::Empty);
                                } else if right_free {
                                    self.set(x + 1, y, Cell::Water);
                                    self.set(x, y, Cell::Empty);
                                }
                            }
                        }
                    }
                    Cell::Empty | Cell::Stone  => {
                        continue
                    }
                }
            }
        }
    }

    fn handle_sendung_mit_der_maus(&mut self, cell: Cell) {
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            
            let grid_x = (mouse_x / CELL_SIZE) as usize;
            let grid_y = (mouse_y / CELL_SIZE) as usize;
            
            self.set(grid_x, grid_y, cell);
        }
    }

    fn handle_keyboard(& self, current: Cell) -> Cell {
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

#[macroquad::main("Falling Sand")]
async fn main() {
    let mut world = World::new(); // Welt Instance

    // Steinpyramide bauen
    let pyramid_center_x = GRID_WIDTH / 2;
    let pyramid_base_y = GRID_HEIGHT - 20;
    let pyramid_height = 15;

    for layer in 0..pyramid_height {
        let y = pyramid_base_y - layer;
        let width = pyramid_height - layer;
        
        for offset in 0..width {
            let x_left = pyramid_center_x - width / 2 + offset;
            world.set(x_left, y, Cell::Stone);
        }
    }

    let mut current_material: Cell = Cell::Sand;
    loop {
        clear_background(BLACK);
        
        current_material = world.handle_keyboard(current_material);
        world.handle_sendung_mit_der_maus(current_material);

        world.set(GRID_HEIGHT / 2 + 25, 10, Cell::Sand);
        
        world.update();
        world.draw();

        next_frame().await
    }
}
