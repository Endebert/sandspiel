mod utils;

use simulation::sand_sim::Simulation;
use simulation::universe::{Cell, CellKind, Universe};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct WasmPackRenderer {
    sim: Simulation,
}

#[wasm_bindgen]
impl WasmPackRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = Simulation::new(width, height);
        let mut fill_area = vec![CellKind::Air; width];
        fill_area[width / 2] = CellKind::SandGenerator;

        sim.universe.fill(&*fill_area);
        Self { sim }
    }

    pub fn tick(&mut self) {
        self.sim.tick();
    }

    pub fn get_data(&self) -> Clamped<Vec<u8>> {
        Clamped(to_u8(&self.sim.universe))
    }
}

const AIR_COLOR: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const WATER_COLOR: [u8; 4] = [0, 0, 0xff, 0xff];
const SAND_COLOR: [u8; 4] = [0xff, 0xff, 0, 0xff];
const WATER_GENERATOR_COLOR: [u8; 4] = [0, 0xff, 0xff, 0xff];
const SAND_GENERATOR_COLOR: [u8; 4] = [0xff, 0, 0xff, 0xff];

fn to_u8(universe: &Universe) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(universe.area.len() * 4);

    for cell in &universe.area {
        let color = match cell.kind {
            CellKind::Sand => &SAND_COLOR,
            CellKind::SandGenerator => &SAND_GENERATOR_COLOR,
            CellKind::Water => &WATER_COLOR,
            CellKind::WaterGenerator => &WATER_GENERATOR_COLOR,
            CellKind::Air => &AIR_COLOR,
        };

        out.extend_from_slice(color);

        // out[4 * i..4 * i + 4].copy_from_slice(color);
    }

    out
}
