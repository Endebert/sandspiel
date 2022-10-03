use crate::sand_sim::{Cell, CellMaterial, SandSimulation};

// const CLEAR_SCREEN: String = format!("{esc}[2J{esc}[1;1H", esc = 27 as char);

const AIR: char = ' ';
const SAND: char = 's';
const WATER: char = 'w';
const SANDGENERATOR: char = 'S';
const WATERGENERATOR: char = 'W';

// impl SandspielTerminal {
//     pub fn new(width: i16, height: i16) -> Self {
//         Self {
//             width,
//             height,
//             area: vec![vec![AIR; width as usize]; height as usize],
//         }
//     }
//
//     fn get_material_from_char(&self, char: char) -> CellMaterial {
//         match char {
//             AIR => CellMaterial::Air,
//             SAND => CellMaterial::Sand,
//             SANDGENERATOR => CellMaterial::SandGenerator,
//             WATERGENERATOR => CellMaterial::WaterGenerator,
//             WATER => CellMaterial::Water,
//             _ => panic!("Tried to get material for unknown character '{char}'"),
//         }
//     }
//
//     fn get_char_from_material(&self, material: CellMaterial) -> char {
//         match material {
//             CellMaterial::Sand => SAND,
//             CellMaterial::SandGenerator => SANDGENERATOR,
//             CellMaterial::Air => AIR,
//             CellMaterial::Water => WATER,
//             CellMaterial::WaterGenerator => WATERGENERATOR,
//         }
//     }
//
//     pub fn draw(&mut self) {
//         // clear screen
//         print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//
//         for row in &self.area {
//             println!("{}", row.iter().collect::<String>())
//         }
//     }
// }
//
// impl SandSimulation for SandspielTerminal {
//     fn width(&self) -> i16 {
//         self.width
//     }
//
//     fn height(&self) -> i16 {
//         self.height
//     }
//
//     fn get_cell(&self, x: i16, y: i16) -> Option<Cell> {
//         if x < 0 || y < 0 {
//             return None;
//         }
//
//         if x >= self.width {
//             return None;
//         }
//
//         if y >= self.height {
//             return None;
//         }
//
//         let cell_representation = &self.area[y as usize][x as usize];
//         let material = self.get_material_from_char(cell_representation.clone());
//
//         let cell = Cell { x, y, material };
//
//         Some(cell)
//     }
//
//     fn set_cell(&mut self, cell: Cell) {
//         let cell_representation = self.get_char_from_material(cell.material);
//         self.area[cell.y as usize][cell.x as usize] = cell_representation;
//     }
//
//     fn switch_cells(&mut self, cell1: Cell, cell2: Cell) {
//         let cell1_char = self.get_char_from_material(cell1.material);
//         let cell2_char = self.get_char_from_material(cell2.material);
//
//         self.area[cell1.y as usize][cell1.x as usize] = cell2_char;
//         self.area[cell2.y as usize][cell2.x as usize] = cell1_char;
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn sand_falls_down() {
//         let a = AIR;
//         let s = SAND;
//
//         let area_s0 = vec![vec![a, s, a], vec![a, a, a], vec![a, a, a]];
//         let area_s1 = vec![vec![a, a, a], vec![a, s, a], vec![a, a, a]];
//         let area_s2 = vec![vec![a, a, a], vec![a, a, a], vec![a, s, a]];
//         let area_s3 = vec![vec![a, a, a], vec![a, a, a], vec![a, s, a]];
//         let mut sim = SandspielTerminal {
//             width: 3,
//             height: 3,
//             area: area_s0.clone(),
//         };
//
//         sim.update();
//         assert_eq!(sim.area, area_s1);
//         sim.update();
//         assert_eq!(sim.area, area_s2);
//         sim.update();
//         assert_eq!(sim.area, area_s3);
//     }
//
//     #[test]
//     fn sand_falls_down_left() {
//         let a = AIR;
//         let s = SAND;
//
//         let area_s0 = vec![vec![a, a, a], vec![a, s, a], vec![a, s, a]];
//         let area_s1 = vec![vec![a, a, a], vec![a, a, a], vec![s, s, a]];
//
//         let mut sim = SandspielTerminal {
//             width: 3,
//             height: 3,
//             area: area_s0.clone(),
//         };
//
//         sim.update();
//         assert_eq!(sim.area, area_s1);
//     }
// }
