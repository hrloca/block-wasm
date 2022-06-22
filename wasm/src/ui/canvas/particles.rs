use super::*;
use crate::store::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::thread;

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

thread_local!(static PARTICLE_POOL: RefCell<Vec<Box<dyn Particle>>> = RefCell::new(vec![]));

pub fn particle_draw(p: Box<dyn Particle>) {
    PARTICLE_POOL.with(|f| {
        f.borrow_mut().push(p);
    })
}

pub fn particle_render(ctx: &CanvasRenderingContext2d, state: &State) {
    PARTICLE_POOL.with(|f| {
        f.borrow_mut().retain(|p| {
            if !p.is_drawed() {
                p.start(state);
            }
            p.draw(ctx, state);

            if !p.is_finish() {
                return true;
            }
            p.finish(state);
            false
        })
    });
}

pub struct ParticleDrawer;
impl ParticleDrawer {
    pub fn create() -> Self {
        ParticleDrawer
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d, state: &State) {
        PARTICLE_POOL.with(|f| {
            f.borrow_mut().retain_mut(|p| {
                if !p.is_drawed() {
                    p.start(state);
                }
                p.draw(ctx, state);

                if !p.is_finish() {
                    return true;
                }
                p.finish(state);
                false
            })
        });
    }

    pub fn draw(&self, p: Box<dyn Particle>) {
        PARTICLE_POOL.with(|f| {
            f.borrow_mut().push(p);
        })
    }
}
