pub struct Sandspiel {
    width: u16,
    height: u16,
    pub area: Area,
}

pub type Area = Vec<Vec<Cell>>;

impl Sandspiel {
    pub fn new(width: u16, height: u16, area: Area) -> Self {
        Self {
            width,
            height,
            area,
        }
    }

    pub fn gen_area(width: u16, height: u16) -> Area {
        let default_cell = Cell::new(CellMaterial::Air, 0);
        vec![vec![default_cell; width as usize]; height as usize]
    }

    pub fn update(&mut self) {
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                self.handle_cell_at(Position { x, y })
            }
        }
    }

    pub fn handle_cell_at(&mut self, pos: Position) {
        let cell = self.get_cell(pos).unwrap();

        match cell.material {
            CellMaterial::Air => {}
            CellMaterial::Sand => self.handle_sand(cell, pos),
            CellMaterial::SandGenerator => self.handle_sand_generator(pos),
        }
    }

    fn get_cell(&self, pos: Position) -> Option<Cell> {
        let Position { x, y } = pos;

        match x >= self.width || y >= self.height {
            true => None,
            false => Some(self.area[y as usize][x as usize]),
        }
    }

    fn set_cell(&mut self, cell: Cell, pos: Position) {
        let Position { x, y } = pos;
        self.area[y as usize][x as usize] = cell;
    }
    fn handle_sand(&mut self, mut cell: Cell, pos: Position) {
        // adding velocity once per handling
        cell.velocity += 1;

        let mut moves = cell.velocity.abs();
        let mut cur_pos = pos;
        while moves > 0 {
            moves -= 1;

            match self.handle_sand_helper(cell, cur_pos) {
                Some(new_pos) => cur_pos = new_pos,
                None => {
                    // unable to move -> set velocity to 0
                    cell.velocity = 0;
                    break;
                }
            }
        }
    }

    fn handle_sand_helper(&mut self, cell: Cell, pos: Position) -> Option<Position> {
        for dir in [Direction::Down, Direction::RightDown, Direction::LeftDown] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell.material {
                    CellMaterial::Sand => {}
                    CellMaterial::SandGenerator => {}
                    CellMaterial::Air => {
                        self.switch_cells(cell, other_cell, pos, other_pos);
                        return Some(other_pos);
                    }
                }
            }
        }
        None
    }

    fn switch_cells(&mut self, cell: Cell, other_cell: Cell, pos: Position, other_pos: Position) {
        self.set_cell(cell, other_pos);
        self.set_cell(other_cell, pos);
    }

    fn get_neighbor(&self, pos: Position, dir: Direction) -> Option<(Cell, Position)> {
        if let Some(other_pos) = self.get_neighbor_pos(pos, dir) {
            Some((self.get_cell(other_pos).unwrap(), other_pos))
        } else {
            None
        }
    }

    fn get_neighbor_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Up => {
                if pos.y.checked_sub(1).is_none() {
                    None
                } else {
                    Some(Position::new(pos.x, pos.y - 1))
                }
            }
            Direction::Down => {
                if pos.y.checked_add(1).is_none() || pos.y + 1 >= self.height {
                    None
                } else {
                    Some(Position::new(pos.x, pos.y + 1))
                }
            }
            Direction::Left => {
                if pos.x.checked_sub(1).is_none() {
                    None
                } else {
                    Some(Position::new(pos.x - 1, pos.y))
                }
            }
            Direction::Right => {
                if pos.x.checked_add(1).is_none() || pos.x + 1 >= self.width {
                    None
                } else {
                    Some(Position::new(pos.x + 1, pos.y))
                }
            }
            Direction::LeftUp => {
                if pos.x.checked_sub(1).is_none() || pos.y.checked_sub(1).is_none() {
                    None
                } else {
                    Some(Position::new(pos.x - 1, pos.y - 1))
                }
            }
            Direction::RightUp => {
                if pos.x.checked_add(1).is_none()
                    || pos.y.checked_sub(1).is_none()
                    || pos.x + 1 >= self.width
                {
                    None
                } else {
                    Some(Position::new(pos.x + 1, pos.y - 1))
                }
            }
            Direction::LeftDown => {
                if pos.x.checked_sub(1).is_none()
                    || pos.y.checked_add(1).is_none()
                    || pos.y + 1 >= self.height
                {
                    None
                } else {
                    Some(Position::new(pos.x - 1, pos.y + 1))
                }
            }
            Direction::RightDown => {
                if pos.x.checked_add(1).is_none()
                    || pos.y.checked_add(1).is_none()
                    || pos.x + 1 >= self.width
                    || pos.y + 1 >= self.height
                {
                    None
                } else {
                    Some(Position::new(pos.x + 1, pos.y + 1))
                }
            }
        }
    }

    fn handle_sand_generator(&mut self, pos: Position) {
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell.material {
                    CellMaterial::Air => {
                        self.set_cell(Cell::new(CellMaterial::Sand, 0), other_pos);
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub material: CellMaterial,
    pub velocity: i8,
}

impl Cell {
    pub fn new(material: CellMaterial, velocity: i8) -> Self {
        Self { velocity, material }
    }
}

#[derive(Clone, Copy)]
pub enum CellMaterial {
    Sand,
    SandGenerator,
    // Water,
    // WaterGenerator,
    Air,
    // Solid,
}

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
