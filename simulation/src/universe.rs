use crate::universe::Direction::{Down, Left, LeftDown, LeftUp, Right, RightDown, RightUp, Up};
use std::sync::{Arc, Mutex};

pub type CellContentWrapper = Mutex<CellContent>;

#[derive(Debug)]
pub struct Universe {
    pub area: Vec<CellContentWrapper>,
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

    fn gen_area(width: usize, height: usize) -> Vec<CellContentWrapper> {
        let mut vec = Vec::with_capacity(width * height);
        let cell_content = CellContent::new(Material::Air, false, 0);

        for _ in 0..width * height {
            vec.push(Mutex::new(cell_content.clone()));
        }
        vec
        // vec![CellContent::new(Material::Air, false, 0); width * height]
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn fill(&self, area: &[Material]) {
        for (i, kind) in area.iter().enumerate() {
            // self.area[i] = CellContent::new(kind.clone(), false, 0);
            self.area[i]
                .lock()
                .unwrap()
                .clone_from(&CellContent::new(kind.clone(), false, 0));
        }
    }

    pub fn get_cell(&self, pos: &Position) -> Option<&CellContentWrapper> {
        self.area.get(self.pos_to_i(pos))
    }

    pub(crate) fn get_neighbor(
        &self,
        pos: &Position,
        dir: &Direction,
    ) -> Option<(Position, &CellContentWrapper)> {
        let neighbor_pos = self.get_neighbor_pos(pos, dir)?;
        let neighbor = self.get_cell(&neighbor_pos)?;

        Some((neighbor_pos, neighbor))
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn set_all_unhandled(&self) {
        for cell in &self.area {
            cell.lock().unwrap().handled = false;
        }
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

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.area.len()).rev().map(|i| self.i_to_pos(i))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Material {
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

pub type Velocity = i16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellContent {
    pub material: Material,
    pub velocity: Velocity,
    pub handled: bool,
}

impl CellContent {
    pub fn new(mat: Material, handled: bool, velocity: Velocity) -> Self {
        Self {
            material: mat,
            velocity,
            handled,
        }
    }
}
