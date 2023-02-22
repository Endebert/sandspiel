use crate::entities::direction::Direction::{
    Down, Left, LeftDown, LeftUp, Right, RightDown, RightUp, Up,
};
use crate::entities::direction::ExtDirection::{One, Random};
use crate::entities::direction::{Direction, ExtDirection};
use crate::entities::material::CollisionDesire::{
    Consume, Convert, Eradicate, Evade, SwapAndMove, SwapAndStop,
};
use crate::entities::material::Material::{
    Air, Fire, Sand, SandGenerator, Smoke, Vapor, Water, WaterGenerator, Wood,
};
use crate::utils;
use rand::random;

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

#[allow(clippy::match_same_arms)]
impl Material {
    /// Returns the desired [Direction]s for neighbors to collide with.
    pub(crate) fn directions(&self) -> Vec<ExtDirection> {
        match self {
            Sand => vec![One(Down), Random(RightDown, LeftDown)],
            SandGenerator => vec![One(Down)],
            Water => vec![One(Down), Random(RightDown, LeftDown), Random(Right, Left)],
            WaterGenerator => vec![One(Down)],
            Air => vec![],
            Fire => vec![
                One(Down),
                Random(RightDown, LeftDown),
                Random(Right, Left),
                One(Up),
                Random(RightUp, LeftUp),
            ],
            Smoke => vec![One(Up), Random(RightUp, LeftUp), Random(Right, Left)],
            Vapor => vec![One(Up), Random(RightUp, LeftUp), Random(Right, Left)],
            Wood => vec![],
        }
    }

    /// Returns the [CollisionDesire] for a collision with another [Material].
    pub(crate) fn collide(&self, other: &Self, dir: &Direction) -> CollisionDesire {
        match self {
            Sand => Self::collide_sand(other),
            SandGenerator => Self::collide_sand_generator(other),
            Water => Self::collide_water(other),
            WaterGenerator => Self::collide_water_generator(other),
            Air => Self::collide_air(other),
            Fire => Self::collide_fire(other, dir),
            Smoke => Self::collide_smoke(other),
            Vapor => Self::collide_vapor(other),
            Wood => Self::collide_wood(other),
        }
    }

    fn collide_sand(other: &Self) -> CollisionDesire {
        match other {
            Water => utils::rand_select(SwapAndStop, Evade),
            Air => SwapAndMove,
            _ => Evade,
        }
    }

    fn collide_sand_generator(other: &Self) -> CollisionDesire {
        match other {
            Air => {
                if random() {
                    Convert(Sand)
                } else {
                    Evade
                }
            }
            _ => Evade,
        }
    }
    fn collide_water(other: &Self) -> CollisionDesire {
        match other {
            Air => utils::rand_select(SwapAndMove, Evade),
            Vapor | Smoke => utils::rand_select(SwapAndMove, Evade),
            Fire => Eradicate(Vapor, Smoke),
            _ => Evade,
        }
    }
    fn collide_water_generator(other: &Self) -> CollisionDesire {
        match other {
            Air => {
                if random() {
                    Convert(Water)
                } else {
                    Evade
                }
            }
            _ => Evade,
        }
    }

    // [Air] doesn't collide with anything on its, at least not until gusts of wind or similar
    // features are implemented
    fn collide_air(_other: &Self) -> CollisionDesire {
        Evade
    }

    fn collide_fire(other: &Self, dir: &Direction) -> CollisionDesire {
        match other {
            Air | Smoke | Vapor => match dir {
                Down | LeftDown | RightDown => utils::rand_select(SwapAndStop, Evade),
                _ => Evade,
            },
            Water => utils::rand_select(Consume(Vapor), Eradicate(Smoke, Vapor)),
            Wood => utils::rand_select3(Consume(Smoke), Consume(Fire), Evade),
            _ => Evade,
        }
    }
    fn collide_smoke(other: &Self) -> CollisionDesire {
        match other {
            // Air => rand_select(SwapAndStop, GetConverted(Air)),
            Air => utils::rand_select(SwapAndStop, Evade),
            Vapor => utils::rand_select(SwapAndStop, Eradicate(Water, Air)),
            _ => Evade,
        }
    }
    fn collide_vapor(other: &Self) -> CollisionDesire {
        // TODO: should have a way to cool down and become water again
        match other {
            // Air => rand_select3(SwapAndStop, GetConverted(Air), GetConverted(Water)),
            Air => utils::rand_select(SwapAndStop, Evade),
            Smoke => utils::rand_select(SwapAndStop, Eradicate(Air, Water)),
            _ => Evade,
        }
    }
    fn collide_wood(_other: &Self) -> CollisionDesire {
        Evade
    }
}

/// Types of desired outcomes for collisions with neighboring cells.
///
/// [A, B] -> [A, B] // Evade, e.g. [Sand, Wood], i.e. "don't do anything"
/// [A, B] -> [B, A] // Swap, e.g. [Sand, Water]
///
/// [A, B] -> [A, C] // Convert, e.g. [Fire, Wood] -> [Fire, Fire]
/// [A, B] -> [C, A] // Consume, e.g. [Vapor, Smoke] -> [Air, Vapor]
///
/// [A, B] -> [C, B] // (be) Converted ?, e.g. [Water, Ice] -> [Ice, Ice]
/// [A, B] -> [B, C] // ?
///
/// [A, B] -> [C, D] // Eradicate ?, e.g. [Water, Fire], [Vapor, Smoke]
pub enum CollisionDesire {
    /// [A, B] -> [A, B] // Evade, e.g. [Sand, Wood]
    Evade,
    /// [A, B] -> [B, A] // Swap, e.g. [Sand, Water]
    SwapAndMove,
    /// [A, B] -> [B, A] // Swap, e.g. [Sand, Water]
    SwapAndStop,

    /// [A, B] -> [A, C] // Convert, e.g. [Fire, Wood] -> [Fire, Fire]
    Convert(Material),
    /// [A, B] -> [C, A] // Consume, e.g. [Water, Vapor] -> [Water, Water]
    Consume(Material),

    /// [A, B] -> [C, B] // (be) Converted ?, e.g. [Water, Ice] -> [Ice, Ice]
    GetConverted(Material),

    /// [A, B] -> [C, D] // Eradicate ?, e.g. [Water, Fire], [Vapor, Smoke]
    Eradicate(Material, Material),
}
