use crate::entities::cell_content::CellContent;
use crate::entities::direction::ExtDirIterator;
use crate::entities::material::CollisionDesire::{
    Consume, Convert, Eradicate, Evade, GetConverted, SwapAndMove, SwapAndStop,
};
use crate::entities::material::Material;
use crate::universe::{Position, Universe};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::available_parallelism;
// use std::time::{Instant, SystemTime};

pub type CellContentWrapper = Mutex<CellContent>;

/// Simulates the behaviour of [Material] in a [Universe] per tick
pub struct Simulation {
    pub universe: Arc<Universe<CellContentWrapper>>,

    // tick & tick_interval are used to reduce the "framerate" of the simulation.
    // halved if tick_interval > 1, in thirds if >2, and so on.
    // TODO: move this into renderer implementation
    tick_interval: u8,
    tick: u8,
    // tick_time_avg: u128,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            universe: Arc::new(Universe::new(width, height)),
            tick_interval: 1,
            tick: 0,
            // tick_time_avg: 0,
        }
    }

    /// Advances the simulation by one step. Might skip calculating the next state if tick_interval > 1.
    ///
    /// # Panics
    /// Panics if one of its threads cannot be joined.
    pub fn tick(&mut self) {
        self.tick += 1;
        self.tick %= self.tick_interval;

        if self.tick != 0 {
            return;
        }

        // let start_time = Instant::now();

        self.set_all_unhandled();

        // lets do multithreading!
        let num_threads = match available_parallelism() {
            Ok(n) => n.get(),
            Err(err) => {
                println!("Failed to get available parallelism: {}", err);
                1
            }
        };

        let len = self.universe.area.len();
        let slice_size = len / num_threads;

        let mut handles = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            let start = slice_size * i;

            // we need to have the special case for the last iteration, as the final part for
            // universe might be bigger than than [slice_size]
            let end = if i == num_threads - 1 {
                len - 1
            } else {
                slice_size * (i + 1)
            };

            let universe = self.universe.clone();

            let handle = thread::spawn(move || {
                for index in (start..end).rev() {
                    let pos = universe.i_to_pos(index);
                    handle_collision(&universe, &pos);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // unthreaded collision handling. Use this for wasm renderer, and comment out the threaded part
        // for index in (0..len).rev() {
        //     let pos = self.universe.i_to_pos(index);
        //     handle_collision(&self.universe, &pos);
        // }

        // tick_time calculation, used to measure execution time for each tick
        // let tick_time = start_time.elapsed().as_nanos();
        // let mut avg = self.tick_time_avg;
        // avg -= avg / 120;
        // avg += tick_time / 120;
        // self.tick_time_avg = avg;
    }

    /// Fills (part of) the universe of the simulation with the given area-
    ///
    /// # Panics
    /// Might panic when a lock for an affected cell is already held by the current thread.
    pub fn fill(&self, area: &[Material]) {
        for (i, kind) in area.iter().enumerate() {
            self.universe.area[i]
                .lock()
                .unwrap()
                .clone_from(&CellContent::new(kind.clone(), false, 0));
        }
    }

    /// Sets all [CellContent] in the [Universe] to unhandled.
    ///
    /// # Panics
    /// Might panic when a lock for a cell is already held by the current thread.
    pub fn set_all_unhandled(&self) {
        for cell in &self.universe.area {
            cell.lock().unwrap().handled = false;
        }
    }
}

/// Handles collisions for a cell in a [Universe] at the given [Position].
fn handle_collision(universe: &Universe<CellContentWrapper>, pos: &Position) {
    let mut cell_content = universe.get_cell(pos).unwrap().lock().unwrap();

    if cell_content.handled {
        return;
    }

    cell_content.velocity += 1;
    let steps_remaining = cell_content.velocity.abs();

    step(universe, pos, cell_content, steps_remaining);
}

/// Calculates a step during collision handling of a cell in a [Universe].
///
/// A cell might want to collide multiple times, based on its velocity. This function recursively
/// calls itself until satisfied.
fn step(
    universe: &Universe<CellContentWrapper>,
    pos: &Position,
    mut cell_content: MutexGuard<CellContent>,
    steps_remaining: i16,
) {
    if steps_remaining == 0 {
        // we used all steps without stopping, i.e. free fall
        cell_content.handled = true;
        return;
    }

    let dirs = cell_content.material.directions();

    for dir in ExtDirIterator::new(&dirs) {
        if let Some((neighbor_pos, neighbor)) = universe.get_neighbor(pos, dir) {
            let Ok(mut neighbor_content) = neighbor.try_lock() else {
                println!("Failed to acquire lock for neighbor at {neighbor_pos:?}");
                // we simply skip this neighbor and check the next one
                continue;
            };

            match cell_content
                .material
                .collide(&neighbor_content.material, dir)
            {
                SwapAndMove => {
                    let copy = cell_content.clone();
                    cell_content.clone_from(&neighbor_content);
                    neighbor_content.clone_from(&copy);

                    drop(cell_content);
                    handle_collision(universe, pos);
                    return step(
                        universe,
                        &neighbor_pos,
                        neighbor_content,
                        steps_remaining - 1,
                    );
                }
                SwapAndStop => {
                    cell_content.velocity = 0;
                    cell_content.handled = true;

                    let copy = cell_content.clone();
                    cell_content.clone_from(&neighbor_content);
                    neighbor_content.clone_from(&copy);

                    return;
                }
                Convert(replace_material) => {
                    neighbor_content.clone_from(&CellContent::new(replace_material, true, 0));

                    break;
                }
                Evade => {}
                Consume(mat) => {
                    neighbor_content.clone_from(&CellContent::new(mat, false, 0));

                    let copy = cell_content.clone();
                    cell_content.clone_from(&neighbor_content);
                    neighbor_content.clone_from(&copy);

                    break;
                }
                GetConverted(mat) => {
                    cell_content.clone_from(&CellContent::new(mat, true, 0));

                    break;
                }
                Eradicate(new_current_mat, new_neighbor_mat) => {
                    cell_content.clone_from(&CellContent::new(new_current_mat, true, 0));
                    neighbor_content.clone_from(&CellContent::new(new_neighbor_mat, true, 0));

                    break;
                }
            }
        }
    }
    // we checked all neighbors and couldnt move, so we save cell with velocity = 0
    cell_content.velocity = 0;
    cell_content.handled = true;
}
