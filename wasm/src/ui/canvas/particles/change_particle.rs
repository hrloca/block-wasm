use super::super::super::Easing;
use super::*;
use crate::board::*;
use crate::uuid;
use js_sys::Date;
use std::cell::Cell;

type CallBack = Box<dyn Fn(Point, Point)>;
pub struct ChangeParticle {
    total: f64,
    started: Cell<Option<f64>>,
    a: Point,
    b: Point,
    finished: CallBack,
    start: CallBack,
    colors: Colors,
    drawed: Cell<bool>,
    id: String,
}

impl ChangeParticle {
    pub fn create(a: Point, b: Point, start: CallBack, finished: CallBack) -> ChangeParticle {
        ChangeParticle {
            id: uuid(),
            colors: Colors::create(),
            a,
            b,
            started: Cell::new(None),
            total: 300.0,
            finished,
            start,
            drawed: Cell::new(false),
        }
    }

    fn elapsed(&self) -> f64 {
        if let Some(started) = self.started.get() {
            let now = Date::new_0().get_time();
            return now - started;
        }
        0.0
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
}

impl Particle for ChangeParticle {
    fn name(&self) -> String {
        String::from("change_particle")
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn is_drawed(&self) -> bool {
        self.drawed.get()
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d, state: &State) {
        if !self.is_drawed() {
            self.drawed.set(true);
            self.started.set(Some(Date::new_0().get_time()));
        }
        let a = self.a;
        let b = self.b;

        let a_block = state.blocks.pick(a).as_ref().unwrap();
        let a_color = self.colors.get(a_block.kind);

        let b_block = state.blocks.pick(b).as_ref().unwrap();
        let b_color = self.colors.get(b_block.kind);

        self.draw_block(ctx, a, b, a_color);
        self.draw_block(ctx, b, a, b_color);
    }

    fn is_finish(&self) -> bool {
        self.elapsed() >= self.total
    }

    fn finish(&self, _: &State) {
        (self.finished)(self.a, self.b);
    }

    fn start(&self, _: &State) {
        (self.start)(self.a, self.b);
    }
}
