mod utils;

use simulation::sand_sim::Simulation;
use simulation::universe::{CellContent, Material, Position, Universe};
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
        let mut fill_area = vec![Material::Air; width];
        fill_area[width / 3] = Material::SandGenerator;
        fill_area[width / 2] = Material::WaterGenerator;

        sim.universe.fill(&fill_area);
        Self { sim }
    }

    pub fn tick(&mut self) {
        self.sim.tick();
    }

    pub fn get_data(&self) -> Clamped<Vec<u8>> {
        Clamped(to_u8(&self.sim.universe))
    }

    pub fn add_material(&mut self, material: &str, x: isize, y: isize) {
        let position = Position::new(x.unsigned_abs(), y.unsigned_abs());
        let material = match material {
            "sand" => Material::Sand,
            "water" => Material::Water,
            "fire" => Material::Fire,
            "smoke" => Material::Smoke,
            "vapor" => Material::Vapor,
            "wood" => Material::Wood,
            _ => panic!("Tried to add unknown material '{material}'"),
        };
        let content = CellContent::new(material, true, 0);

        self.sim
            .universe
            .get_cell(&position)
            .unwrap()
            .lock()
            .unwrap()
            .clone_from(&content);
    }
}

const AIR_COLOR: [u8; 4] = [0xff, 0xff, 0xff, 0x00];
const WATER_COLOR: [u8; 4] = [0, 0, 0xff, 0xff];
const SAND_COLOR: [u8; 4] = [0xff, 0xff, 0, 0xff];
const WATER_GENERATOR_COLOR: [u8; 4] = [0, 0xff, 0xff, 0xff];
const SAND_GENERATOR_COLOR: [u8; 4] = [0xff, 0, 0xff, 0xff];
const FIRE_COLOR: [u8; 4] = [0xff, 0, 0, 0xff];
const SMOKE_COLOR: [u8; 4] = [0x77, 0x77, 0x77, 0x77];
const VAPOR_COLOR: [u8; 4] = [0, 0, 0xff, 0x77];
const WOOD_COLOR: [u8; 4] = [0xDE, 0xB8, 0x87, 0xff];

fn to_u8(universe: &Universe) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(universe.area.len() * 4);

    for cell in &universe.area {
        let color = match cell.lock().unwrap().material {
            Material::Sand => &SAND_COLOR,
            Material::SandGenerator => &SAND_GENERATOR_COLOR,
            Material::Water => &WATER_COLOR,
            Material::WaterGenerator => &WATER_GENERATOR_COLOR,
            Material::Air => &AIR_COLOR,
            Material::Fire => &FIRE_COLOR,
            Material::Smoke => &SMOKE_COLOR,
            Material::Vapor => &VAPOR_COLOR,
            Material::Wood => &WOOD_COLOR,
        };

        out.extend_from_slice(color);

        // out[4 * i..4 * i + 4].copy_from_slice(color);
    }

    out
}
