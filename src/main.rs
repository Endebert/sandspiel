extern crate core;

use std::thread::sleep;
use std::time::Duration;

mod sand_sim;
mod sandspiel_static_5;

const S: sand_sim::CellMaterial = sand_sim::CellMaterial::Sand;
const W: sand_sim::CellMaterial = sand_sim::CellMaterial::Water;
const A: sand_sim::CellMaterial = sand_sim::CellMaterial::Air;

fn main() {
    let snapshot_t0: sandspiel_static_5::Snapshot = [
        [A, A, S, A, A],
        [A, A, A, A, A],
        [A, A, A, A, A],
        [A, A, A, A, A],
        [A, A, A, A, A],
    ];
    let mut spiel = sandspiel_static_5::SandspielStatic5::from_snapshot(snapshot_t0);

    loop {
        print_snapshot(spiel.snapshot());
        spiel.run();
        sleep(Duration::from_millis(500));
    }
}

fn print_snapshot(snapshot: sandspiel_static_5::Snapshot) {
    let printable_snapshot: [String; 5] = snapshot.map(|row| {
        String::from_iter(
            row.map(|material| match material {
                sand_sim::CellMaterial::Sand => 's',
                sand_sim::CellMaterial::SandGenerator => 'S',
                sand_sim::CellMaterial::Water => 'w',
                sand_sim::CellMaterial::WaterGenerator => 'W',
                sand_sim::CellMaterial::Air => ' ',
            })
            .iter(),
        )
    });

    eprintln!("printable_snapshot = {:#?}", printable_snapshot);
}
