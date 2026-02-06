
use macroquad::prelude::*;

mod physics;
mod world;
mod cell;

use world::{World, GRID_HEIGHT, GRID_WIDTH};
use cell::Cell;

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
    let mut radius = 2;
    loop {
        clear_background(BLACK);
        
        (current_material, radius) = world.handle_keyboard(current_material, radius);
        world.handle_sendung_mit_der_maus(current_material,radius);

        world.set(GRID_HEIGHT / 2 + 25, 10, Cell::Sand);
        
        world.update();
        world.draw();

        next_frame().await
    }
}
