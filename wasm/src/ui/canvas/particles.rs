use crate::board::*;
use crate::log;
use crate::store::*;
use crate::tools::store::Store;
use js_sys::Date;

type BlockStore = Store<State, Actions>;

pub trait Particle {
    fn draw(&mut self, state: &State, action: &mut ActionDispacher);
    fn finish(&mut self, state: &State, action: &mut ActionDispacher);
    fn is_finish(&self) -> bool;
}

type Finished = Box<dyn Fn(&mut ActionDispacher, Point, Point)>;

pub struct ChangeParticle {
    total: f64,
    created: f64,
    from: Point,
    to: Point,
    finished: Finished,
}

impl ChangeParticle {
    pub fn new(from: Point, to: Point, finished: Finished) -> ChangeParticle {
        ChangeParticle {
            from,
            to,
            created: Date::new_0().get_time(),
            total: 200.0,
            finished,
        }
    }
    fn elapsed(&self) -> f64 {
        let now = Date::new_0().get_time();
        now - self.created
    }
}

impl Particle for ChangeParticle {
    fn draw(&mut self, state: &State, action: &mut ActionDispacher) {
        let from = self.from;
        let to = self.to;
        log!("{:?}: {:?}:", from, to);
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&mut self, state: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.from, self.to);
    }
}
