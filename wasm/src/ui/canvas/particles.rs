use super::*;
use crate::store::*;
use web_sys::*;

mod change_particle;
mod delete_particle;
mod fall_particle;
mod touch_particle;

pub use change_particle::*;
pub use delete_particle::*;
pub use fall_particle::*;
pub use touch_particle::*;

pub trait Particle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State, action: &mut ActionDispacher);
    fn finish(&mut self, state: &State, action: &mut ActionDispacher);
    fn start(&mut self, state: &State, action: &mut ActionDispacher);
    fn is_finish(&self) -> bool;
    fn is_drawed(&self) -> bool;
    fn name(&self) -> String;
}

pub trait ParticleSet: Particle {}
