extern crate core;

use crate::sand_sim::{Cell, Sandspiel};
use std::io;
use std::thread::sleep;
use std::time::Duration;

mod sand_sim;
mod sim_printer;

const A: Cell = Cell::Air;
const s: Cell = Cell::Sand;
const S: Cell = Cell::SandGenerator;

const W: Cell = Cell::WaterGenerator;
const w: Cell = Cell::Water;

fn main() {
    // let mut sim = gen_sim1();
    // let mut sim = gen_sim2();
    // let mut sim = gen_sim3();
    let mut sim = gen_sim4();
    // let mut sim = gen_sim5();
    // let mut sim = gen_sim6();
    // let mut sim = gen_sim7();
    // let mut sim = gen_sim8();
    let mut buf = String::new();

    loop {
        sim_printer::print_sim(&sim);
        sim.update();
        io::stdin().read_line(&mut buf);
        // sleep(Duration::from_millis(250))
    }
}

fn gen_sim1() -> Sandspiel {
    let width: u16 = 5;
    let height: u16 = 5;
    let area: Vec<Cell> = vec![
        vec![A, A, s, A, A],
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
    ]
    .concat();
    Sandspiel::new(width, height, area)
}

fn gen_sim2() -> Sandspiel {
    let width: u16 = 30;
    let height: u16 = 30;
    let mut area = Sandspiel::gen_area(width, height);
    area[15] = Cell::Sand;
    Sandspiel::new(width, height, area)
}

fn gen_sim3() -> Sandspiel {
    let width: u16 = 30;
    let height: u16 = 30;
    let mut area = Sandspiel::gen_area(width, height);
    area[15] = Cell::SandGenerator;
    Sandspiel::new(width, height, area)
}

fn gen_sim4() -> Sandspiel {
    let width: u16 = 30;
    let height: u16 = 30;
    let mut area = Sandspiel::gen_area(width, height);
    area[20] = Cell::SandGenerator;
    area[10] = Cell::WaterGenerator;
    Sandspiel::new(width, height, area)
}

fn gen_sim5() -> Sandspiel {
    let width: u16 = 30;
    let height: u16 = 30;
    let mut area = Sandspiel::gen_area(width, height);
    area[15] = Cell::WaterGenerator;
    Sandspiel::new(width, height, area)
}

fn gen_sim6() -> Sandspiel {
    let width: u16 = 5;
    let height: u16 = 5;
    let area: Vec<Cell> = vec![
        vec![A, A, w, A, A],
        vec![A, A, w, A, A],
        vec![A, A, w, A, A],
        vec![A, A, w, A, A],
        vec![A, A, w, A, A],
    ]
    .concat();
    Sandspiel::new(width, height, area)
}

fn gen_sim7() -> Sandspiel {
    let width: u16 = 5;
    let height: u16 = 5;
    let area: Vec<Cell> = vec![
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
        vec![A, A, A, A, A],
        vec![A, A, w, A, A],
        vec![A, A, w, A, A],
    ]
    .concat();
    Sandspiel::new(width, height, area)
}

fn gen_sim8() -> Sandspiel {
    let width: u16 = 3;
    let height: u16 = 3;
    let area: Vec<Cell> = vec![vec![A, S, A], vec![A, A, A], vec![A, A, A]].concat();
    Sandspiel::new(width, height, area)
}
