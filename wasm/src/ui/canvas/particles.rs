use super::*;
use crate::log;
use crate::store::*;
use std::cell::RefCell;
use std::collections::HashMap;

mod change_particle;
mod delete_particle;
mod fall_particle;
mod touch_particle;

pub use change_particle::*;
pub use delete_particle::*;
pub use fall_particle::*;
pub use touch_particle::*;

pub trait Particle {
    fn is_finish(&self) -> bool;
    fn is_drawed(&self) -> bool;
    fn name(&self) -> String;
    fn draw(&self, ctx: &CanvasRenderingContext2d, state: &State);
    fn finish(&self, state: &State);
    fn start(&self, state: &State);
    fn id(&self) -> String;
}

pub struct ParticleDrawer {
    particles_list: RefCell<Vec<Box<dyn Particle>>>,
}

impl ParticleDrawer {
    pub fn create() -> Self {
        ParticleDrawer {
            particles_list: RefCell::new(vec![]),
        }
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d, state: &State) {
        self.particles_list.borrow_mut().retain_mut(|p| {
            if !p.is_drawed() {
                p.start(state);
            }
            p.draw(ctx, state);

            if !p.is_finish() {
                return true;
            }
            p.finish(state);
            false
        });
    }

    pub fn draw(&self, p: Box<dyn Particle>) -> String {
        let id = p.id();
        self.particles_list.borrow_mut().push(p);
        id
    }

    pub fn drop(&mut self) {}
}
