// extern crate core;
//
// use crate::sand_sim::{Cell, CellMaterial, SandSimulation};
// use std::ops::Add;
// use std::thread::sleep;
// use std::time::Duration;

// mod sand_sim;
// mod sandspiel_terminal;

mod sand_sim;

// #[derive(Debug)]
// struct StoredSomeStruct {
//     // string: String,
//     val: i32,
// }
//
// struct SomeStruct<'a> {
//     // string: &'a String,
//     val: &'a mut i32,
// }

fn main() {
    println!("Hello, world!");
    // let mut arr = vec![
    //     StoredSomeStruct {
    //         // string: String::from("Hello"),
    //         val: 32,
    //     },
    //     StoredSomeStruct {
    //         // string: String::from("Hello"),
    //         val: 99,
    //     },
    // ];
    // let blub = &mut arr[0];
    // let bib = SomeStruct {
    //     // string: &blub.string,
    //     val: &mut blub.val,
    // };
    //
    // arr.swap(0, 1);

    // bib.string = String::from("World");
    // *bib.val += 3;

    // println!("{:?}", arr)

    // let mut sim = SandspielTerminal::new(16, 16);
    // sim.set_cell(Cell {
    //     x: 16,
    //     y: 0,
    //     material: CellMaterial::Sand,
    // });
    // sim.set_cell(Cell {
    //     x: 16,
    //     y: 16,
    //     material: CellMaterial::Sand,
    // });
    // sim.set_cell(Cell {
    //     x: 1,
    //     y: 5,
    //     material: CellMaterial::WaterGenerator,
    // });
    // sim.set_cell(Cell {
    //     x: 3,
    //     y: 0,
    //     material: CellMaterial::SandGenerator,
    // });
    // sim.set_cell(Cell {
    //     x: 3,
    //     y: 1,
    //     material: CellMaterial::Air,
    // });
    //
    // sim.set_cell(Cell {
    //     x: 3,
    //     y: 2,
    //     material: CellMaterial::Air,
    // });
    // loop {
    //     sim.draw();
    //     sim.update();
    //     sleep(Duration::from_millis(125));
    // }
    // loop {
    //     sim.draw();
    //     sim.update();
    //     sleep(Duration::from_millis(125));
    // }
}
