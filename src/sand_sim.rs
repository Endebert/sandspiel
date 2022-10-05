use rand::{random, Rng};

pub struct Sandspiel {
    width: u16,
    height: u16,
    pub area: Area<Cell>,
    pub handled_area: Vec<bool>,
}

pub type Area<T> = Vec<Vec<T>>;

fn or<T>(a: T, b: T) -> T {
    if random() {
        a
    } else {
        b
    }
}

impl Sandspiel {
    pub fn new(width: u16, height: u16, area: Area<Cell>) -> Self {
        Self {
            width,
            height,
            area,
            handled_area: vec![false; (width * height) as usize],
        }
    }

    pub fn gen_area(width: u16, height: u16) -> Area<Cell> {
        let default_cell = Cell::new(CellMaterial::Air);
        vec![vec![default_cell; width as usize]; height as usize]
    }

    pub fn gen_handled_area(width: u16, height: u16) -> Area<bool> {
        vec![vec![false; width as usize]; height as usize]
    }

    pub fn update(&mut self) {
        self.handled_area.fill(false);
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                self.handle_cell_at(Position { x, y });
            }
        }
    }

    pub fn handle_cell_at(&mut self, pos: Position) {
        if self.is_handled(pos.x, pos.y) {
            return;
        }

        let mut cell = self.get_cell(pos).unwrap();

        let new_pos = match cell.material {
            CellMaterial::Air => pos,
            CellMaterial::Sand => self.handle_sand(cell, pos),
            CellMaterial::SandGenerator => self.handle_sand_generator(pos),

            CellMaterial::Water => self.handle_water(cell, pos),
            CellMaterial::WaterGenerator => self.handle_water_generator(pos),
        };

        self.handled_area[(new_pos.y * self.width + new_pos.x) as usize] = true;
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
    fn handle_sand(&mut self, mut cell: Cell, pos: Position) -> Position {
        let mut arr_down_lr = [Direction::LeftDown, Direction::RightDown];

        if random() {
            arr_down_lr.reverse();
        }
        for dir in [Direction::Down, arr_down_lr[0], arr_down_lr[1]] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell.material {
                    CellMaterial::Sand => {}
                    CellMaterial::SandGenerator => {}
                    CellMaterial::Air => {
                        self.switch_cells(cell, other_cell, pos, other_pos);
                        return other_pos;
                    }
                    CellMaterial::Water => {
                        self.switch_cells(cell, other_cell, pos, other_pos);
                        self.handle_water(Cell::new(CellMaterial::Water), pos);
                        return other_pos;
                    }
                    CellMaterial::WaterGenerator => {}
                }
            }
        }
        return pos;
    }

    fn handle_water(&mut self, mut cell: Cell, pos: Position) -> Position {
        let mut arr_down_lr = [Direction::LeftDown, Direction::RightDown];
        let mut arr_lr = [Direction::Left, Direction::Right];

        if random() {
            arr_down_lr.reverse();
            arr_lr.reverse();
        }

        for dir in [
            Direction::Down,
            arr_down_lr[0],
            arr_down_lr[1],
            arr_lr[0],
            arr_lr[1],
        ] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell.material {
                    CellMaterial::Sand => {}
                    CellMaterial::SandGenerator => {}
                    CellMaterial::Air => {
                        self.switch_cells(cell, other_cell, pos, other_pos);
                        return other_pos;
                    }
                    CellMaterial::Water => {}
                    CellMaterial::WaterGenerator => {}
                }
            }
        }
        return pos;
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

    fn is_handled(&self, x: u16, y: u16) -> bool {
        self.handled_area[(y * self.width + x) as usize]
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

    fn handle_sand_generator(&mut self, pos: Position) -> Position {
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell.material {
                    CellMaterial::Air => {
                        self.set_cell(Cell::new(CellMaterial::Sand), other_pos);
                        break;
                    }
                    _ => {}
                }
            }
        }
        return pos;
    }
    fn handle_water_generator(&mut self, pos: Position) -> Position {
        for dir in [Direction::Down] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell.material {
                    CellMaterial::Air => {
                        self.set_cell(Cell::new(CellMaterial::Water), other_pos);
                        break;
                    }
                    _ => {}
                }
            }
        }
        return pos;
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
}

impl Cell {
    pub fn new(material: CellMaterial) -> Self {
        Self { material }
    }
}

#[derive(Clone, Copy)]
pub enum CellMaterial {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}

#[derive(Clone, Copy)]
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
