use crate::universe::CellKind::*;
use crate::universe::Direction::*;
use crate::universe::{Cell, Position, Universe, Velocity};
use rand::random;

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
            self.handle_cell_at(self.universe.i_to_pos(index), false);
        }
    }

    fn handle_cell_at(&mut self, pos: Position, force: bool) {
        let cell = self.universe.get_cell_mut(&pos).unwrap();

        if cell.handled && !force {
            return;
        }

        cell.handled = true;

        let initial_velocity = cell.velocity.clone();

        let (final_pos, final_velocity) = match cell.kind {
            Air => self.handle_air(pos, initial_velocity),
            Sand => self.handle_sand(pos, initial_velocity),
            SandGenerator => self.handle_sand_generator(pos, initial_velocity),
            Water => self.handle_water(pos, initial_velocity),
            WaterGenerator => self.handle_water_generator(pos, initial_velocity),
        };

        self.universe.get_cell_mut(&final_pos).unwrap().velocity = final_velocity;
    }

    fn handle_sand(&mut self, pos: Position, velocity: Velocity) -> (Position, Velocity) {
        let mut current_pos = pos;
        let mut moves = velocity.abs() + 1;

        'outer_loop: while moves > 0 {
            moves -= 1;
            'inner_loop: for dir in [Down, RightDown, LeftDown] {
                if let Some((other_cell, other_pos)) =
                    self.universe.get_neighbor_mut(&current_pos, &dir)
                {
                    match other_cell.kind {
                        Air => {
                            self.universe.swap_cells(&current_pos, &other_pos);
                            current_pos = other_pos;
                            continue 'outer_loop;
                        }
                        Water => {
                            // if dir == Down {
                            self.universe.swap_cells(&current_pos, &other_pos);
                            // want the switched water cell to be handled as well, as it might have been put past the iteration cursor
                            self.handle_cell_at(current_pos, true);
                            current_pos = other_pos;
                            break 'inner_loop;
                        }
                        _ => {}
                    }
                }
            }
            return (current_pos, 0);
        }

        (current_pos, velocity + 1)
    }

    fn handle_water(&mut self, pos: Position, velocity: Velocity) -> (Position, Velocity) {
        let mut current_pos = pos;
        let mut moves = velocity.abs() + 1;
        'outer_loop: while moves > 0 {
            moves -= 1;
            for dir in [Down, RightDown, LeftDown, Right, Left] {
                if let Some((other_cell, other_pos)) =
                    self.universe.get_neighbor(&current_pos, &dir)
                {
                    match other_cell.kind {
                        Air => {
                            self.universe.swap_cells(&current_pos, &other_pos);
                            current_pos = other_pos;
                            continue 'outer_loop;
                        }
                        _ => {}
                    }
                }
            }
            return (current_pos, 0);
        }

        (current_pos, velocity + 1)
    }

    fn handle_sand_generator(&mut self, pos: Position, velocity: Velocity) -> (Position, Velocity) {
        for dir in [Down] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(&pos, &dir) {
                match other_cell.kind {
                    Air => {
                        if random() {
                            self.universe.set_cell(Cell::new(Sand, true), &other_pos);
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        (pos, velocity)
    }

    fn handle_water_generator(
        &mut self,
        pos: Position,
        velocity: Velocity,
    ) -> (Position, Velocity) {
        for dir in [Down] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(&pos, &dir) {
                match other_cell.kind {
                    Air => {
                        if random() {
                            self.universe.set_cell(Cell::new(Water, true), &other_pos);
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        (pos, velocity)
    }

    fn handle_air(&self, pos: Position, velocity: Velocity) -> (Position, Velocity) {
        // air doesn't move on its own
        (pos, velocity)
    }
}
