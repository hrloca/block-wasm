use super::super::super::Easing;
use super::*;
use js_sys::Date;

pub struct ChangeParticle {
    pub core: ParticleCore,
    a: Point,
    b: Point,
}

impl ChangeParticle {
    pub fn create(a: Point, b: Point, dur: f64) -> Self {
        ChangeParticle {
            a,
            b,
            core: ParticleCore::create(dur),
        }
    }

    fn draw_block(&self, ctx: &CanvasRenderingContext2d, from: Point, to: Point, color: &str) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let from = Point::of(from.x * width, from.y * height);
        let to = Point::of(to.x * width, to.y * height);

        let rate = self.core.rate_with(Easing::ease_in_out_cubic);

        let x = from.x as f64 + (to.x as f64 - from.x as f64) * rate;
        let y = from.y as f64 + (to.y as f64 - from.y as f64) * rate;

        BlockShape::create((x, y), color).draw(ctx);
    }
}

impl ParticleEntity for ChangeParticle {
    fn draw(&mut self, context: &crate::Ctx) {
        let ctx = context.canvas_ctx;
        let state = context.state;
        let colors = Colors::create();
        if !self.core.is_enter() {
            self.core.start_at(Date::new_0().get_time());
        }

        let a = self.a;
        let b = self.b;

        let a_block = state.blocks.pick(a).as_ref().unwrap();
        let a_color = colors.get(a_block.kind);

        let b_block = state.blocks.pick(b).as_ref().unwrap();
        let b_color = colors.get(b_block.kind);

        self.draw_block(ctx, a, b, a_color);
        self.draw_block(ctx, b, a, b_color);
    }

    fn is_complete(&self) -> bool {
        self.core.is_exit()
    }

    fn is_started(&self) -> bool {
        self.core.is_enter()
    }

    fn complete(&self, context: &crate::Ctx) {
        context.action_dispacher.change(self.a, self.b);
    }

    fn started(&self, context: &crate::Ctx) {
        context.action_dispacher.will_change(self.a, self.b);
    }
}
