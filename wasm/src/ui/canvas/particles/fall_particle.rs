use super::super::super::Easing;
use super::super::*;
use super::*;
use crate::board::*;
use js_sys::Date;

type Finished = Box<dyn Fn(&mut ActionDispacher, Point, Point)>;

pub struct FallParticle {
    speed: f64,
    created: f64,
    from: Point,
    to: Point,
    finished: Finished,
    colors: Colors,
}

impl FallParticle {
    pub fn create(from: Point, to: Point, finished: Finished) -> Self {
        FallParticle {
            colors: Colors::create(),
            from,
            to,
            created: Date::new_0().get_time(),
            speed: 0.007,
            finished,
        }
    }

    fn elapsed(&self) -> f64 {
        let now = Date::new_0().get_time();
        now - self.created
    }

    fn progress(&self) -> f64 {
        self.clamp(self.elapsed() / self.total(), 1.0, 0.0)
    }

    fn rate(&self) -> f64 {
        Easing::ease_in_cubic(self.progress())
    }

    fn clamp(&self, num: f64, min: f64, max: f64) -> f64 {
        num.max(max).min(min)
    }

    fn draw_block(&self, ctx: &CanvasRenderingContext2d, from: Point, to: Point, color: &str) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let from = Point::of(from.x * width, from.y * height);
        let to = Point::of(to.x * width, to.y * height);

        let rate = self.rate();

        let x = from.x as f64 + (to.x as f64 - from.x as f64) * rate;
        let y = from.y as f64 + (to.y as f64 - from.y as f64) * rate;
        BlockShape::create((x, y), color).draw(ctx);
    }

    fn total(&self) -> f64 {
        let distance = (self.to.y - self.from.y) as f64;
        distance / self.speed
    }
}

impl Particle for FallParticle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State, _: &mut ActionDispacher) {
        let target_point = self.from;

        let block = state.blocks.pick(target_point).as_ref().unwrap();
        let color = self.colors.get(block.kind);

        self.draw_block(ctx, target_point, self.to, color);
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total()
    }

    fn finish(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.from, self.to);
    }
}
