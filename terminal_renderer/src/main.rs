extern crate core;

use simulation::entities::material::Material;
use simulation::sand_sim::{CellContentWrapper, Simulation};
use simulation::universe::Universe;
use std::io;

const S: Material = Material::Sand;
const A: Material = Material::Air;
const W: Material = Material::Water;
const w: Material = Material::WaterGenerator;
const D: Material = Material::Wood;
const V: Material = Material::Vapor;
const F: Material = Material::Fire;

fn main() {
    // let mut sim = Simulation::new(30, 15);
    //
    // let mut fill_area = vec![Material::Air; 30];
    // fill_area[10] = Material::SandGenerator;
    // fill_area[20] = Material::WaterGenerator;

    // let mut sim = Simulation::new(12, 30);

    // let mut fill_area = [
    //     A,S,A,A,S,S,S,A,S,A,A,A,
    //     A,S,A,A,S,A,S,A,S,A,A,A,
    //     A,S,S,A,S,S,S,A,S,S,S,A,
    // ];
    // let fill_area = vec![
    //     A, S, A, A, S, S, S, A, S, A, A, A, A, S, A, A, S, A, S, A, S, A, A, A, A, S, S, A, S, S,
    //     S, A, S, S, S, A,
    // ];

    let sim = Simulation::new(5, 5);
    //
    // let mut fill_area = vec![Material::Air; 5];
    // fill_area[1] = Material::SandGenerator;
    // fill_area[3] = Material::WaterGenerator;

    let fill_area_2d = [
        [A, A, F, A, A],
        [A, A, A, A, A],
        [A, A, A, A, A],
        [A, A, A, A, A],
        [D, D, D, D, D],
    ];

    let fill_area = fill_area_2d.concat();

    sim.par_fill(&fill_area);
    let mut buf = String::new();

    loop {
        draw(&sim.universe);
        let _ignored = io::stdin().read_line(&mut buf);
        // sleep(Duration::from_millis(40));
        sim.par_tick();
    }
}

pub fn get_as_string(area: &[CellContentWrapper], width: usize) -> String {
    let lines: Vec<String> = area
        .chunks(width)
        .map(|chunk| chunk.iter().map(content_to_char).collect::<String>())
        .collect();
    lines.join("\n")
}

pub fn draw(universe: &Universe<CellContentWrapper>) {
    // clear screen
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // print!("{}[2J", 27 as char);
    // print!("\x1B[2J\x1B[1;1H");

    print!("{esc}c", esc = 27 as char);

    println!("{}", get_as_string(&universe.area, universe.width));
}

fn content_to_char(content: &CellContentWrapper) -> char {
    match content.lock().unwrap().material {
        Material::Sand => '■',
        Material::SandGenerator => 'S',
        Material::Air => ' ',
        Material::Water => '◉',
        Material::WaterGenerator => 'W',
        Material::Fire => 'f',
        Material::Smoke => '~',
        Material::Vapor => '|',
        Material::Wood => '=',
    }
}
