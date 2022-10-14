use crate::universe::Direction::*;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Universe {
    pub area: Vec<CellInternal>,
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

    fn gen_area(width: usize, height: usize) -> Vec<CellInternal> {
        vec![CellInternal::new(CellKind::Air, false, 0); width * height]
    }

    pub fn fill(&mut self, area: &[CellKind]) {
        for (i, kind) in area.iter().enumerate() {
            self.area[i] = CellInternal::new(kind.clone(), false, 0);
        }
    }

    pub(crate) fn get_cell(&self, pos: &Position) -> Option<Cell> {
        let internal = self.area.get(self.pos_to_i(pos))?;

        Some(Cell::new(internal.clone(), pos.clone()))
    }

    pub(crate) fn set_cell(&mut self, internal: CellInternal, pos: &Position) -> Cell {
        let index = self.pos_to_i(pos);
        self.area[index] = internal;

        self.get_cell(pos).unwrap()
    }

    pub(crate) fn swap_cells(&mut self, cell1: Cell, cell2: Cell) -> (Cell, Cell) {
        let index1 = self.pos_to_i(cell1.position());
        let index2 = self.pos_to_i(cell2.position());

        self.area.swap(index1, index2);

        let new_cell1 = self.get_cell(cell1.position()).unwrap();
        let new_cell2 = self.get_cell(cell2.position()).unwrap();

        (new_cell1, new_cell2)
    }

    pub(crate) fn get_neighbor(&self, cell: &Cell, dir: &Direction) -> Option<Cell> {
        let neighbor_pos = self.get_neighbor_pos(cell.position(), dir)?;
        let neighbor = self.get_cell(&neighbor_pos)?;

        Some(neighbor)
    }

    pub fn set_all_unhandled(&mut self) {
        for mut cell in &mut self.area {
            cell.handled = false;
        }
    }

    pub fn set_handled(&mut self, cell: Cell) -> Cell {
        let pos = cell.position();
        let mut internal = cell.internal.clone();
        internal.handled = true;

        self.set_cell(internal, pos)
    }

    pub fn set_velocity(&mut self, cell: Cell, velocity: Velocity) -> Cell {
        let pos = cell.position();
        let kind = cell.internal.kind().clone();

        self.set_cell(CellInternal::new(kind, true, velocity), pos)
    }

    fn get_neighbor_pos(&self, pos: &Position, dir: &Direction) -> Option<Position> {
        let Position { x, y } = *pos;
        let x = match dir {
            Left | LeftUp | LeftDown => x.checked_sub(1)?,
            Right | RightUp | RightDown => x.checked_add(1)?,
            Up | Down => x,
        };

        let y = match dir {
            Up | LeftUp | RightUp => y.checked_sub(1)?,
            Down | LeftDown | RightDown => y.checked_add(1)?,
            Left | Right => y,
        };

        if x < self.width && y < self.height {
            Some(Position { x, y })
        } else {
            None
        }
    }

    pub(crate) fn i_to_pos(&self, index: usize) -> Position {
        let x = index % self.width;
        let y = (index - x) / self.width;

        Position { x, y }
    }

    pub(crate) fn pos_to_i(&self, position: &Position) -> usize {
        let Position { x, y } = position;

        y * self.width + x
    }

    pub fn positions(&self) -> impl Iterator<Item=Position> + '_ {
        (0..self.area.len()).rev().map(|i| self.i_to_pos(i))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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

#[derive(Clone, Debug, PartialEq)]
pub enum CellKind {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    Fire,
    Smoke,
    Vapor,
    Wood,
}

pub type Velocity = i8;

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    position: Position,
    internal: CellInternal,
}

impl Cell {
    fn new(internal: CellInternal, position: Position) -> Self {
        Self { position, internal }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn kind(&self) -> &CellKind {
        &self.internal.kind
    }
    pub fn velocity(&self) -> &Velocity {
        &self.internal.velocity
    }
    pub fn handled(&self) -> bool {
        self.internal.handled
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellInternal {
    kind: CellKind,
    velocity: Velocity,
    handled: bool,
}

impl CellInternal {
    pub fn new(kind: CellKind, handled: bool, velocity: Velocity) -> Self {
        Self {
            kind,
            velocity,
            handled,
        }
    }
    pub fn kind(&self) -> &CellKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn cannot_get_neighor_right_on_edge() {
    //     let universe = Universe::new(3, 3);
    //     assert!(universe
    //         .get_neighbor(&Position { x: 2, y: 1 }, &Right)
    //         .is_none());
    // }

    #[test]
    fn can_swap_cells() {
        let mut uni = Universe::new(2, 1);
        uni.fill(&[CellKind::Water, CellKind::Sand]);

        let cell1 = uni.get_cell(&Position::new(0, 0)).unwrap();
        let cell2 = uni.get_cell(&Position::new(1, 0)).unwrap();

        let o_cell1 = cell1.clone();
        let o_cell2 = cell2.clone();

        assert_ne!(cell1, cell2);

        println!("pre-swap:  {:?}", uni);

        let (n_cell1, n_cell2) = uni.swap_cells(cell1, cell2);

        println!("post-swap: {:?}", uni);

        assert_ne!(n_cell1.kind(), n_cell2.kind());
        assert_eq!(o_cell1.kind(), n_cell2.kind());
        assert_eq!(o_cell2.kind(), n_cell1.kind());

        assert_ne!(n_cell1.position(), n_cell2.position());
        assert_eq!(o_cell1.position(), n_cell1.position());
        assert_eq!(o_cell2.position(), n_cell2.position());
    }
}
