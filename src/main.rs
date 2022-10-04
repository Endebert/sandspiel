extern crate core;

use crate::sand_sim::{Area, Cell, CellMaterial, Sandspiel};
use std::thread::sleep;
use std::time::Duration;

mod sand_sim;
mod sim_printer;

const A: Cell = Cell {
    material: CellMaterial::Air,
    velocity: 0,
};
const S: Cell = Cell {
    material: CellMaterial::Sand,
    velocity: 0,
};

fn main() {
    // let mut sim = gen_sim1();
    let mut sim = gen_sim1();

    loop {
        sim_printer::print_sim(&sim);
        sim.update();
        sleep(Duration::from_millis(1000))
    }
}

fn gen_sim1() -> Sandspiel {
    let width: u16 = 5;
    let height: u16 = 5;
    let area: Area = vec![
        vec![A, A, S, A, A],
        vec![A, A, S, A, A],
        vec![A, A, S, A, A],
        vec![A, A, S, A, A],
        vec![A, A, S, A, A],
    ];
    Sandspiel::new(width, height, area)
}
