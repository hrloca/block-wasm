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
            total: 300.0,
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
        if x < 0.5 {
            4.0 * x * x * x
        } else {
            1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
        }
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
        ctx.fill_rect(a_x, a_y, width_f64, height_f64);
        ctx.set_fill_style(&"rgb(0,0,0)".into());
        ctx.fill_text(
            &a_block.kind.to_string(),
            a_x + width_f64 / 2.0,
            a_y + height_f64 / 2.0,
        )
        .unwrap();

        ctx.set_fill_style(&b_color.into());
        ctx.fill_rect(b_x, b_y, width_f64, height_f64);
        ctx.set_fill_style(&"rgb(0,0,0)".into());
        ctx.fill_text(
            &b_block.kind.to_string(),
            b_x + width_f64 / 2.0,
            b_y + height_f64 / 2.0,
        )
        .unwrap();

        ctx.stroke();
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.a, self.b);
    }
}
