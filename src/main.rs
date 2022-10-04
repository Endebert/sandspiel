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
const s: Cell = Cell {
    material: CellMaterial::Sand,
    velocity: 0,
};
const S: Cell = Cell {
    material: CellMaterial::SandGenerator,
    velocity: 0,
};

fn main() {
    // let mut sim = gen_sim1();
    // let mut sim = gen_sim2();
    let mut sim = gen_sim3();

    loop {
        sim_printer::print_sim(&sim);
        sim.update();
        sleep(Duration::from_millis(125))
    }
}

fn gen_sim1() -> Sandspiel {
    let width: u16 = 5;
    let height: u16 = 5;
    let area: Area = vec![
        vec![A, A, s, A, A],
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
    ];
    Sandspiel::new(width, height, area)
}

fn gen_sim2() -> Sandspiel {
    let width: u16 = 30;
    let height: u16 = 30;
    let mut area = Sandspiel::gen_area(width, height);
    area[0][20] = Cell::new(CellMaterial::Sand, 0);
    Sandspiel::new(width, height, area)
}

fn gen_sim3() -> Sandspiel {
    let width: u16 = 30;
    let height: u16 = 30;
    let mut area = Sandspiel::gen_area(width, height);
    area[0][20] = Cell::new(CellMaterial::SandGenerator, 0);
    Sandspiel::new(width, height, area)
}
