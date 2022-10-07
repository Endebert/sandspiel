extern crate core;

use crate::sand_sim::{Cell, Sandspiel};
use std::io;
use std::thread::sleep;
use std::time::Duration;

mod sand_sim;
mod universe;
mod wgpu_renderer;

const A: Cell = Cell::Air;
const s: Cell = Cell::Sand;
const S: Cell = Cell::SandGenerator;

const W: Cell = Cell::WaterGenerator;
const w: Cell = Cell::Water;

fn main() {
    pollster::block_on(wgpu_renderer::run());
}

fn run_terminal() {
    let mut sim = gen_sim4();

    let mut buf = String::new();

    loop {
        // clear screen
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("{esc}c", esc = 27 as char);

        println!("{}", sim.universe);
        sim.tick();
        let _ignored = io::stdin().read_line(&mut buf);
        // sleep(Duration::from_millis(250))
    }
}

fn gen_sim4() -> Sandspiel {
    let width: usize = 30;
    let height: usize = 30;

    let mut sim = Sandspiel::new(width, height);
    sim.universe.set_cell(Cell::WaterGenerator, 10);
    sim.universe.set_cell(Cell::SandGenerator, 20);

    sim
}
