use crate::entities::direction::ExtDirection::{One, Random};
use rand::random;
use std::slice::Iter;

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

/// An "extension" of the basic [Direction], indicating a specific [Direction] or a random order of multiple [Direction]s.
#[allow(clippy::module_name_repetitions)]
pub enum ExtDirection {
    One(Direction),
    Random(Direction, Direction),
}

/// A sloppily implemented [Iterator] for [ExtDirection], that applies the random selection for
/// [ExtDirection::Random] while preserving the overall order.
pub struct ExtDirIterator<'a> {
    dirs: Iter<'a, ExtDirection>,
    temp_remainder: Option<&'a Direction>,
}

impl<'a> ExtDirIterator<'a> {
    pub fn new(dirs: &'a [ExtDirection]) -> Self {
        Self {
            dirs: dirs.iter(),
            temp_remainder: None,
        }
    }
}

impl<'a> Iterator for ExtDirIterator<'a> {
    type Item = &'a Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remainder) = self.temp_remainder {
            self.temp_remainder = None;
            return Some(remainder);
        }

        let ext_dir = self.dirs.next()?;
        match ext_dir {
            One(d) => Some(d),
            Random(a, b) => {
                if random() {
                    self.temp_remainder = Some(b);
                    Some(a)
                } else {
                    self.temp_remainder = Some(a);
                    Some(b)
                }
            }
        }
    }
}
