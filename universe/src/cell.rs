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

pub type Velocity = i8;

#[derive(Clone, Debug, PartialEq)]
pub enum CellKind {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}

pub trait IsCell {
    fn position(&self) -> &Position;
    fn kind(&self) -> &CellKind;
    fn velocity(&self) -> &Velocity;
    fn handled(&self) -> &bool;
    fn set_kind(&mut self, kind: CellKind);
    fn set_velocity(&mut self, velocity: Velocity);
    fn set_handled(&mut self, handled: bool);
}
