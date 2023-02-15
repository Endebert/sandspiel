use crate::entities::direction::Direction;
use crate::entities::direction::Direction::{
    Down, Left, LeftDown, LeftUp, Right, RightDown, RightUp, Up,
};

#[derive(Debug)]
pub struct Universe<T> {
    pub area: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Default> Universe<T> {
    /// Initializes a new Universe and fills each cell with the default of [T].
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            area: Self::gen_area(width, height),
            width,
            height,
        }
    }

    /// Generates an area with a given width and height and fills it with the default value of [T].
    fn gen_area(width: usize, height: usize) -> Vec<T> {
        // not sure if this is the best approach to fill vec with the default value of [T],
        // as [T] might not be cloneable
        let mut vec = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            vec.push(T::default());
        }
        vec
    }

    /// Returns a cell at the given position, or [None] if position is outside of area.
    pub fn get_cell(&self, pos: &Position) -> Option<&T> {
        self.area.get(self.pos_to_i(pos))
    }

    /// Returns a valid neighbor and its position from a given position based on [Direction],
    /// or [None] if neighbor would be outside of area.
    pub(crate) fn get_neighbor(&self, pos: &Position, dir: &Direction) -> Option<(Position, &T)> {
        let neighbor_pos = self.get_neighbor_pos(pos, dir)?;
        let neighbor = self.get_cell(&neighbor_pos)?;

        Some((neighbor_pos, neighbor))
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

    /// Converts an index of of the internal area to [Position].
    pub(crate) fn i_to_pos(&self, index: usize) -> Position {
        let x = index % self.width;
        let y = (index - x) / self.width;

        Position { x, y }
    }

    /// Converts a [Position] to an index of the internal area.
    pub(crate) fn pos_to_i(&self, position: &Position) -> usize {
        let Position { x, y } = position;

        y * self.width + x
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
