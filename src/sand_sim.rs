use crate::universe::{Direction, Position, Universe};
use rand::{random, Rng};

pub struct Sandspiel {
    pub universe: Universe,
}

impl Sandspiel {
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
        if *self.universe.is_handled(pos).unwrap() {
            return;
        }

        let cell = self.universe.get_cell(pos).unwrap();

        let new_pos = match cell {
            Cell::Air => self.handle_air(pos),
            Cell::Sand => self.handle_sand(pos),
            Cell::SandGenerator => self.handle_sand_generator(pos),

            Cell::Water => self.handle_water(pos),
            Cell::WaterGenerator => self.handle_water_generator(pos),
        };

        self.universe.set_handled(new_pos);
    }

    fn handle_sand(&mut self, pos: Position) -> Position {
        for dir in [Direction::Down, Direction::RightDown, Direction::LeftDown] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell {
                    Cell::Sand => {}
                    Cell::SandGenerator => {}
                    Cell::Air => {
                        self.universe.swap_cells(pos, other_pos);
                        return other_pos;
                    }
                    Cell::Water => {
                        self.universe.swap_cells(pos, other_pos);
                        self.handle_water(pos);
                        return other_pos;
                    }
                    Cell::WaterGenerator => {}
                }
            }
        }
        return pos;
    }

    fn handle_water(&mut self, pos: Position) -> Position {
        for dir in [
            Direction::Down,
            Direction::RightDown,
            Direction::LeftDown,
            Direction::Right,
            Direction::Left,
        ] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell {
                    Cell::Sand => {}
                    Cell::SandGenerator => {}
                    Cell::Air => {
                        self.universe.swap_cells(pos, other_pos);
                        return other_pos;
                    }
                    Cell::Water => {}
                    Cell::WaterGenerator => {}
                }
            }
        }
        return pos;
    }

    fn handle_sand_generator(&mut self, pos: Position) -> Position {
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, dir) {
                match other_cell {
                    Cell::Air => {
                        self.universe.set_cell(Cell::Sand, other_pos);
                        break;
                    }
                    _ => {}
                }
            }
        }
        return pos;
    }

    fn handle_water_generator(&mut self, pos: Position) -> Position {
        if let Some((other_cell, other_pos)) = self.universe.get_neighbor(pos, Direction::Down) {
            match other_cell {
                Cell::Air => {
                    self.universe.set_cell(Cell::Water, other_pos);
                }
                _ => {}
            }
        }
        return pos;
    }

    fn handle_air(&self, pos: Position) -> Position {
        // air doesn't move on its own
        pos
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}
