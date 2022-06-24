use super::super::*;
use super::*;
use crate::board::*;
use crate::store::*;
use js_sys::Date;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod change_particle;
mod delete_particle;
mod fall_particle;
mod touch_particle;

pub use change_particle::*;
pub use delete_particle::*;
pub use fall_particle::*;
pub use touch_particle::*;

pub trait ParticleEntity {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State);
    fn is_complete(&self) -> bool;
    fn is_started(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct ParticleCore {
    total_time: f64,
    start_time: Option<f64>,
}

impl ParticleCore {
    pub fn create(total_time: f64) -> Self {
        ParticleCore {
            total_time,
            start_time: None,
        }
    }

    pub fn start_at(&mut self, time: f64) {
        self.start_time = Some(time);
    }

    pub fn elapsed(&self) -> f64 {
        match self.start_time {
            Some(time) => Date::new_0().get_time() - time,
            None => 0.,
        }
    }

    pub fn rate(&self) -> f64 {
        crate::libs::clamp(self.elapsed() / self.total_time, 1.0, 0.0)
    }

    pub fn rate_with(&self, ease: Ease) -> f64 {
        ease(self.rate())
    }

    pub fn is_exit(&self) -> bool {
        self.elapsed() > self.total_time
    }

    pub fn is_enter(&self) -> bool {
        self.start_time.is_some()
    }
}

pub enum Particles {
    Change(Point, Point),
    Touch(Point),
    Delete(Vec<Point>),
    Fall(Point, Point),
}

type ParticlePool = HashMap<u64, Box<dyn ParticleEntity>>;
type ParticleIdPool = Vec<u64>;

pub struct ParticleRender {
    pool: ParticlePool,
    task_pool: ParticleIdPool,
}

impl ParticleRender {
    pub fn create() -> Self {
        ParticleRender {
            pool: HashMap::new(),
            task_pool: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, p: Particles) -> u64 {
        let particle = matcher(p);
        let id = *get_task_id().borrow_mut();
        self.task_pool.push(id);
        self.pool.insert(id, particle);
        id
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State) {
        self.task_pool.iter_mut().for_each(|id| {
            let p = self
                .pool
                .get_mut(id)
                .expect("パーティクルが登録されていない");
            p.draw(ctx, state);
        });
    }

    fn drop(&mut self) -> Vec<u64> {
        self.task_pool
            .drain_filter(|id| {
                let p = self.pool.get(id).expect("パーティクルが登録されていない");
                p.is_complete()
            })
            .collect()
    }

    pub fn render(&mut self, context: &crate::Ctx) {
        self.draw(context.canvas_ctx, &context.state);
        for tasks in self.drop().iter() {
            context.action_dispacher.add_complete(*tasks);
        }
        // TODO: delete particle gabege.
    }
}

pub fn matcher(ps: Particles) -> Box<dyn ParticleEntity> {
    match ps {
        Particles::Change(a, b) => Box::new(NewChangeParticle::create(a, b)),
        Particles::Touch(target) => Box::new(NewTouchParticle::create(target)),
        Particles::Delete(dels) => Box::new(NewDeleteParticle::create(dels)),
        Particles::Fall(a, b) => Box::new(NewFallParticle::create(a, b)),
    }
}

thread_local!(static SCHEDULE_ID: Rc<RefCell<u64>> = Rc::new(RefCell::new(1)));

pub fn get_task_id() -> Rc<RefCell<u64>> {
    SCHEDULE_ID.with(|rc| *rc.borrow_mut() += 1);
    SCHEDULE_ID.with(|rc| rc.clone())
}

type DispatchPool = HashMap<u64, Box<dyn Fn(&crate::Ctx)>>;

pub struct TaskScheduler {
    pool: RefCell<DispatchPool>,
}

impl TaskScheduler {
    pub fn create() -> Self {
        TaskScheduler {
            pool: RefCell::new(HashMap::new()),
        }
    }

    pub fn then(&self, id: u64, f: Box<dyn Fn(&crate::Ctx)>) -> u64 {
        self.pool.borrow_mut().insert(id, f);
        self.issue()
    }

    fn issue(&self) -> u64 {
        *get_task_id().borrow_mut()
    }

    pub fn exec(&self, ids: &Vec<u64>, context: &crate::Ctx) {
        for id in ids.iter() {
            match self.pool.borrow().get(id) {
                Some(f) => {
                    f(context);
                    // context.action_dispacher.add_complete(*id);
                }
                None => (),
            }
            context.action_dispacher.delete_complete(*id);
        }
    }
}
