use crate::Cell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

pub struct Universe {
    area: Vec<Cell>,
    handled_area: Vec<bool>,
    width: usize,
    height: usize,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            area: Self::gen_area(width, height),
            handled_area: Self::gen_handled_area(width, height),
            width,
            height,
        }
    }

    pub fn positions(&self) -> Range<usize> {
        (0..self.area.len()).into_iter()
    }

    fn gen_area(width: usize, height: usize) -> Vec<Cell> {
        vec![Cell::Air; width * height]
    }

    fn gen_handled_area(width: usize, height: usize) -> Vec<bool> {
        vec![false; width * height]
    }

    pub fn fill(&mut self, area: Vec<Cell>) {
        for (i, cell) in area.iter().enumerate() {
            self.area[i] = *cell;
        }
    }

    pub fn get_cell(&self, pos: Position) -> Option<&Cell> {
        self.area.get(pos)
    }

    pub fn set_cell(&mut self, cell: Cell, pos: Position) {
        self.area[pos] = cell
    }

    pub fn swap_cells(&mut self, pos1: Position, pos2: Position) {
        self.area.swap(pos1, pos2);
    }

    pub fn is_handled(&self, pos: Position) -> Option<&bool> {
        self.handled_area.get(pos)
    }

    pub fn set_handled(&mut self, pos: Position) {
        self.handled_area[pos] = true;
    }

    pub fn get_neighbor(&self, pos: Position, dir: Direction) -> Option<(&Cell, Position)> {
        self.get_neighbor_pos(pos, dir)
            .map(|other_pos| (self.get_cell(other_pos).unwrap(), other_pos))
    }

    pub fn set_all_unhandled(&mut self) {
        self.handled_area.fill(false)
    }

    fn get_neighbor_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
        let width = self.width as isize;

        let offset: isize = match dir {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Up => -width,
            Direction::Down => width,
            Direction::LeftUp => -1 - width,
            Direction::RightUp => 1 - width,
            Direction::LeftDown => -1 + width,
            Direction::RightDown => 1 + width,
        };

        if offset.is_positive() {
            pos.checked_add(offset as usize)
                .filter(|x| *x <= self.area.len())
        } else {
            pos.checked_sub(offset as usize)
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn cell_to_char(cell: &Cell) -> char {
            match cell {
                Cell::Sand => '■',
                Cell::SandGenerator => 'S',
                Cell::Air => ' ',
                Cell::Water => '◉',
                Cell::WaterGenerator => 'W',
            }
        }

        // for row in self
        //     .area
        //     .chunks(self.width)
        //     .map(|row| row.iter().map(cell_to_char).collect::<String>())
        // {
        //     let res = writeln!(f, "{}", row);
        //     if res.is_err() {
        //         return res;
        //     }
        // }
        //
        // write!(f, "")

        let mut str = "".to_owned();
        for row in self
            .area
            .chunks(self.width)
            .map(|row| row.iter().map(cell_to_char).collect::<String>())
        {
            str += &*row;
            str += "\n";
        }

        write!(f, "{str}")
    }
}

pub type Position = usize;

#[derive(Clone, Copy, PartialEq)]
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
