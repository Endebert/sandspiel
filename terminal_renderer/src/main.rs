use simulation::sand_sim::Simulation;
use simulation::universe::{Cell, CellInternal, CellKind, Universe};
use std::fmt::{Display, Formatter};
use std::io;
use std::thread::sleep;
use std::time::Duration;

const S: CellKind = CellKind::Sand;
const A: CellKind = CellKind::Air;

fn main() {
    let mut sim = Simulation::new(30, 30);

    let mut fill_area = vec![CellKind::Air; 30];
    fill_area[10] = CellKind::SandGenerator;
    fill_area[20] = CellKind::WaterGenerator;

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

    // let mut sim = Simulation::new(5, 5);
    //
    // let mut fill_area = vec![CellKind::Air; 5];
    // fill_area[1] = CellKind::SandGenerator;
    // fill_area[3] = CellKind::WaterGenerator;

    sim.universe.fill(&*fill_area);
    let mut buf = String::new();

    loop {
        sim.tick();
        draw(&sim.universe);
        // let _ignored = io::stdin().read_line(&mut buf);
        sleep(Duration::from_millis(40))
    }
}

pub fn draw(universe: &Universe) {
    let cell_to_char = |cell: &CellInternal| -> char {
        match cell.kind() {
            CellKind::Sand => '■',
            CellKind::SandGenerator => 'S',
            CellKind::Air => ' ',
            CellKind::Water => '◉',
            CellKind::WaterGenerator => 'W',
        }
    };

    // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // print!("{esc}c", esc = 27 as char);

    for chunk in universe.area.chunks(universe.width) {
        println!("{}", chunk.iter().map(cell_to_char).collect::<String>());
    }
}
