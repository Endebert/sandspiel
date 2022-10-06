use crate::Cell;
use std::fmt::{Display, Formatter};
use std::iter::Rev;
use std::ops::Range;

pub struct Universe {
    pub area: Vec<Cell>,
    handled_area: Vec<bool>,
    pub width: usize,
    pub height: usize,
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

    pub fn positions(&self) -> Rev<Range<usize>> {
        (0..self.area.len()).rev()
    }

    fn gen_area(width: usize, height: usize) -> Vec<Cell> {
        vec![Cell::Air; width * height]
    }

    fn gen_handled_area(width: usize, height: usize) -> Vec<bool> {
        vec![false; width * height]
    }

    pub fn fill(&mut self, area: &[Cell]) {
        for (i, cell) in area.iter().enumerate() {
            self.area[i] = *cell;
        }
    }

    pub fn get_cell(&self, pos: Position) -> Option<&Cell> {
        self.area.get(pos)
    }

    pub fn set_cell(&mut self, cell: Cell, pos: Position) {
        self.area[pos] = cell;
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
        self.handled_area.fill(false);
    }

    fn get_neighbor_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
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

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cell_to_char = |cell: &Cell| -> char {
            match cell {
                Cell::Sand => '■',
                Cell::SandGenerator => 'S',
                Cell::Air => ' ',
                Cell::Water => '◉',
                Cell::WaterGenerator => 'W',
            }
        };

        let mut lines: Vec<String> = Vec::with_capacity(self.height);
        for chunk in self.area.chunks(self.width) {
            lines.push(chunk.iter().map(cell_to_char).collect::<String>());
        }

        write!(f, "{}", lines.join("\n"))
    }
}

pub type Position = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
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
