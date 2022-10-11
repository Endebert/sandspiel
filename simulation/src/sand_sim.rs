use rand::random;
use std::borrow::Borrow;
use std::collections::HashMap;
use universe::cell::CellKind::*;
use universe::cell::IsCell;
use universe::universe::Direction::*;
use universe::universe::{CellImpl, IsUniverse, Universe};

pub struct Simulation {
    pub universe: Universe,
}

impl Simulation {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            universe: Universe::new(width, height),
        }
    }

    pub fn tick(&mut self) {
        self.universe.set_all_unhandled();

        for index in (0..self.universe.area.len()).rev() {
            let cell = self
                .universe
                .get_cell(&self.universe.i_to_pos(index))
                .unwrap();
            self.handle_cell(cell, false);
        }
    }

    fn handle_cell(&mut self, cell: CellImpl, force: bool) {
        if cell.handled().clone() && !force {
            return;
        }

        cell.set_handled(true);

        match *cell.kind() {
            Air => self.handle_air(cell),
            Sand => self.handle_sand(cell),
            SandGenerator => self.handle_sand_generator(cell),
            Water => self.handle_water(cell),
            WaterGenerator => self.handle_water_generator(cell),
        };
    }

    fn handle_sand(&mut self, cell: CellImpl) {
        let mut moves = cell.velocity().abs();

        let mut current_cell = cell;

        'outer_loop: while moves > 0 {
            moves -= 1;
            'inner_loop: for dir in [Down, RightDown, LeftDown] {
                if let Some(mut other_cell) = self.universe.get_neighbor(&current_cell, &dir) {
                    match other_cell.kind() {
                        Water => {
                            self.universe.swap_cells(&current_cell, &other_cell);
                            current_cell = other_cell;
                            self.handle_cell(current_cell, true);
                            // upon collision with water we want to reset the velocity, so we break the inner loop
                            break 'inner_loop;
                        }
                        Air => {
                            self.universe.swap_cells(&current_cell, &other_cell);
                            current_cell = other_cell;
                            continue 'outer_loop;
                        }
                        _ => {}
                    }
                }
            }
            // we checked all neighbors and couldnt move, so we save cell with velocity = 0
            current_cell.set_velocity(0);
            return;
        }
        let final_velocity = current_cell.velocity() + 1;
        current_cell.set_velocity(final_velocity);
    }

    fn handle_water(&mut self, cell: CellImpl) {
        let mut moves = cell.velocity().abs();

        let mut current_cell = cell;

        'outer_loop: while moves > 0 {
            moves -= 1;
            for dir in [Down, RightDown, LeftDown, Right, Left] {
                if let Some(mut other_cell) = self.universe.get_neighbor(&current_cell, &dir) {
                    match other_cell.kind() {
                        Air => {
                            self.universe.swap_cells(&current_cell, &other_cell);
                            current_cell = other_cell;
                            continue 'outer_loop;
                        }
                        _ => {}
                    }
                }
            }
            // we checked all neighbors and couldnt move, so we save cell with velocity = 0
            current_cell.set_velocity(0);
            return;
        }
        let final_velocity = current_cell.velocity() + 1;
        current_cell.set_velocity(final_velocity);
    }

    // fn handle_sand(&mut self, cell: Cell) {
    //     // let mut current_pos = pos;
    //     let mut moves = cell.velocity().abs();
    //
    //     'outer_loop: while moves > 0 {
    //         moves -= 1;
    //         'inner_loop: for dir in [Down, RightDown, LeftDown] {
    //             if let Some((other_cell, other_pos)) =
    //                 self.universe.get_neighbor_mut(&current_pos, &dir)
    //             {
    //                 match other_cell.kind {
    //                     Air => {
    //                         self.universe.swap_cells(&current_pos, &other_pos);
    //                         current_pos = other_pos;
    //                         continue 'outer_loop;
    //                     }
    //                     Water => {
    //                         // if dir == Down {
    //                         self.universe.swap_cells(&current_pos, &other_pos);
    //                         // want the switched water cell to be handled as well, as it might have been put past the iteration cursor
    //                         self.handle_cell_at(current_pos, true);
    //                         current_pos = other_pos;
    //                         break 'inner_loop;
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         return (current_pos, 0);
    //     }
    //
    //     (current_pos, velocity + 1)
    // }
    //
    // fn handle_water(&mut self, cell: Cell) {
    //     let mut current_pos = pos;
    //     let mut moves = velocity.abs() + 1;
    //     'outer_loop: while moves > 0 {
    //         moves -= 1;
    //         for dir in [Down, RightDown, LeftDown, Right, Left] {
    //             if let Some((other_cell, other_pos)) =
    //                 self.universe.get_neighbor(&current_pos, &dir)
    //             {
    //                 match other_cell.kind {
    //                     Air => {
    //                         self.universe.swap_cells(&current_pos, &other_pos);
    //                         current_pos = other_pos;
    //                         continue 'outer_loop;
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         return (current_pos, 0);
    //     }
    //
    //     (current_pos, velocity + 1)
    // }
    //
    fn handle_sand_generator(&mut self, cell: CellImpl) {
        for dir in [Down] {
            if let Some(other_cell) = self.universe.get_neighbor(&cell, &dir) {
                match other_cell.kind() {
                    Air => {
                        if random() {
                            other_cell.set_kind(Sand);
                            other_cell.set_handled(true);
                            other_cell.set_velocity(0);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_water_generator(&mut self, cell: CellImpl) {
        for dir in [Down] {
            if let Some(other_cell) = self.universe.get_neighbor(&cell, &dir) {
                match other_cell.kind() {
                    Air => {
                        if random() {
                            other_cell.set_kind(Water);
                            other_cell.set_handled(true);
                            other_cell.set_velocity(0);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_air(&mut self, cell: CellImpl) {
        // air doesn't move on its own
        // cell
    }
}
