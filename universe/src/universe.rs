use crate::cell::CellKind::*;
use crate::cell::{CellKind, IsCell, Position, Velocity};
use crate::universe::Direction::*;
use std::iter::{Enumerate, Map, Rev};
use std::slice::IterMut;

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
        let cell = CellInternal {
            kind: Air,
            velocity: 0,
            handled: false,
        };
        vec![cell; width * height]
    }

    fn get_neighbor_pos(&self, pos: &Position, dir: &Direction) -> Option<Position> {
        let Position { x, y } = *pos;
        let x = match dir {
            Left | LeftUp | LeftDown => x.checked_sub(1)?,
            Right | RightUp | RightDown => x.checked_add(1)?,
            _ => x,
        };

        let y = match dir {
            Up | LeftUp | RightUp => y.checked_sub(1)?,
            Down | LeftDown | RightDown => y.checked_add(1)?,
            _ => y,
        };

        if x < self.width && y < self.height {
            Some(Position { x, y })
        } else {
            None
        }
    }

    pub fn i_to_pos(&self, index: usize) -> Position {
        let x = index % self.width;
        let y = (index - x) / self.width;

        Position { x, y }
    }
    fn pos_to_i(&self, position: &Position) -> usize {
        let Position { x, y } = position;

        y * self.width + x
    }
}

impl<'a> IsUniverse<'a, CellImpl<'a>> for Universe {
    fn fill(&mut self, area: &[CellKind]) {
        for (i, kind) in area.iter().enumerate() {
            self.area[i] = CellInternal {
                kind: kind.clone(),
                velocity: 0,
                handled: false,
            };
        }
    }
    fn get_cell(&mut self, pos: &Position) -> Option<CellImpl> {
        let index = self.pos_to_i(pos);
        let internal = self.area.get_mut(index)?;

        Some(CellImpl {
            internal,
            pos: pos.clone(),
        })
    }
    // fn set_cell(&mut self, internal: CellInternal, pos: &Position) -> CellImpl {
    //     let index = self.pos_to_i(pos);
    //     self.area[index] = internal;
    //
    //     self.get_cell(pos).unwrap()
    // }
    fn swap_cells(&mut self, cell1: &CellImpl, cell2: &CellImpl) {
        let index1 = self.pos_to_i(cell1.position());
        let index2 = self.pos_to_i(cell2.position());

        self.area.swap(index1, index2);
    }
    fn get_neighbor(&mut self, cell: &CellImpl, dir: &Direction) -> Option<CellImpl> {
        let neighbor_pos = self.get_neighbor_pos(cell.position(), dir)?;
        let neighbor = self.get_cell(&neighbor_pos)?;

        Some(neighbor)
    }
    fn set_all_unhandled(&mut self) {
        for mut cell in &mut self.area {
            cell.handled = false;
        }
    }

    // fn iter(&'a mut self) -> IterMut<CellImpl> {
    //     self.area
    //         .iter_mut()
    //         .enumerate()
    //         .map(|(i, internal)| CellImpl {
    //             internal,
    //             pos: self.i_to_pos(i),
    //         })
    // }
}

pub trait IsUniverse<'a, Cell: IsCell> {
    fn fill(&'a mut self, area: &[CellKind]);
    fn get_cell(&'a mut self, pos: &Position) -> Option<Cell>;
    fn swap_cells(&'a mut self, cell1: &Cell, cell2: &Cell);
    fn get_neighbor(&'a mut self, cell: &Cell, dir: &Direction) -> Option<Cell>;
    fn set_all_unhandled(&'a mut self);
    // fn iter<T: Iterator<Item = Cell>>(&'a mut self) -> T;
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
pub struct CellInternal {
    kind: CellKind,
    velocity: Velocity,
    handled: bool,
}

pub struct CellImpl<'a> {
    internal: &'a mut CellInternal,
    pos: Position,
}

impl<'a> IsCell for CellImpl<'a> {
    fn position(&self) -> &Position {
        &self.pos
    }

    fn kind(&self) -> &CellKind {
        &self.internal.kind
    }

    fn velocity(&self) -> &Velocity {
        &self.internal.velocity
    }

    fn handled(&self) -> &bool {
        &self.internal.handled
    }

    fn set_kind(&mut self, kind: CellKind) {
        self.internal.kind = kind
    }

    fn set_velocity(&mut self, velocity: Velocity) {
        self.internal.velocity = velocity
    }

    fn set_handled(&mut self, handled: bool) {
        self.internal.handled = handled
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

    // #[test]
    // fn can_swap_cells() {
    //     let mut uni = Universe::new(2, 1);
    //     uni.fill(&[CellKind::Water, CellKind::Sand]);
    //
    //     let cell1 = uni.get_cell(&Position::new(0, 0)).unwrap();
    //     let cell2 = uni.get_cell(&Position::new(1, 0)).unwrap();
    //
    //     let o_cell1 = cell1.clone();
    //     let o_cell2 = cell2.clone();
    //
    //     assert_ne!(cell1, cell2);
    //
    //     println!("pre-swap:  {:?}", uni);
    //
    //     let (n_cell1, n_cell2) = uni.swap_cells(cell1, cell2);
    //
    //     println!("post-swap: {:?}", uni);
    //
    //     assert_ne!(n_cell1.kind(), n_cell2.kind());
    //     assert_eq!(o_cell1.kind(), n_cell2.kind());
    //     assert_eq!(o_cell2.kind(), n_cell1.kind());
    //
    //     assert_ne!(n_cell1.position(), n_cell2.position());
    //     assert_eq!(o_cell1.position(), n_cell1.position());
    //     assert_eq!(o_cell2.position(), n_cell2.position());
    // }
}
