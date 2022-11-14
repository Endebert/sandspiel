use crate::sand_sim::CollisionDesire::{
    Consume, Convert, Eradicate, Evade, GetConverted, SwapAndMove, SwapAndStop,
};
use crate::sand_sim::ExtDirection::{One, Random};
use crate::universe::Direction::{Down, Left, LeftDown, LeftUp, Right, RightDown, RightUp, Up};
use crate::universe::Material::{
    Air, Fire, Sand, SandGenerator, Smoke, Vapor, Water, WaterGenerator, Wood,
};
use crate::universe::{Cell, CellContent, Direction, Material, Universe};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::{random, thread_rng, Rng};
use std::ops::Deref;
use std::slice::Iter;

pub struct Simulation {
    pub universe: Universe,
}

impl Simulation {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            universe: Universe::new(width, height),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn tick(&mut self) {
        self.universe.set_all_unhandled();

        for index in (0..self.universe.area.len()).rev() {
            let pos = self.universe.i_to_pos(index);
            let mut cell = self.universe.get_cell(&pos).unwrap();
            cell.content.velocity += 1;
            self.handle_collision(cell);
        }
    }

    fn handle_collision(&mut self, cell: Cell) {
        if cell.content.handled {
            return;
        }
        let steps = cell.content.velocity.abs();
        let mut current_cell = cell;
        'stepping: for _step in 0..steps {
            let material = &current_cell.content.material;
            'checking_directions: for dir in ExtDirIterator::new(material.directions()) {
                if let Some(mut neighbor) = self.universe.get_neighbor(&current_cell, dir) {
                    match material.collide(&neighbor.content.material, dir) {
                        SwapAndMove => {
                            self.universe.swap_cells(&mut current_cell, &mut neighbor);
                            current_cell.content.handled = false;
                            self.handle_collision(current_cell);
                            current_cell = neighbor;
                            continue 'stepping;
                        }
                        SwapAndStop => {
                            self.universe.swap_cells(&mut current_cell, &mut neighbor);
                            current_cell.content.handled = false;
                            self.handle_collision(current_cell);
                            current_cell = neighbor;
                            break 'checking_directions;
                        }
                        Convert(replace_material) => {
                            neighbor.content = CellContent::new(replace_material, true, 0);
                            self.universe.save_cell(&neighbor);
                            break 'checking_directions;
                        }
                        Evade => {}
                        Consume(mat) => {
                            neighbor.content = CellContent::new(mat, false, 0);
                            self.universe.swap_cells(&mut current_cell, &mut neighbor);
                            self.handle_collision(current_cell);
                            current_cell = neighbor;
                            break 'checking_directions;
                        }
                        GetConverted(mat) => {
                            current_cell.content = CellContent::new(mat, true, 0);
                            break 'checking_directions;
                        }
                        Eradicate(new_current_mat, new_neighbor_mat) => {
                            current_cell.content = CellContent::new(new_current_mat, true, 0);
                            neighbor.content = CellContent::new(new_neighbor_mat, true, 0);
                            self.universe.save_cell(&neighbor);
                            break 'checking_directions;
                        }
                    }
                }
            }
            // we checked all neighbors and couldnt move, so we save cell with velocity = 0
            current_cell.content.velocity = 0;
            current_cell.content.handled = true;
            self.universe.save_cell(&current_cell);
            return;
        }
        // we used all steps without stopping, i.e. free fall
        current_cell.content.handled = true;
        self.universe.save_cell(&current_cell);
    }
}

/// [A, B] -> [A, B] // Evade, e.g. [Sand, Wood]
/// [A, B] -> [B, A] // Swap, e.g. [Sand, Water]
///
/// [A, B] -> [A, C] // Convert, e.g. [Fire, Wood] -> [Fire, Fire]
/// [A, B] -> [C, A] // Consume, e.g. [Water, Vapor] -> [Water, Water]
///
/// [A, B] -> [C, B] // (be) Converted ?, e.g. [Water, Ice] -> [Ice, Ice]
/// [A, B] -> [B, C] // ?
///
/// [A, B] -> [C, D] // Eradicate ?, e.g. [Water, Fire], [Vapor, Smoke]
enum CollisionDesire {
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

fn rand_select<T>(a: T, b: T) -> T {
    if random() {
        a
    } else {
        b
    }
}

fn rand_select3<T>(a: T, b: T, c: T) -> T {
    match thread_rng().gen_range(0..3) {
        0 => a,
        1 => b,
        _ => c,
    }
}

#[allow(clippy::match_same_arms)]
impl Material {
    fn directions(&self) -> &[ExtDirection] {
        match self {
            Sand => &[One(Down), Random(RightDown, LeftDown)],
            SandGenerator => &[One(Down)],
            Water => &[One(Down), Random(RightDown, LeftDown), Random(Right, Left)],
            WaterGenerator => &[One(Down)],
            Air => &[],
            Fire => &[
                One(Down),
                Random(RightDown, LeftDown),
                Random(Right, Left),
                One(Up),
                Random(RightUp, LeftUp),
            ],
            Smoke => &[One(Up), Random(RightUp, LeftUp), Random(Right, Left)],
            Vapor => &[One(Up), Random(RightUp, LeftUp), Random(Right, Left)],
            Wood => &[],
        }
    }

    fn collide(&self, other: &Self, dir: &Direction) -> CollisionDesire {
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
            Water => rand_select(SwapAndStop, Evade),
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
            Air => rand_select(SwapAndMove, Evade),
            Vapor | Smoke => rand_select(SwapAndMove, Evade),
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
    fn collide_air(_other: &Self) -> CollisionDesire {
        Evade
    }
    fn collide_fire(other: &Self, dir: &Direction) -> CollisionDesire {
        match other {
            Air | Smoke | Vapor => match dir {
                Down | LeftDown | RightDown => rand_select(SwapAndStop, Evade),
                _ => Evade,
            },
            Water => rand_select(Consume(Vapor), Eradicate(Smoke, Vapor)),
            Wood => rand_select3(Consume(Smoke), Consume(Fire), Evade),
            _ => Evade,
        }
    }
    fn collide_smoke(other: &Self) -> CollisionDesire {
        match other {
            // Air => rand_select(SwapAndStop, GetConverted(Air)),
            Air => rand_select(SwapAndStop, Evade),
            Vapor => rand_select(SwapAndStop, Eradicate(Water, Air)),
            _ => Evade,
        }
    }
    fn collide_vapor(other: &Self) -> CollisionDesire {
        // TODO: should have a way to cool down and become water again
        match other {
            // Air => rand_select3(SwapAndStop, GetConverted(Air), GetConverted(Water)),
            Air => rand_select(SwapAndStop, Evade),
            Smoke => rand_select(SwapAndStop, Eradicate(Air, Water)),
            _ => Evade,
        }
    }
    fn collide_wood(_other: &Self) -> CollisionDesire {
        Evade
    }
}

pub enum ExtDirection {
    One(Direction),
    Random(Direction, Direction),
}

struct ExtDirIterator<'a> {
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
