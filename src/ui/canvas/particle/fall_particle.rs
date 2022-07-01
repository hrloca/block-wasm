use super::super::super::Easing;
use super::*;
use crate::board::*;
use js_sys::Date;

const G: f64 = 9.80665;

fn px_to_m(px: f64) -> f64 {
    px / 1000.0
}

fn elapsed(px: f64) -> f64 {
    let h = px_to_m(px);
    (2.0 * h / G).sqrt() * 1000.0
}

pub struct FallParticle {
    pub core: ParticleCore,
    from: Point,
    to: Point,
    completed: bool,
}

impl FallParticle {
    pub fn create(from: Point, to: Point) -> Self {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let _from = Point::of(from.x * width, from.y * height);
        let _to = Point::of(to.x * width, to.y * height);

        let distance = _to.y - _from.y;
        let total = elapsed(distance as f64);

        FallParticle {
            from,
            to,
            core: ParticleCore::create(total),
            completed: false,
        }
    }

    fn draw_block(&self, ctx: &CanvasRenderingContext2d, from: Point, _: Point, color: &str) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let from = Point::of(from.x * width, from.y * height);

        // TODO: 適当数値だからちゃんとする
        let dis = (self.core.elapsed() / 1000.0).powf(2.0) * G / 2.0 * 750.0;

        let x = from.x as f64;
        let y = from.y as f64 + dis;

        BlockShape::create((x, y), color).draw(ctx);
    }
}

impl ParticleEntity for FallParticle {
    fn draw(&mut self, context: &crate::Ctx) {
        if self.completed {
            return;
        }
        let ctx = context.canvas_ctx;
        let state = context.state;
        let colors = Colors::create();
        if !self.core.is_enter() {
            self.core.start_at(Date::new_0().get_time());
        }

        let target_point = self.from;
        if let Some(block) = state.blocks.pick(target_point) {
            let color = colors.get(block.kind);
            self.draw_block(ctx, target_point, self.to, color);
        }
        if self.is_complete() {
            self.completed = true;
        }
    }

    fn is_complete(&self) -> bool {
        self.core.is_exit()
    }

    fn is_completed(&self) -> bool {
        self.completed
    }

    fn is_started(&self) -> bool {
        self.core.is_enter()
    }

    fn complete(&self, context: &crate::Ctx) {
        context.se.landing.play();
        context.action_dispacher.fall(self.from, self.to);
    }

    fn started(&self, context: &crate::Ctx) {
        context.action_dispacher.will_fall(self.from, self.to);
    }
}
