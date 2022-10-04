use crate::{CellMaterial, Sandspiel};

const AIR: char = ' ';
const SAND: char = '■';
const WATER: char = '◉';
const SANDGENERATOR: char = 'S';
const WATERGENERATOR: char = 'W';

fn get_char_from_material(material: CellMaterial) -> char {
    match material {
        CellMaterial::Sand => SAND,
        CellMaterial::SandGenerator => SANDGENERATOR,
        CellMaterial::Air => AIR,
        // CellMaterial::Water => WATER,
        // CellMaterial::WaterGenerator => WATERGENERATOR,
    }
}
pub fn print_sim(sim: &Sandspiel) {
    // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for row in &sim.area {
        println!(
            "{}",
            row.iter()
                .map(|cell| get_char_from_material(cell.material))
                .collect::<String>()
        )
    }
}

pub fn print_sim_velocities(sim: &Sandspiel) {
    // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for row in &sim.area {
        println!(
            "{}",
            row.iter()
                .map(|cell| cell.velocity.to_string())
                .collect::<String>()
        )
    }
}
