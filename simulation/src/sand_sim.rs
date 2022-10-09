use crate::universe::{Cell, CellKind, Direction, Position, Universe};

pub struct Simulation {
    pub universe: Universe,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            universe: Universe::new(width, height),
        }
    }

    pub fn tick(&mut self) {
        self.universe.set_all_unhandled();

        // for pos in (0..self.universe.area.len()).rev() {
        //     self.handle_cell_at(pos);
        // }

        for cell in self.universe.get_cells().iter() {
            self.handle_cell(cell);
        }
    }

    fn handle_cell(&mut self, cell: &Cell) {
        if *cell.handled() {
            return;
        }

        let cell = match cell.kind() {
            CellKind::Air => self.handle_air(cell),
            CellKind::Sand => self.handle_sand(cell),
            CellKind::SandGenerator => self.handle_sand_generator(cell),
            CellKind::Water => self.handle_water(cell),
            CellKind::WaterGenerator => self.handle_water_generator(cell),
        };

        debug_assert_eq!(*cell.handled(), true);
    }

    fn handle_sand(&mut self, cell: &Cell) -> Cell {
        for dir in [Direction::Down, Direction::RightDown, Direction::LeftDown] {
            if let Some(other_cell) = self.universe.get_neighbor(cell, dir) {
                match other_cell.kind() {
                    CellKind::Sand => {}
                    CellKind::SandGenerator => {}
                    CellKind::Air => {
                        let (mut cell, other_cell) = self.universe.swap_cells(*cell, other_cell);
                        cell.set_handled(true);
                        return cell;
                    }
                    CellKind::Water => {
                        let (mut cell, other_cell) = self.universe.swap_cells(*cell, other_cell);

                        self.handle_water(&mut other_cell);
                        cell.set_handled(true);
                        return cell;
                    }
                    CellKind::WaterGenerator => {}
                }
            }
        }
        cell.set_handled(true);
        return *cell;
    }

    fn handle_water(&mut self, cell: &Cell) -> Cell {
        for dir in [
            Direction::Down,
            Direction::RightDown,
            Direction::LeftDown,
            Direction::Right,
            Direction::Left,
        ] {
            if let Some(other_cell) = self.universe.get_neighbor(cell, dir) {
                match other_cell.kind() {
                    CellKind::Air => {
                        let (mut cell, other_cell) = self.universe.swap_cells(*cell, other_cell);
                        cell.set_handled(true);
                        return cell;
                    }
                    _ => {}
                }
            }
        }
        cell.set_handled(true);
        return *cell;
    }

    fn handle_sand_generator(&mut self, cell: &Cell) -> Cell {
        for dir in [Direction::Down] {
            if let Some(other_cell) = self.universe.get_neighbor(cell, dir) {
                match other_cell.kind() {
                    CellKind::Air => {
                        return self.universe.replace_cell(&other_cell, CellKind::Sand);
                    }
                    _ => {}
                }
            }
        }
        cell.set_handled(true);
        return *cell;
    }

    fn handle_water_generator(&mut self, cell: &Cell) -> Cell {
        for dir in [Direction::Down] {
            if let Some(other_cell) = self.universe.get_neighbor(cell, dir) {
                match other_cell.kind() {
                    CellKind::Air => {
                        return self.universe.replace_cell(&other_cell, CellKind::Water);
                    }
                    _ => {}
                }
            }
        }
        cell.set_handled(true);
        return *cell;
    }

    fn handle_air(&self, cell: &Cell) -> Cell {
        // air doesn't move on its own
        cell.set_handled(true);
        return *cell;
    }
}
