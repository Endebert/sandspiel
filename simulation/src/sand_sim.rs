use crate::universe::{Cell, CellKind, Direction, Position, Universe, Velocity};
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

        for pos in (0..self.universe.area.len()).rev() {
            self.handle_cell_at(pos);
        }
    }

    fn handle_cell_at(&mut self, pos: Position) {
        let cell = self.universe.get_cell_mut(pos).unwrap();

        if cell.handled {
            return;
        }

        cell.handled = true;

        let initial_velocity = cell.velocity.clone();

        let (final_pos, final_velocity) = match cell.kind {
            CellKind::Air => self.handle_air(pos, initial_velocity),
            CellKind::Sand => self.handle_sand(pos, initial_velocity),
            CellKind::SandGenerator => self.handle_sand_generator(pos, initial_velocity),
            CellKind::Water => self.handle_water(pos, initial_velocity),
            CellKind::WaterGenerator => self.handle_water_generator(pos, initial_velocity),
        };

        self.universe.get_cell_mut(final_pos).unwrap().velocity = final_velocity;
    }

    fn handle_sand(&mut self, pos: Position, velocity: Velocity) -> (Position, Velocity) {
        let mut current_pos = pos.clone();
        'outer_for: for _ in 0..velocity.x.abs() {
            for dir in [Direction::Down, Direction::RightDown, Direction::LeftDown] {
                if let Some((other_cell, other_pos)) =
                    self.universe.get_neighbor_mut(&current_pos, &dir)
                {
                    match other_cell.kind {
                        CellKind::Air => {
                            self.universe.swap_cells(current_pos, other_pos);
                            current_pos = other_pos;
                            continue 'outer_for;
                        }
                        CellKind::Water => {
                            self.universe.swap_cells(current_pos, other_pos);
                            // want the switched water cell to be handled as well, as it might have been put past the iteration cursor
                            self.handle_cell_at(current_pos);
                            current_pos = other_pos;
                            continue 'outer_for;
                        }
                        _ => {}
                    }
                }
            }
            return (current_pos, Velocity::new());
        }
        return (
            current_pos,
            Velocity {
                x: velocity.x + 1,
                y: velocity.y,
            },
        );
    }

    fn handle_water(&mut self, pos: Position, velocity: Velocity) -> (Position, Velocity) {
        let mut current_pos = pos.clone();
        'outer_loop: for _ in 0..velocity.x.abs() {
            for dir in [
                Direction::Down,
                Direction::RightDown,
                Direction::LeftDown,
                Direction::Right,
                Direction::Left,
            ] {
                if let Some((other_cell, other_pos)) = self.universe.get_neighbor(current_pos, dir)
                {
                    match other_cell.kind {
                        CellKind::Air => {
                            self.universe.swap_cells(current_pos, other_pos);
                            current_pos = other_pos;
                            continue 'outer_loop;
                        }
                        _ => {}
                    }
                }
            }
            return (current_pos, Velocity::new());
        }
        return (
            current_pos,
            Velocity {
                x: velocity.x + 1,
                y: velocity.y,
            },
        );
    }

    fn handle_sand_generator(&mut self, pos: Position, velocity: Velocity) -> (Position, Velocity) {
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell.kind {
                    CellKind::Air => {
                        if random() {
                            self.universe
                                .set_cell(Cell::new(CellKind::Sand, true), other_pos);
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
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell.kind {
                    CellKind::Air => {
                        if random() {
                            self.universe
                                .set_cell(Cell::new(CellKind::Water, true), other_pos);
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
