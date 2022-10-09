use crate::universe::{Cell, CellKind, Direction, Position, Universe};
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

        match cell.kind {
            CellKind::Air => self.handle_air(pos),
            CellKind::Sand => self.handle_sand(pos),
            CellKind::SandGenerator => self.handle_sand_generator(pos),
            CellKind::Water => self.handle_water(pos),
            CellKind::WaterGenerator => self.handle_water_generator(pos),
        };
    }

    fn handle_sand(&mut self, pos: Position) {
        for dir in [Direction::Down, Direction::RightDown, Direction::LeftDown] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor_mut(&pos, &dir) {
                match other_cell.kind {
                    CellKind::Air => {
                        self.universe.swap_cells(pos, other_pos);
                        return;
                    }
                    CellKind::Water => {
                        self.universe.swap_cells(pos, other_pos);
                        // want the switched water cell to be handled as well, as it might have been put past the iteration cursor
                        self.handle_cell_at(pos);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_water(&mut self, pos: Position) {
        for dir in [
            Direction::Down,
            Direction::RightDown,
            Direction::LeftDown,
            Direction::Right,
            Direction::Left,
        ] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell.kind {
                    CellKind::Air => {
                        self.universe.swap_cells(pos, other_pos);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_sand_generator(&mut self, pos: Position) {
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell.kind {
                    CellKind::Air => {
                        if random() {
                            self.universe
                                .set_cell(Cell::new(CellKind::Sand, true), other_pos);
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_water_generator(&mut self, pos: Position) {
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell.kind {
                    CellKind::Air => {
                        if random() {
                            self.universe
                                .set_cell(Cell::new(CellKind::Water, true), other_pos);
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_air(&self, pos: Position) {
        // air doesn't move on its own
    }
}
