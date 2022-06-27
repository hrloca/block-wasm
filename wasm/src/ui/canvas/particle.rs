use super::super::*;
use super::*;
use crate::board::*;
use crate::store::*;
use std::collections::HashMap;

mod change_particle;
mod delete_particle;
mod fall_particle;
mod particle_core;
mod touch_particle;

pub use change_particle::*;
pub use delete_particle::*;
pub use fall_particle::*;
pub use particle_core::*;
pub use touch_particle::*;

pub trait ParticleEntity {
    fn draw(&mut self, context: &crate::Ctx);
    fn is_complete(&self) -> bool;
    fn is_started(&self) -> bool;
    fn complete(&self, context: &crate::Ctx);
    fn started(&self, context: &crate::Ctx);
}

pub enum ParticleAction {
    Change(Point, Point),
    Touch(Point),
    Delete(Vec<Point>),
    Fall(Point, Point),
}

type Particle = Box<dyn ParticleEntity>;
type ParticleGropup = Vec<Particle>;

enum Particles {
    Simgle(Particle),
    Multi(ParticleGropup),
}

type ParticlePool = HashMap<u64, Box<dyn ParticleEntity>>;

type TaskIdPool = Vec<u64>;

pub struct ParticleRender {
    pool: ParticlePool,
    task_pool: TaskIdPool,
}

impl ParticleRender {
    pub fn create() -> Self {
        ParticleRender {
            pool: HashMap::new(),
            task_pool: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, p: ParticleAction) {
        let task_id = *issue_task_id().borrow_mut();
        self.dispatch_with(task_id, p);
    }

    pub fn dispatch_with(&mut self, task_id: u64, p: ParticleAction) {
        let particle = matcher(p);
        self.task_pool.push(task_id);
        self.pool.insert(task_id, particle);
    }

    fn draw(&mut self, context: &crate::Ctx) {
        self.task_pool.iter_mut().for_each(|task_id| {
            let p = self
                .pool
                .get_mut(task_id)
                .expect("パーティクルが登録されていない");

            if !p.is_started() {
                p.started(context);
            }

            p.draw(context);
        });
    }

    fn drop(&mut self, context: &crate::Ctx) -> Vec<u64> {
        self.task_pool
            .drain_filter(|id| {
                let p = self.pool.get(id).expect("パーティクルが登録されていない");
                if p.is_complete() {
                    p.complete(context);
                }
                p.is_complete()
            })
            .collect()
    }

    pub fn render(&mut self, context: &crate::Ctx) {
        self.draw(context);
        for tasks in self.drop(context).iter() {
            context.action_dispacher.add_complete(*tasks);
        }
        // TODO: delete particle gabege.
    }
}

fn matcher(ps: ParticleAction) -> Box<dyn ParticleEntity> {
    match ps {
        ParticleAction::Change(a, b) => Box::new(ChangeParticle::create(a, b)),
        ParticleAction::Touch(target) => Box::new(TouchParticle::create(target)),
        ParticleAction::Delete(dels) => Box::new(DeleteParticle::create(dels)),
        ParticleAction::Fall(a, b) => Box::new(FallParticle::create(a, b)),
    }
}
