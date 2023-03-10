use crate::entities::cell_content::Particle;
use crate::entities::direction::ExtDirIterator;
use crate::entities::material::CollisionDesire::{
    Consume, Convert, Eradicate, Evade, GetConverted, SwapAndMove, SwapAndStop,
};
use crate::entities::material::Material;
use crate::universe::{Position, Universe};
use std::ops::Deref;

use rayon::current_num_threads;
use rayon::prelude::*;
use std::sync::{Mutex, MutexGuard};
// use std::time::{Instant, SystemTime};

pub type Cell = Mutex<Particle>;

/// Simulates the behaviour of [Material] in a [Universe] per tick
pub struct Simulation {
    pub universe: Universe<Cell>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            universe: Universe::new(width, height),
        }
    }

    /// Advances the simulation by one step.
    pub fn tick(&self) {
        self.set_all_unhandled();
        self.simulate();
    }

    /// Advances the simulation by one step. Uses multithreading where possible.
    pub fn par_tick(&self) -> usize {
        self.par_set_all_unhandled();
        self.par_simulate()
    }

    fn simulate(&self) {
        let failed_locks = Mutex::new(0usize);

        for index in (0..self.universe.area.len()).rev() {
            let pos = self.universe.i_to_pos(index);
            self.handle_collision(&pos, &failed_locks);
        }
    }

    fn par_simulate(&self) -> usize {
        let failed_locks = Mutex::new(0usize);

        let len = self.universe.area.len();
        let num_threads = current_num_threads();
        let slice_size = len / num_threads;

        (0..num_threads).into_par_iter().for_each(|i| {
            let start = slice_size * i;

            // we need to have the special case for the last iteration, as the final part for
            // universe might be bigger than than [slice_size]
            let end = if i == num_threads - 1 {
                len - 1
            } else {
                slice_size * (i + 1)
            };

            for index in (start..end).rev() {
                let pos = self.universe.i_to_pos(index);
                self.handle_collision(&pos, &failed_locks);
            }
        });

        let x = *failed_locks.lock().unwrap();
        x
    }

    /// Fills (part of) the universe of the simulation with the given area.
    pub fn fill(&self, area: &[Material]) {
        for (i, kind) in area.iter().enumerate() {
            *self.universe.area[i].lock().unwrap() = Particle::new(kind.clone(), false, 0);
        }
    }

    /// Fills (part of) the universe of the simulation with the given area. Uses multithreading where possible.
    pub fn par_fill(&self, area: &[Material]) {
        area.par_iter().enumerate().for_each(|(i, kind)| {
            *self.universe.area[i].lock().unwrap() = Particle::new(kind.clone(), false, 0);
        });
    }

    /// Sets all [Particle] in the [Universe] to unhandled.
    pub fn set_all_unhandled(&self) {
        for cell in &self.universe.area {
            cell.lock().unwrap().handled = false;
        }
    }

    /// Sets all [Particle] in the [Universe] to unhandled. Uses multithreading where possible.
    pub fn par_set_all_unhandled(&self) {
        self.universe
            .area
            .par_iter()
            .for_each(|cell| cell.lock().unwrap().handled = false);
    }

    /// Handles collisions for a cell in a [Universe] at the given [Position].
    fn handle_collision(&self, pos: &Position, failed_locks: &Mutex<usize>) {
        let mut cell_content = self.universe.get_cell(pos).unwrap().lock().unwrap();

        if cell_content.handled {
            return;
        }

        cell_content.velocity += 1;
        let steps_remaining = cell_content.velocity.abs();

        self.step(pos, cell_content, steps_remaining, failed_locks);
    }

    /// Calculates a step during collision handling of a cell in a [Universe].
    ///
    /// A cell might want to collide multiple times, based on its velocity. This function recursively
    /// calls itself until satisfied.
    fn step(
        &self,
        pos: &Position,
        mut cell_content: MutexGuard<Particle>,
        steps_remaining: i16,
        failed_locks: &Mutex<usize>,
    ) {
        if steps_remaining == 0 {
            // we used all steps without stopping, i.e. free fall
            cell_content.handled = true;
            return;
        }

        let dirs = cell_content.material.directions();

        for dir in ExtDirIterator::new(&dirs) {
            if let Some((neighbor_pos, neighbor)) = self.universe.get_neighbor(pos, dir) {
                // we cannot `neighbor.lock()` here as this might cause a deadlock.
                // therefore we just `try_lock()` and move on to the next neighbor if it fails
                let Ok(mut neighbor_content) = neighbor.try_lock() else {
                    // println!("Failed to acquire lock for neighbor at {neighbor_pos:?}");
                    *failed_locks.lock().unwrap() += 1;
                    continue;
                };

                // TODO: surely there must be a more elegant way for this instead of having multiple `breaks`, one `return` and one case of recursion
                match cell_content
                    .material
                    .collide(&neighbor_content.material, dir)
                {
                    SwapAndMove => {
                        let copy = cell_content.clone();
                        *cell_content = neighbor_content.clone();
                        *neighbor_content = copy;

                        drop(cell_content);
                        self.handle_collision(pos, failed_locks);
                        return self.step(
                            &neighbor_pos,
                            neighbor_content,
                            steps_remaining - 1,
                            failed_locks,
                        );
                    }
                    SwapAndStop => {
                        cell_content.velocity = 0;

                        let copy = cell_content.clone();
                        *cell_content = neighbor_content.clone();
                        *neighbor_content = copy;

                        drop(cell_content);
                        self.handle_collision(pos, failed_locks);
                        return self.step(&neighbor_pos, neighbor_content, 0, failed_locks);
                    }
                    Convert(replace_material) => {
                        *neighbor_content = Particle::new(replace_material, true, 0);

                        break;
                    }
                    Evade => {}
                    Consume(mat) => {
                        *neighbor_content = cell_content.clone();
                        *cell_content = Particle::new(mat, true, 0);

                        break;
                    }
                    GetConverted(mat) => {
                        *cell_content = Particle::new(mat, true, 0);

                        break;
                    }
                    Eradicate(new_current_mat, new_neighbor_mat) => {
                        *cell_content = Particle::new(new_current_mat, true, 0);
                        *neighbor_content = Particle::new(new_neighbor_mat, true, 0);

                        break;
                    }
                }
            }
        }
        // we checked all neighbors and couldnt move, so we save cell with velocity = 0
        cell_content.velocity = 0;
        cell_content.handled = true;
    }
}
