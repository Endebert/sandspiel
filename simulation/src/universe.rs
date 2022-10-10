use crate::universe::Direction::*;

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

    pub(crate) fn get_cell(&self, pos: &Position) -> Option<&Cell> {
        self.area.get(self.pos_to_i(pos))
    }

    pub(crate) fn get_cell_mut(&mut self, pos: &Position) -> Option<&mut Cell> {
        let index = self.pos_to_i(pos);
        self.area.get_mut(index)
    }

    pub(crate) fn set_cell(&mut self, cell: Cell, pos: &Position) {
        let index = self.pos_to_i(pos);
        self.area[index] = cell;
    }

    pub(crate) fn swap_cells(&mut self, pos1: &Position, pos2: &Position) {
        let index1 = self.pos_to_i(pos1);
        let index2 = self.pos_to_i(pos2);
        self.area.swap(index1, index2);
    }

    pub(crate) fn get_neighbor(
        &self,
        pos: &Position,
        dir: &Direction,
    ) -> Option<(&Cell, Position)> {
        let neighbor_pos = self.get_neighbor_pos(&pos, dir)?;
        let neighbor = self.get_cell(&neighbor_pos)?;

        Some((neighbor, neighbor_pos))
    }

    pub(crate) fn get_neighbor_mut(
        &mut self,
        pos: &Position,
        dir: &Direction,
    ) -> Option<(&mut Cell, Position)> {
        let neighbor_pos = self.get_neighbor_pos(&pos, &dir)?;
        let neighbor = self.get_cell_mut(&neighbor_pos)?;

        Some((neighbor, neighbor_pos))
    }

    pub fn set_all_unhandled(&mut self) {
        for mut cell in &mut self.area {
            cell.handled = false;
        }
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

    pub(crate) fn i_to_pos(&self, index: usize) -> Position {
        let x = index % self.width;
        let y = (index - x) / self.width;

        Position { x, y }
    }

    pub(crate) fn pos_to_i(&self, position: &Position) -> usize {
        let Position { x, y } = position;

        y * self.width + x
    }
}

#[derive(Clone)]
pub(crate) struct Position {
    pub x: usize,
    pub y: usize,
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

pub type Velocity = i8;

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
            velocity: 0,
            handled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_get_neighor_right_on_edge() {
        let universe = Universe::new(3, 3);
        assert!(universe
            .get_neighbor(&Position { x: 2, y: 1 }, &Right)
            .is_none());
    }
}
