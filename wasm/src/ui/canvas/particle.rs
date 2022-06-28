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
    fn is_completed(&self) -> bool;
    fn complete(&self, context: &crate::Ctx);
    fn started(&self, context: &crate::Ctx);
}

pub enum ParticleAction {
    Change(Point, Point),
    Touch(Point),
    Delete(Vec<Point>),
    Fall(Vec<(Point, Point)>),
}

type Particle = Box<dyn ParticleEntity>;
type ParticleGropup = Vec<Particle>;

enum Particles {
    Simgle(Particle),
    Multi(ParticleGropup),
}

type ParticlePool = HashMap<u64, Particles>;

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

            match p {
                Particles::Multi(x) => {
                    for x in x.iter_mut() {
                        if !x.is_started() {
                            x.started(context);
                        }
                        x.draw(context);
                        if x.is_complete() {
                            x.complete(context);
                        }
                    }
                }
                Particles::Simgle(x) => {
                    if !x.is_started() {
                        x.started(context);
                    }

                    x.draw(context);

                    if x.is_complete() {
                        x.complete(context);
                    }
                }
            }
        });
    }

    fn drop(&mut self, _: &crate::Ctx) -> Vec<u64> {
        self.task_pool
            .drain_filter(|id| {
                let p = self.pool.get(id).expect("パーティクルが登録されていない");

                match p {
                    Particles::Multi(x) => x.iter().all(|x| x.is_complete()),
                    Particles::Simgle(x) => x.is_complete(),
                }
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

fn matcher(ps: ParticleAction) -> Particles {
    match ps {
        ParticleAction::Change(a, b) => Particles::Simgle(Box::new(ChangeParticle::create(a, b))),
        ParticleAction::Touch(target) => Particles::Simgle(Box::new(TouchParticle::create(target))),
        ParticleAction::Delete(dels) => Particles::Simgle(Box::new(DeleteParticle::create(dels))),
        ParticleAction::Fall(falls) => {
            let pg: ParticleGropup = vec![];
            let pg = falls.iter().fold(pg, |mut acc, (from, to)| {
                acc.push(Box::new(FallParticle::create(*from, *to)));
                acc
            });
            Particles::Multi(pg)
        }
    }
}
