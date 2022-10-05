use rand::{random, Rng};

pub struct Sandspiel {
    pub width: u16,
    pub height: u16,
    pub area: Vec<Cell>,
    handled_area: Vec<bool>,
}

impl Sandspiel {
    pub fn new(width: u16, height: u16, area: Vec<Cell>) -> Self {
        Self {
            width,
            height,
            area,
            handled_area: vec![false; (width * height) as usize],
        }
    }

    pub fn update(&mut self) {
        self.handled_area.fill(false);
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                self.handle_cell_at(Position { x, y });
            }
        }
    }

    pub fn gen_area(width: u16, height: u16) -> Vec<Cell> {
        vec![Cell::Air; (width * height) as usize]
    }

    fn handle_cell_at(&mut self, pos: Position) {
        if self.is_handled(pos) {
            return;
        }

        let mut cell = self.get_cell(pos).unwrap();

        let new_pos = match cell {
            Cell::Air => pos,
            Cell::Sand => self.handle_sand(pos),
            Cell::SandGenerator => self.handle_sand_generator(pos),

            Cell::Water => self.handle_water(pos),
            Cell::WaterGenerator => self.handle_water_generator(pos),
        };

        let area_index = self.get_area_index(&new_pos);
        self.handled_area[area_index] = true;
    }

    fn get_cell(&self, pos: Position) -> Option<Cell> {
        let Position { x, y } = pos;

        match x >= self.width || y >= self.height {
            true => None,
            false => Some(self.area[self.get_area_index(&pos)]),
        }
    }

    fn set_cell(&mut self, cell: Cell, pos: &Position) {
        let area_index = self.get_area_index(pos);
        self.area[area_index] = cell;
    }

    fn swap_cells(&mut self, pos1: &Position, pos2: &Position) {
        let area_index1 = self.get_area_index(pos1);
        let area_index2 = self.get_area_index(pos2);
        self.area.swap(area_index1, area_index2);
    }

    fn get_area_index(&self, pos: &Position) -> usize {
        (pos.y * self.width + pos.x) as usize
    }

    fn is_handled(&self, pos: Position) -> bool {
        self.handled_area[self.get_area_index(&pos)]
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

    fn handle_sand(&mut self, pos: Position) -> Position {
        let mut arr_down_lr = [Direction::LeftDown, Direction::RightDown];

        // if random() {
        //     arr_down_lr.reverse();
        // }
        for dir in [Direction::Down, arr_down_lr[0], arr_down_lr[1]] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell {
                    Cell::Sand => {}
                    Cell::SandGenerator => {}
                    Cell::Air => {
                        self.swap_cells(&pos, &other_pos);
                        return other_pos;
                    }
                    Cell::Water => {
                        self.swap_cells(&pos, &other_pos);
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
        let mut arr_down_lr = [Direction::LeftDown, Direction::RightDown];
        let mut arr_lr = [Direction::Left, Direction::Right];

        // if random() {
        //     arr_down_lr.reverse();
        //     arr_lr.reverse();
        // }

        for dir in [
            Direction::Down,
            arr_down_lr[0],
            arr_down_lr[1],
            arr_lr[0],
            arr_lr[1],
        ] {
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell {
                    Cell::Sand => {}
                    Cell::SandGenerator => {}
                    Cell::Air => {
                        self.swap_cells(&pos, &other_pos);
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
            if let Some((other_cell, other_pos)) = self.get_neighbor(pos, dir) {
                match other_cell {
                    Cell::Air => {
                        self.set_cell(Cell::Sand, &other_pos);
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
                match other_cell {
                    Cell::Air => {
                        self.set_cell(Cell::Water, &other_pos);
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
pub enum Cell {
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
