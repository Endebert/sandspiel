pub struct Universe {
    pub area: Vec<Cell>,
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

    fn gen_area(width: usize, height: usize) -> Vec<Cell> {
        vec![Cell::new(CellKind::Air, false); width * height]
    }

    pub fn fill(&mut self, area: &[CellKind]) {
        for (i, kind) in area.iter().enumerate() {
            self.area[i] = Cell::new(kind.clone(), false);
        }
    }

    pub fn get_cell(&self, pos: Position) -> Option<&Cell> {
        self.area.get(pos)
    }

    pub fn get_cell_mut(&mut self, pos: Position) -> Option<&mut Cell> {
        self.area.get_mut(pos)
    }

    pub fn set_cell(&mut self, cell: Cell, pos: Position) {
        self.area[pos] = cell;
    }

    pub fn swap_cells(&mut self, pos1: Position, pos2: Position) {
        self.area.swap(pos1, pos2);
    }

    pub fn get_neighbor(&self, pos: Position, dir: Direction) -> Option<(&Cell, Position)> {
        self.get_neighbor_pos(&pos, &dir)
            .map(|other_pos| (self.get_cell(other_pos).unwrap(), other_pos))
    }

    pub fn get_neighbor_mut(
        &mut self,
        pos: &Position,
        dir: &Direction,
    ) -> Option<(&mut Cell, Position)> {
        self.get_neighbor_pos(&pos, &dir)
            .map(|other_pos| (self.get_cell_mut(other_pos).unwrap(), other_pos))
    }

    pub fn set_all_unhandled(&mut self) {
        for mut cell in &mut self.area {
            cell.handled = false;
        }
    }

    fn get_neighbor_pos(&self, pos: &Position, dir: &Direction) -> Option<Position> {
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

#[derive(Clone)]
pub struct Velocity {
    pub x: i8,
    pub y: i8,
}

impl Velocity {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

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

#[derive(Clone, PartialEq)]
pub enum CellKind {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}

#[derive(Clone)]
pub struct Cell {
    pub kind: CellKind,
    pub velocity: Velocity,
    pub handled: bool,
}

impl Cell {
    pub fn new(kind: CellKind, handled: bool) -> Self {
        Self {
            kind,
            velocity: Velocity::new(),
            handled,
        }
    }
}
