use crate::{Cell, Sandspiel};

const AIR: char = ' ';
const SAND: char = '■';
const WATER: char = '◉';
const SANDGENERATOR: char = 'S';
const WATERGENERATOR: char = 'W';

fn cell_to_char(material: &Cell) -> char {
    match material {
        Cell::Sand => SAND,
        Cell::SandGenerator => SANDGENERATOR,
        Cell::Air => AIR,
        Cell::Water => WATER,
        Cell::WaterGenerator => WATERGENERATOR,
    }
}
pub fn print_sim(sim: &Sandspiel) {
    // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for row in sim.area.chunks(sim.width as usize) {
        println!("{}", row.iter().map(cell_to_char).collect::<String>())
    }
}
