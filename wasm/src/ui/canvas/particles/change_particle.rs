use super::super::super::Easing;
use super::super::*;
use super::*;
use crate::board::*;
use js_sys::Date;

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
    pub fn create(a: Point, b: Point, finished: Finished) -> ChangeParticle {
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
        self.clamp(self.elapsed() / self.total, 1.0, 0.0)
    }

    fn rate(&self) -> f64 {
        Easing::ease_in_out_cubic(self.progress())
    }

    fn clamp(&self, num: f64, min: f64, max: f64) -> f64 {
        num.max(max).min(min)
    }

    fn draw_block(
        &self,
        ctx: &CanvasRenderingContext2d,
        from: Point,
        to: Point,
        kind: u8,
        color: &str,
    ) {
        let width = 80;
        let height = 80;
        let width_f64 = width as f64;
        let height_f64 = height as f64;
        let from = Point::of(from.x * width, from.y * height);
        let to = Point::of(to.x * width, to.y * height);

        let rate = self.rate();

        let x = from.x as f64 + (to.x as f64 - from.x as f64) * rate;
        let y = from.y as f64 + (to.y as f64 - from.y as f64) * rate;

        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x, y, width_f64, height_f64);
        ctx.set_fill_style(&"rgb(0,0,0)".into());
        ctx.fill_text(&kind.to_string(), x + width_f64 / 2.0, y + height_f64 / 2.0)
            .unwrap();
    }
}

impl Particle for ChangeParticle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State, _: &mut ActionDispacher) {
        let a = self.a;
        let b = self.b;

        let a_block = state.blocks.pick(a).unwrap();
        let a_color = self.colors.get(a_block.kind);

        let b_block = state.blocks.pick(b).unwrap();
        let b_color = self.colors.get(b_block.kind);

        ctx.begin_path();

        self.draw_block(ctx, a, b, a_block.kind, a_color);
        self.draw_block(ctx, b, a, b_block.kind, b_color);

        ctx.stroke();
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.a, self.b);
    }
}
