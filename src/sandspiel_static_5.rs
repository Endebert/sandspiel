use crate::sand_sim::{CellMaterial, Direction, SandSimulation, TCell};

pub type Snapshot = [[CellMaterial; 5]; 5];

pub struct SandspielStatic5 {
    field: [[StoredCell; 5]; 5],
}

impl SandspielStatic5 {
    pub fn new(field: [[StoredCell; 5]; 5]) -> Self {
        Self { field }
    }

    pub fn snapshot(&self) -> Snapshot {
        self.field
            .map(|row| row.map(|StoredCell(material, _)| material))
    }

    pub fn from_snapshot(snapshot: Snapshot) -> Self {
        Self::new(snapshot.map(|row| row.map(|material| StoredCell(material, 0))))
    }

    pub fn run(&mut self) {
        for y in (0..5).rev() {
            for x in (0..5).rev() {
                let cell = self.get_cell(x, y);
                self.handle_cell(cell);
            }
        }
    }

    fn get_cell(&mut self, x: u16, y: u16) -> Cell2d {
        let stored = &mut self.field[y as usize][x as usize];
        let mut cell = Cell2d::new(x, y, stored);
        cell
    }
}

#[derive(Clone, Copy)]
struct StoredCell(CellMaterial, i8);

pub struct Cell2d<'a> {
    x: u16,
    y: u16,
    stored: &'a mut StoredCell,
}

impl<'a> Cell2d<'a> {
    pub fn new(x: u16, y: u16, stored: &mut StoredCell) -> Self {
        Self { x, y, stored }
    }
}

impl<'a> TCell for Cell2d<'a> {
    fn velocity(&mut self) -> &mut i8 {
        &mut self.stored.1
    }

    fn material(&self) -> &CellMaterial {
        &self.stored.0
    }
}

impl<'a> SandSimulation<Cell2d<'a>> for SandspielStatic5 {
    fn get_neighbor(&mut self, cell: &Cell2d, dir: Direction) -> Option<Cell2d<'a>> {
        let Cell2d { x, y, .. } = *cell;
        let (delta_x, delta_y): (i32, i32) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        };

        let x: u16 = match if delta_x.is_positive() {
            x.checked_add(delta_x as u16)
        } else {
            x.checked_sub(delta_x as u16)
        } {
            Some(v) => v,
            None => {
                return None;
            }
        };

        let y: u16 = match if delta_y.is_positive() {
            y.checked_add(delta_y as u16)
        } else {
            y.checked_sub(delta_y as u16)
        } {
            Some(v) => v,
            None => {
                return None;
            }
        };

        if x >= 5 || y >= 5 {
            return None;
        }

        let stored = &mut self.field[y as usize][x as usize];
        Some(Cell2d::new(x, y, stored))
    }

    fn swap_cells(&mut self, a: &mut Cell2d, b: &mut Cell2d) {
        let old_b_stored = b.stored;

        self.field[b.y as usize][b.x as usize] = *a.stored;
        self.field[a.y as usize][a.x as usize] = *old_b_stored;

        b.stored = a.stored;
        a.stored = old_b_stored;

        let old_b_x = b.x;
        let old_b_y = b.y;

        b.x = a.x;
        b.y = a.y;

        a.x = old_b_x;
        a.y = old_b_y;
    }

    fn get_random_bool(&self) -> bool {
        rand::random()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const S: CellMaterial = CellMaterial::Sand;
    const W: CellMaterial = CellMaterial::Water;
    const A: CellMaterial = CellMaterial::Air;

    #[test]
    fn sand_falls() {
        let snapshot_t0: Snapshot = [
            [A, A, S, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
        ];
        let snapshot_t1: Snapshot = [
            [A, A, A, A, A],
            [A, A, S, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
        ];
        let snapshot_t2: Snapshot = [
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, S, A, A],
            [A, A, A, A, A],
        ];
        let snapshot_t3: Snapshot = [
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, S, A, A],
        ];

        let mut spiel = SandspielStatic5::from_snapshot(snapshot_t0);
        assert_eq!(spiel.snapshot(), snapshot_t0);
        spiel.run();
        assert_eq!(spiel.snapshot(), snapshot_t1);
        spiel.run();
        assert_eq!(spiel.snapshot(), snapshot_t2);
        spiel.run();
        assert_eq!(spiel.snapshot(), snapshot_t3);
    }

    #[test]
    fn sand_blocks_sand() {
        let snapshot_t0: Snapshot = [
            [A, A, S, A, A],
            [A, A, A, A, A],
            [A, A, S, A, A],
            [A, A, A, A, A],
            [A, A, S, A, A],
        ];
        let snapshot_t1: Snapshot = [
            [A, A, A, A, A],
            [A, A, S, A, A],
            [A, A, A, A, A],
            [A, A, S, A, A],
            [A, A, S, A, A],
        ];
        let snapshot_t2: Snapshot = [
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, S, A, A],
            [A, A, S, S, A],
        ];
        let snapshot_t3: Snapshot = [
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, A, A, A, A],
            [A, S, S, S, A],
        ];
        let mut spiel = SandspielStatic5::from_snapshot(snapshot_t0);
        assert_eq!(spiel.snapshot(), snapshot_t0);
        spiel.run();
        assert_eq!(spiel.snapshot(), snapshot_t1);
        spiel.run();
        assert_eq!(spiel.snapshot(), snapshot_t2);
        spiel.run();
        assert_eq!(spiel.snapshot(), snapshot_t3);
    }
}
