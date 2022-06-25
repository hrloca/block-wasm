use super::*;
use js_sys::Date;
use std::cell::RefCell;
use std::rc::Rc;

thread_local!(static PARTICLE_ID: Rc<RefCell<u64>> = Rc::new(RefCell::new(0)));

pub fn get_particle_id() -> Rc<RefCell<u64>> {
    PARTICLE_ID.with(|rc| *rc.borrow_mut() += 1);
    PARTICLE_ID.with(|rc| rc.clone())
}

pub struct ParticleCore {
    pub id: u64,
    total_time: f64,
    start_time: Option<f64>,
}

impl ParticleCore {
    pub fn create(total_time: f64) -> Self {
        ParticleCore {
            id: *get_particle_id().borrow_mut(),
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
