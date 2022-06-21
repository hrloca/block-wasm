use super::*;
use crate::store::*;

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
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State);
    fn finish(&mut self, state: &State, action: &mut ActionDispacher);
    fn start(&mut self, state: &State, action: &mut ActionDispacher);
}

//

pub trait ParticleSet: Particle {}

pub struct ParticleDrawer {
    particles: Vec<Box<dyn Particle>>,
}

impl ParticleDrawer {
    pub fn create() -> Self {
        ParticleDrawer { particles: vec![] }
    }
    pub fn render(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        state: &State,
        action: &mut ActionDispacher,
    ) {
        self.particles.iter_mut().for_each(|p| {
            if !p.is_drawed() {
                p.start(state, action);
            }

            p.draw(ctx, state);

            if p.is_finish() {
                p.finish(state, action);
            }
        });
    }

    pub fn draw(&mut self, p: Box<dyn Particle>) {
        self.particles.push(p);
    }

    pub fn drop(&mut self) {
        self.particles.retain_mut(|p| !p.is_finish());
    }
}
