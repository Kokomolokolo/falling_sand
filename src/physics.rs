use crate::world::{GRID_HEIGHT, GRID_WIDTH, World};
use crate::cell::Cell;
use macroquad::prelude::*;

pub fn update_sand(world: &mut World, x: usize, y: usize) {
    if y + 1 < GRID_HEIGHT && world.is_empty(x, y + 1) {
        world.set(x, y, Cell::Empty);
        world.set(x, y +  1, Cell::Sand);
    } else {
        // Diagonale Bewegung. Ist rechts oder links frei?
        let left_free = x > 0 && world.is_empty(x - 1, y + 1);
        let right_free: bool = x + 1 < GRID_WIDTH && world.is_empty(x + 1, y + 1);

        if left_free && right_free {
            // go left hat 50% true zu sein
            let go_left = rand::gen_range(0, 2) == 0; // 2 ist exkludiert.
            let new_x = if go_left { x - 1 } else { x + 1};
            // Sand auf die zufällig gewählte Stelle gesetzt.
            world.set(new_x, y + 1, Cell::Sand);
            world.set(x, y, Cell::Empty);
        } else if left_free {
            world.set(x - 1, y + 1, Cell::Sand);
            world.set(x, y, Cell::Empty);
        } else if right_free {
            world.set(x + 1, y + 1, Cell::Sand);
            world.set(x, y, Cell::Empty);
        }
    }
}

pub fn update_water(world: &mut World, x: usize, y: usize) {
    // Nach unten Fallen
    if y + 1 < GRID_HEIGHT && world.is_empty(x, y + 1) {
        world.set(x, y, Cell::Empty);
        world.set(x, y +  1, Cell::Water);
    } 
    // Diagonale Bewegung. Ist rechts oder links frei?
    else {
        let left_free = x > 0 && world.is_empty(x - 1, y + 1);
        let right_free: bool = x + 1 < GRID_WIDTH && world.is_empty(x + 1, y + 1);

        if left_free && right_free {
            // go left hat 50% true zu sein
            let go_left = rand::gen_range(0, 2) == 0; // 2 ist exkludiert.
            let new_x = if go_left { x - 1 } else { x + 1};
            // Sand auf die zufällig gewählte Stelle gesetzt.
            world.set(new_x, y + 1, Cell::Water);
            world.set(x, y, Cell::Empty);
        } else if left_free {
            world.set(x - 1, y + 1, Cell::Water);
            world.set(x, y, Cell::Empty);
        } else if right_free {
            world.set(x + 1, y + 1, Cell::Water);
            world.set(x, y, Cell::Empty);
        }
        // Schräg nach rechts/links
        else {
            let left_free: bool = x > 0 && world.is_empty(x - 1, y);
            let right_free: bool = x + 1 < GRID_WIDTH && world.is_empty(x + 1, y);

            if left_free && right_free {
                // go left hat 50% true zu sein
                let go_left = rand::gen_range(0, 2) == 0; // 2 ist exkludiert.
                let new_x = if go_left { x - 1 } else { x + 1};
                // Sand auf die zufällig gewählte Stelle gesetzt.
                world.set(new_x, y, Cell::Water);
                world.set(x, y, Cell::Empty);
            } else if left_free {
                world.set(x - 1, y, Cell::Water);
                world.set(x, y, Cell::Empty);
            } else if right_free {
                world.set(x + 1, y, Cell::Water);
                world.set(x, y, Cell::Empty);
            }
        }
    }
}

pub fn update_fire(world: &mut World, x: usize, y: usize) {
    
}