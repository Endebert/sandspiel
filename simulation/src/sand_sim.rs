use crate::sand_sim::CollisionDesire::*;
use crate::universe::CellKind::*;
use crate::universe::Direction::*;
use crate::universe::{Cell, CellInternal, CellKind, Direction, Position, Universe, Velocity};
use rand::{random, Rng, thread_rng};
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
        let cell = self.universe.get_cell(&pos).unwrap();

        if cell.handled() && !force {
            return;
        }

        let cell = self.universe.set_handled(cell);

        self.handle_collision(cell);
    }

    fn handle_collision(&mut self, cell: Cell) {
        let mut steps = cell.velocity().abs();
        let mut current_cell = cell;
        'stepping: for step in 0..steps {
            let kind = current_cell.kind();
            'checking_directions: for dir in kind.directions() {
                if let Some(mut other_cell) = self.universe.get_neighbor(&current_cell, &dir) {
                    match kind.collide(other_cell.kind(), dir) {
                        SwapAndMove => {
                            (other_cell, current_cell) =
                                self.universe.swap_cells(current_cell, other_cell);
                            continue 'stepping;
                        }
                        SwapAndStop => {
                            (other_cell, current_cell) =
                                self.universe.swap_cells(current_cell, other_cell);
                            self.handle_cell_at(other_cell.position(), true);
                            // upon collision with water we want to reset the velocity, so we break the inner loop
                            break 'checking_directions;
                        }
                        Replace(replace_kind) => {
                            if random() {
                                self.universe.set_cell(
                                    CellInternal::new(replace_kind, true, 0),
                                    other_cell.position(),
                                );
                            }
                        }
                        Evade => {}
                    }
                }
            }
            // we checked all neighbors and couldnt move, so we save cell with velocity = 0
            self.universe.set_velocity(current_cell, 0);
            return;
        }
        // we used all steps without stopping, so we increase velocity for next tick
        let final_velocity = current_cell.velocity() + 1;
        self.universe.set_velocity(current_cell, final_velocity);
    }
}

enum CollisionDesire {
    SwapAndMove,
    SwapAndStop,
    Evade,
    Replace(CellKind),
}

impl CellKind {
    fn directions(&self) -> &[Direction] {
        match self {
            Sand => &[Down, RightDown, LeftDown],
            SandGenerator => &[Down],
            Water => &[Down, RightDown, LeftDown, Right, Left],
            WaterGenerator => &[Down],
            Air => &[],
            Fire => &[Down, RightDown, LeftDown, Right, Left, Up, RightUp, LeftUp],
            Smoke => &[Up, RightUp, LeftUp, Right, Left],
            Vapor => &[Up, RightUp, LeftUp, Right, Left],
            Wood => &[],
        }
    }

    fn collide(&self, other: &CellKind, dir: &Direction) -> CollisionDesire {
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

    fn collide_sand(other: &CellKind) -> CollisionDesire {
        match other {
            Water => SwapAndStop,
            Air => SwapAndMove,
            _ => Evade,
        }
    }

    fn collide_sand_generator(other: &CellKind) -> CollisionDesire {
        match other {
            Air => Replace(Sand),
            _ => Evade,
        }
    }
    fn collide_water(other: &CellKind) -> CollisionDesire {
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
    fn collide_water_generator(other: &CellKind) -> CollisionDesire {
        match other {
            Air => Replace(Water),
            _ => Evade,
        }
    }
    fn collide_air(other: &CellKind) -> CollisionDesire {
        Evade
    }
    fn collide_fire(other: &CellKind, dir: &Direction) -> CollisionDesire {
        match other {
            Air => {
                match dir {
                    Down => SwapAndMove,
                    _ => {
                        if random() {
                            Replace(Smoke)
                        } else {
                            Evade
                        }
                    }
                }
            }
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
            _ => Evade
        }
    }
    fn collide_smoke(other: &CellKind) -> CollisionDesire {
        match other {
            Air => SwapAndStop,
            Vapor => SwapAndStop,
            _ => Evade
        }
    }
    fn collide_vapor(other: &CellKind) -> CollisionDesire {
        // TODO: should have a way to cool down and become water again
        match other {
            Air => SwapAndStop,
            _ => Evade
        }
    }
    fn collide_wood(other: &CellKind) -> CollisionDesire {
        Evade
    }
}
