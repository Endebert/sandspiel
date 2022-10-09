use simulation::sand_sim::Simulation;
use simulation::universe::{Cell, CellKind, Universe};
use std::fmt::{Display, Formatter};
use std::io;

fn main() {
    let mut sim = Simulation::new(15, 15);

    let mut fill_area = vec![CellKind::Air; 15];
    fill_area[5] = CellKind::SandGenerator;
    fill_area[10] = CellKind::WaterGenerator;

    sim.universe.fill(&*fill_area);
    let mut buf = String::new();

    loop {
        sim.tick();
        draw(&sim.universe);
        let _ignored = io::stdin().read_line(&mut buf);
        // sleep(Duration::from_millis(250))
    }
}

fn draw(universe: &Universe) {
    let cell_to_char = |cell: &Cell| -> char {
        match cell.kind {
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
