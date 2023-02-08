use crate::sand_sim::CollisionDesire::{
    Consume, Convert, Eradicate, Evade, GetConverted, SwapAndMove, SwapAndStop,
};
use crate::sand_sim::ExtDirection::{One, Random};
use crate::universe::Direction::{Down, Left, LeftDown, LeftUp, Right, RightDown, RightUp, Up};
use crate::universe::Material::{
    Air, Fire, Sand, SandGenerator, Smoke, Vapor, Water, WaterGenerator, Wood,
};
use crate::universe::{CellContent, Direction, Material, Position, Universe};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::{random, thread_rng, Rng};
use std::ops::Deref;
use std::slice::Iter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

pub struct Simulation {
    pub universe: Arc<Universe>,
    tick_interval: u8,
    tick: u8,
}

impl Simulation {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            universe: Arc::new(Universe::new(width, height)),
            tick_interval: 2,
            tick: 0,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn tick(&mut self) {
        self.tick = (self.tick + 1) % self.tick_interval;
        if self.tick != 0 {
            return;
        }

        self.universe.set_all_unhandled();

        // for index in (0..self.universe.area.len()).rev() {
        //     let pos = self.universe.i_to_pos(index);
        //     // let cell = self.universe.get_cell(&pos).unwrap();
        //     // {
        //     // cell.lock().unwrap().velocity += 1;
        //     // }
        //     self.handle_collision(&pos);
        // }

        // lets do multithreading!
        // let universe1 = Arc::clone(self.universe);
        let universe1 = self.universe.clone();
        let universe2 = self.universe.clone();
        // let universe2 = Arc::clone(self.universe);
        let len = self.universe.area.len();
        let midpoint = self.universe.area.len() / 2;

        let handle1 = thread::spawn(move || {
            for index in (midpoint..len).rev() {
                eprintln!("thread 1 start");
                let pos = universe1.i_to_pos(index);
                handle_collision(&universe1, &pos);
                eprintln!("thread 1 end");
            }
        });

        let handle2 = thread::spawn(move || {
            for index in (0..midpoint).rev() {
                eprintln!("thread 2 start");
                let pos = universe2.i_to_pos(index);
                handle_collision(&universe2, &pos);
                eprintln!("thread 2 end");
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}

fn handle_collision(universe: &Universe, pos: &Position) {
    let cell = universe.get_cell(pos).unwrap();
    let steps;
    let mut cell_content = cell.lock().unwrap();
    if cell_content.handled {
        return;
    }
    cell_content.velocity += 1;
    steps = cell_content.velocity.abs();
    drop(cell_content);
    step(universe, pos, cell, steps);
}

fn step(universe: &Universe, pos: &Position, cell: &Arc<Mutex<CellContent>>, steps_remaining: i16) {
    let mut cell_content = cell.lock().unwrap();

    if steps_remaining == 0 {
        // we used all steps without stopping, i.e. free fall
        cell_content.handled = true;
        return;
    }

    let dirs = cell_content.material.directions();

    for dir in ExtDirIterator::new(&dirs) {
        if let Some((neighbor_pos, neighbor)) = universe.get_neighbor(pos, dir) {
            let mut neighbor_content = neighbor.lock().unwrap();
            match cell_content
                .material
                .collide(&neighbor_content.material, dir)
            {
                SwapAndMove => {
                    // self.universe.swap_cells(&mut current_cell, &mut neighbor);
                    let copy = cell_content.clone();

                    cell_content.clone_from(&neighbor_content);
                    neighbor_content.clone_from(&copy);
                    // cell_content.handled = false;

                    // self.handle_collision(pos, current_cell);
                    drop(neighbor_content);
                    drop(cell_content);
                    return step(universe, &neighbor_pos, &neighbor, steps_remaining - 1);
                }
                SwapAndStop => {
                    // self.universe.swap_cells(&mut current_cell, &mut neighbor);
                    cell_content.velocity = 0;
                    cell_content.handled = true;
                    let copy = cell_content.clone();

                    cell_content.clone_from(&neighbor_content);
                    neighbor_content.clone_from(&copy);
                    // cell_content.handled = false;

                    // self.handle_collision(current_cell);
                    // current_cell = neighbor;
                    return;
                }
                Convert(replace_material) => {
                    neighbor_content.clone_from(&CellContent::new(replace_material, true, 0));
                    return;
                }
                Evade => {}
                Consume(mat) => {
                    neighbor_content.clone_from(&CellContent::new(mat, false, 0));
                    let copy = cell_content.clone();

                    cell_content.clone_from(&neighbor_content);
                    neighbor_content.clone_from(&copy);

                    // self.handle_collision(current_cell);
                    // current_cell = neighbor;
                    return;
                }
                GetConverted(mat) => {
                    cell_content.clone_from(&CellContent::new(mat, true, 0));
                    return;
                }
                Eradicate(new_current_mat, new_neighbor_mat) => {
                    cell_content.clone_from(&CellContent::new(new_current_mat, true, 0));
                    neighbor_content.clone_from(&CellContent::new(new_neighbor_mat, true, 0));
                    return;
                }
            }
        }
    }
    // we checked all neighbors and couldnt move, so we save cell with velocity = 0
    // let content = cell.lock().unwrap();
    cell_content.velocity = 0;
    cell_content.handled = true;
    return;
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
    fn directions(&self) -> Vec<ExtDirection> {
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
