use crate::sand_sim::CollisionDesire::{Evade, Replace, SwapAndMove, SwapAndStop};
use crate::sand_sim::ExtDirection::{Either, B};
use crate::universe::Direction::{Down, Left, LeftDown, LeftUp, Right, RightDown, RightUp, Up};
use crate::universe::Material::{
    Air, Fire, Sand, SandGenerator, Smoke, Vapor, Water, WaterGenerator, Wood,
};
use crate::universe::{Cell, CellContent, Direction, Material, Position, Universe};
use rand::{random, thread_rng, Rng};
use std::borrow::Borrow;
use std::collections::HashMap;

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

    pub fn tick(&mut self) {
        self.universe.set_all_unhandled();

        for index in (0..self.universe.area.len()).rev() {
            self.handle_cell_at(&self.universe.i_to_pos(index), false);
        }
    }

    fn handle_cell_at(&mut self, pos: &Position, force: bool) {
        let mut cell = self.universe.get_cell(pos).unwrap();

        if cell.content.handled && !force {
            return;
        }

        self.handle_collision(&mut cell);
    }

    fn handle_collision(&mut self, cell: &mut Cell) {
        let steps = cell.content.velocity.abs();
        let mut current_cell = cell;
        'stepping: for _step in 0..=steps {
            let material = &current_cell.content.material;
            'checking_directions: for dir in material.directions() {
                let dir = match dir {
                    B(d) => d,
                    Either(a, b) => {
                        if random() {
                            a
                        } else {
                            b
                        }
                    }
                };
                if let Some(mut neighbor) = self.universe.get_neighbor(current_cell, dir) {
                    match material.collide(&neighbor.content.material, dir) {
                        SwapAndMove => {
                            self.universe.swap_cells(current_cell, &mut neighbor);
                            self.handle_collision(current_cell);
                            current_cell.clone_from(&neighbor);
                            continue 'stepping;
                        }
                        SwapAndStop => {
                            self.universe.swap_cells(current_cell, &mut neighbor);
                            self.handle_collision(current_cell);
                            current_cell.clone_from(&neighbor);
                            break 'checking_directions;
                        }
                        Replace(replace_material) => {
                            neighbor.content = CellContent::new(replace_material, true, 0);
                            self.universe.save_cell(&neighbor);
                        }
                        Evade => {}
                    }
                }
            }
            // we checked all neighbors and couldnt move, so we save cell with velocity = 0
            current_cell.content.velocity = 0;
            current_cell.content.handled = true;
            self.universe.save_cell(current_cell);
            return;
        }
        // we used all steps without stopping, i.e. free fall, so we increase velocity for next tick
        current_cell.content.velocity += 1;
        current_cell.content.handled = true;
        self.universe.save_cell(current_cell);
    }
}

enum CollisionDesire {
    SwapAndMove,
    SwapAndStop,
    Evade,
    Replace(Material),
}

impl Material {
    fn directions(&self) -> &[ExtDirection] {
        match self {
            Sand => &[B(Down), Either(RightDown, LeftDown)],
            SandGenerator => &[B(Down)],
            Water => &[B(Down), Either(RightDown, LeftDown), Either(Right, Left)],
            WaterGenerator => &[B(Down)],
            Air => &[],
            Fire => &[
                B(Down),
                Either(RightDown, LeftDown),
                Either(Right, Left),
                B(Up),
                Either(RightUp, LeftUp),
            ],
            Smoke => &[B(Up), Either(RightUp, LeftUp), Either(Right, Left)],
            Vapor => &[B(Up), Either(RightUp, LeftUp), Either(Right, Left)],
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
            Water => SwapAndStop,
            Air => SwapAndMove,
            _ => Evade,
        }
    }

    fn collide_sand_generator(other: &Self) -> CollisionDesire {
        match other {
            Air => {
                if random() {
                    Replace(Sand)
                } else {
                    Evade
                }
            }
            _ => Evade,
        }
    }
    fn collide_water(other: &Self) -> CollisionDesire {
        match other {
            Air => SwapAndMove,
            Fire => {
                if random() {
                    Replace(Vapor)
                } else {
                    Evade
                }
            }
            _ => Evade,
        }
    }
    fn collide_water_generator(other: &Self) -> CollisionDesire {
        match other {
            Air => {
                if random() {
                    Replace(Water)
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
            Air => match dir {
                Down => SwapAndMove,
                _ => {
                    if random() {
                        Replace(Smoke)
                    } else {
                        Evade
                    }
                }
            },
            Water => {
                if random() {
                    Replace(Vapor)
                } else {
                    Evade
                }
            }
            Wood => {
                if random() {
                    Replace(Fire)
                } else {
                    Evade
                }
            }
            _ => Evade,
        }
    }
    fn collide_smoke(other: &Self) -> CollisionDesire {
        match other {
            Air => SwapAndStop,
            Vapor => SwapAndStop,
            _ => Evade,
        }
    }
    fn collide_vapor(other: &Self) -> CollisionDesire {
        // TODO: should have a way to cool down and become water again
        match other {
            Air => SwapAndStop,
            _ => Evade,
        }
    }
    fn collide_wood(_other: &Self) -> CollisionDesire {
        Evade
    }
}

pub enum ExtDirection {
    B(Direction),
    Either(Direction, Direction),
}
