pub struct Universe {
    area: Vec<StoredCell>,
    pub width: usize,
    pub height: usize,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            area: Self::gen_area(width, height),
            width,
            height,
        }
    }

    fn gen_area(width: usize, height: usize) -> Vec<StoredCell> {
        vec![StoredCell::new(CellKind::Air); width * height]
    }

    pub fn fill(&mut self, area: &[CellKind]) {
        for (i, kind) in area.iter().enumerate() {
            self.area[i] = StoredCell::new(kind.clone());
        }
    }

    pub fn get_cells(&self) -> Vec<Cell> {
        self.area
            .iter()
            .enumerate()
            .map(|(i, sc)| Cell::new(sc, i))
            .rev()
            .collect()
    }

    fn get_stored_cell(&mut self, pos: Position) -> Option<&mut StoredCell> {
        self.area.get_mut(pos)
    }

    // fn set_cell(&mut self, cell: Cell, pos: Position) {
    //     self.area[pos] = cell;
    // }

    pub fn replace_cell(&mut self, cell_to_replace: &Cell, kind: CellKind) -> Cell {
        self.area[cell_to_replace.position] = StoredCell {
            kind,
            velocity: Velocity::new(),
            handled: true,
        };
        Cell::new(
            self.get_stored_cell(cell_to_replace.position).unwrap(),
            cell_to_replace.position,
        )
    }

    pub fn swap_cells(&mut self, cell1: Cell, cell2: Cell) -> (Cell, Cell) {
        self.area.swap(cell1.position, cell2.position);

        let new_cell1 = self.get_stored_cell(cell1.position).unwrap();
        let new_cell2 = self.get_stored_cell(cell2.position).unwrap();

        (
            Cell::new(new_cell1, cell1.position),
            Cell::new(new_cell2, cell2.position),
        )
    }

    pub fn get_neighbor(&self, cell: &Cell, dir: Direction) -> Option<Cell> {
        let neighbor_pos = self.get_neighbor_pos(&cell.position, dir)?;
        let stored_cell = self.get_stored_cell(neighbor_pos)?;

        Some(Cell::new(stored_cell, neighbor_pos))
    }

    pub fn set_all_unhandled(&mut self) {
        for stored_cell in &mut self.area {
            stored_cell.handled = false;
        }
    }

    fn get_neighbor_pos(&self, pos: &Position, dir: Direction) -> Option<Position> {
        let pos_x = pos % self.height;

        let offset_hor: isize = match dir {
            Direction::Left | Direction::LeftUp | Direction::LeftDown => -1,
            Direction::Right | Direction::RightUp | Direction::RightDown => 1,
            _ => 0,
        };

        let offset_ver: isize = match dir {
            Direction::Up | Direction::LeftUp | Direction::RightUp => -self.signed_width(),
            Direction::Down | Direction::LeftDown | Direction::RightDown => self.signed_width(),
            _ => 0,
        };

        let x: isize = isize::try_from(pos_x).unwrap() + offset_hor;

        if 0 < x && x < self.signed_width() {
            let offset = offset_hor + offset_ver;

            if offset.is_positive() {
                // add offset to pos and check if it is still in bounds of area
                pos.checked_add(offset.unsigned_abs())
                    .filter(|x| *x < self.area.len())
            } else {
                pos.checked_sub(offset.unsigned_abs())
            }
        } else {
            None
        }
    }

    fn signed_width(&self) -> isize {
        self.width.try_into().unwrap()
    }
}

pub type Position = usize;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

#[derive(Clone)]
pub struct Velocity {
    pub x: u8,
    pub y: u8,
}

impl Velocity {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone)]
struct StoredCell {
    kind: CellKind,
    velocity: Velocity,
    handled: bool,
}

impl StoredCell {
    pub fn new(kind: CellKind) -> Self {
        Self {
            kind,
            velocity: Velocity::new(),
            handled: false,
        }
    }

    pub fn set_kind(&mut self, kind: CellKind) {
        self.kind = kind;
    }
    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
    pub fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }
}

pub struct Cell<'a> {
    stored_cell: &'a StoredCell,
    position: Position,
}

impl<'a> Cell<'a> {
    fn new(stored_cell: &'a StoredCell, position: Position) -> Self {
        Self {
            stored_cell,
            position,
        }
    }

    pub fn kind(&self) -> &'a CellKind {
        &self.stored_cell.kind
    }

    pub fn velocity(&self) -> &'a Velocity {
        &self.stored_cell.velocity
    }

    pub fn handled(&self) -> &'a bool {
        &self.stored_cell.handled
    }

    pub fn set_kind(&mut self, kind: CellKind) {
        self.stored_cell.set_kind(kind)
    }
    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.stored_cell.set_velocity(velocity)
    }
    pub fn set_handled(&mut self, handled: bool) {
        self.stored_cell.set_handled(handled)
    }
}

#[derive(Clone)]
pub enum CellKind {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}
