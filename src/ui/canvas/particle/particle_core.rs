use super::*;
use js_sys::Date;

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
        crate::fns::clamp(self.elapsed() / self.total_time, 1.0, 0.0)
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
