use super::super::*;
use super::*;
use crate::board::*;
use crate::log;
use js_sys::Date;
use std::f64::consts::PI;

type Finished = Box<dyn Fn(&mut ActionDispacher, Point, Point)>;

pub struct ChangeParticle {
    total: f64,
    created: f64,
    a: Point,
    b: Point,
    finished: Finished,
    colors: BlockColors,
}

impl ChangeParticle {
    pub fn new(a: Point, b: Point, finished: Finished) -> ChangeParticle {
        ChangeParticle {
            colors: BlockColors::create(),
            a,
            b,
            created: Date::new_0().get_time(),
            total: 200.0,
            finished,
        }
    }
    fn elapsed(&self) -> f64 {
        let now = Date::new_0().get_time();
        now - self.created
    }

    fn progress(&self) -> f64 {
        clamp(self.elapsed() / self.total, 1.0, 0.0)
    }

    fn ease(&self, x: f64) -> f64 {
        1.0 - ((x * PI) / 2.0).cos()
    }
}

fn clamp(num: f64, min: f64, max: f64) -> f64 {
    num.max(max).min(min)
}

impl Particle for ChangeParticle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State, _: &mut ActionDispacher) {
        let width = 80;
        let height = 80;
        let width_f64 = width as f64;
        let height_f64 = height as f64;
        let a = self.a;
        let b = self.b;

        let a_block = state.blocks.pick(a).unwrap();
        let a_color = self.colors.get(a_block.kind);

        let b_block = state.blocks.pick(b).unwrap();
        let b_color = self.colors.get(b_block.kind);

        let a_from = Point::of(a.x * width, a.y * height);
        let a_to = Point::of(b.x * width, b.y * height);

        let b_from = Point::of(b.x * width, b.y * height);
        let b_to = Point::of(a.x * width, a.y * height);

        let rate = self.ease(self.progress());

        let a_x = a_from.x as f64 + (a_to.x as f64 - a_from.x as f64) * rate;
        let a_y = a_from.y as f64 + (a_to.y as f64 - a_from.y as f64) * rate;

        let b_x = b_from.x as f64 + (b_to.x as f64 - b_from.x as f64) * rate;
        let b_y = b_from.y as f64 + (b_to.y as f64 - b_from.y as f64) * rate;

        ctx.begin_path();

        ctx.set_fill_style(&a_color.into());
        ctx.rect(a_x, a_y, width_f64, height_f64);
        ctx.fill_rect(a_x, a_y, width_f64, height_f64);

        ctx.set_fill_style(&b_color.into());
        ctx.rect(b_x, b_y, width_f64, height_f64);
        ctx.fill_rect(b_x, b_y, width_f64, height_f64);

        ctx.stroke();
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.a, self.b);
    }
}
