extern crate core;

use crate::sand_sim::{Cell, CellMaterial, SandSimulation};
use crate::sandspiel_terminal::SandspielTerminal;
use std::thread::sleep;
use std::time::Duration;

mod sand_sim;
mod sandspiel_terminal;

fn main() {
    println!("Hello, world!");

    let mut sim = SandspielTerminal::new(16, 16);
    // sim.set_cell(Cell {
    //     x: 16,
    //     y: 0,
    //     material: CellMaterial::Sand,
    // });
    // sim.set_cell(Cell {
    //     x: 16,
    //     y: 16,
    //     material: CellMaterial::Sand,
    // });
    // sim.set_cell(Cell {
    //     x: 1,
    //     y: 5,
    //     material: CellMaterial::WaterGenerator,
    // });
    sim.set_cell(Cell {
        x: 3,
        y: 0,
        material: CellMaterial::SandGenerator,
    });
    sim.set_cell(Cell {
        x: 3,
        y: 1,
        material: CellMaterial::Air,
    });

    sim.set_cell(Cell {
        x: 3,
        y: 2,
        material: CellMaterial::Air,
    });
    // loop {
    //     sim.draw();
    //     sim.update();
    //     sleep(Duration::from_millis(125));
    // }
    loop {
        sim.draw();
        sim.update();
        sleep(Duration::from_millis(125));
    }
}
