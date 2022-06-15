use super::super::*;
use super::*;
use crate::board::*;
use js_sys::Date;

type Finished = Box<dyn Fn(&mut ActionDispacher, Vec<Point>)>;

pub struct DeleteParticle {
    total: f64,
    created: f64,
    delete: Vec<Point>,
    finished: Finished,
    colors: Colors,
    off: bool,
}

impl DeleteParticle {
    pub fn create(delete: Vec<Point>, finished: Finished) -> Self {
        DeleteParticle {
            colors: Colors::create(),
            off: false,
            delete,
            created: Date::new_0().get_time(),
            total: 500.0,
            finished,
        }
    }

    fn elapsed(&self) -> f64 {
        let now = Date::new_0().get_time();
        now - self.created
    }

    fn delete_draw(&self, ctx: &CanvasRenderingContext2d, point: Point, color: &str) {
        let x = point.x as f64 * WIDTH;
        let y = point.y as f64 * HEIGHT;

        if !self.off {
            BlockShape::create((x, y), color).draw(ctx);
        }
    }
}

impl Particle for DeleteParticle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State, _: &mut ActionDispacher) {
        self.delete.iter().for_each(|p| {
            let block = state.blocks.pick(*p).as_ref().unwrap();
            let color = self.colors.get(block.kind);
            self.delete_draw(ctx, *p, color);
        });

        self.off = !self.off;
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.delete.clone());
    }
}
