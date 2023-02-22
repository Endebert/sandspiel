use crate::entities::material::Material;
use crate::entities::material::Material::Air;

/// Velocity is the "speed" at which a [Material] moves through the [Universe].
/// It is used to simulate gravity.
pub type Velocity = i16;

/// The contents of a cell in a [Universe].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellContent {
    /// The material properties of the cell.
    pub material: Material,

    /// The velocity of the cell.
    pub velocity: Velocity,

    /// Whether or not a cell has been simulated during a simulation tick of a [Simulation].
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

impl Default for CellContent {
    fn default() -> Self {
        CellContent::new(Air, false, 0)
    }
}
