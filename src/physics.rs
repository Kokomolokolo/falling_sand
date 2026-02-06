use crate::world::{self, GRID_HEIGHT, GRID_WIDTH, World};
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
    if y + 1 < GRID_HEIGHT && (world.is_empty(x, y + 1) || world.get(x, y + 1) == Some(Cell::Fire)) {
        world.set(x, y, Cell::Empty);
        world.set(x, y +  1, Cell::Water);
    } 
    // Diagonale Bewegung. Ist rechts oder links frei?
    else {
        let left_free = x > 0 && (world.is_empty(x - 1, y + 1) || world.get(x - 1, y + 1) == Some(Cell::Fire));
        let right_free: bool = x + 1 < GRID_WIDTH && (world.is_empty(x + 1, y + 1) || world.get(x + 1, y + 1) == Some(Cell::Fire));

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
    // Feuer bewegt sich nicht jeden Frame
    let moving = rand::gen_range(0, 4) == 1;
    if !moving {
        return;
    }
    if y + 1 < GRID_HEIGHT && world.is_empty(x, y + 1) {
        world.set(x, y, Cell::Empty);
        world.set(x, y +  1, Cell::Fire);
    }
    if world.get(x, y + 1) == Some(Cell::Fire) && world.get(x, y + 2) == Some(Cell::Fire) {
        let remove_fire = rand::gen_range(0, 250) == 1;
        if remove_fire { // Nur wenn moving auch "weggehen"
            world.set(x, y, Cell::Empty);
        }
    }
    // Smoke über dem Feuer spawnen
    if y > 0 && world.is_empty(x, y - 1) {
        let spawn = rand::gen_range(0, 10) == 1;
        if spawn {
            world.set(x, y - 1, Cell::Smoke);
        }
    }
}

pub fn update_smoke(world: &mut World, x: usize, y: usize) {
    // Smoke sollte sich mit der Zeit auflösen
    let disappear = rand::gen_range(0, 1000) == 1;
    if disappear {
        world.set(x, y, Cell::Empty);
        return;
    }
    // Smoke nach oben bewegen
    // Nach unten Fallen
    if y > 0 && world.is_empty(x, y - 1) {
        world.set(x, y, Cell::Empty);
        world.set(x, y - 1, Cell::Smoke);
    }

    // Diagonale Bewegung. Ist rechts oder links frei?
    else if y > 0 {
        let left_free = x > 0 && world.is_empty(x - 1, y - 1);
        let right_free: bool = x + 1 < GRID_WIDTH && world.is_empty(x + 1, y - 1);

        if left_free && right_free {
            // go left hat 50% true zu sein
            let go_left = rand::gen_range(0, 2) == 0; // 2 ist exkludiert.
            let new_x = if go_left { x - 1 } else { x + 1};
            // Sand auf die zufällig gewählte Stelle gesetzt.
            world.set(new_x, y - 1, Cell::Smoke);
            world.set(x, y, Cell::Empty);
        } else if left_free {
            world.set(x - 1, y - 1, Cell::Smoke);
            world.set(x, y, Cell::Empty);
        } else if right_free {
            world.set(x + 1, y - 1, Cell::Smoke);
            world.set(x, y, Cell::Empty);
        }
    }
    
}