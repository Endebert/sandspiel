use crate::sand_sim::{CellMaterial, Direction, SandSimulation, TCell};
pub type Snapshot = [[CellMaterial; 5]; 5];

pub struct SandspielStatic5 {
    field: [[(CellMaterial, i8); 5]; 5],
}

impl SandspielStatic5 {
    pub fn new(field: [[(CellMaterial, i8); 5]; 5]) -> Self {
        Self { field }
    }

    pub fn snapshot(&self) -> Snapshot {
        self.field.map(|row| row.map(|(material, _)| material))
    }

    pub fn from_snapshot(snapshot: Snapshot) -> Self {
        Self::new(snapshot.map(|row| row.map(|material| (material, 0))))
    }

    pub fn run(&mut self) {
        for y in (0..5).rev() {
            for x in (0..5).rev() {
                let (material, velocity) = self.field[y][x];
                let cell = Cell2d::new(x as u16, y as u16, velocity, material);
                self.handle_cell(cell);
            }
        }
    }
}

pub struct Cell2d {
    x: u16,
    y: u16,
    velocity: i8,
    material: CellMaterial,
}

impl Cell2d {
    pub fn new(x: u16, y: u16, velocity: i8, material: CellMaterial) -> Self {
        Self {
            x,
            y,
            velocity,
            material,
        }
    }
}

impl TCell for Cell2d {
    fn velocity(&self) -> &i8 {
        &self.velocity
    }

    fn material(&self) -> &CellMaterial {
        &self.material
    }
}

impl SandSimulation<Cell2d> for SandspielStatic5 {
    fn get_neighbor(&self, cell: &Cell2d, dir: Direction) -> Option<Cell2d> {
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

        let (material, velocity) = self.field[y as usize][x as usize];
        Some(Cell2d::new(x, y, velocity, material))
    }

    fn swap_cells(&mut self, mut a: &mut Cell2d, mut b: &mut Cell2d) {
        // let (mut a, mut b) = cells;
        self.field[b.y as usize][b.x as usize] = (a.material, a.velocity);
        self.field[a.y as usize][a.x as usize] = (b.material, b.velocity);

        let t = (a.x, a.y);

        a.x = b.x;
        a.y = b.y;

        b.x = t.0;
        b.y = t.1;
    }

    fn mod_velocity(&mut self, velocity: i8, cell: &mut Cell2d) {
        match cell.velocity.checked_add(velocity) {
            None => {}
            Some(velocity) => {
                self.field[cell.y as usize][cell.x as usize] = (cell.material, velocity);
                cell.velocity = velocity;
            }
        }
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
}
