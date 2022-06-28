use super::super::super::Easing;
use super::*;
use js_sys::Date;

pub struct TouchParticle {
    pub core: ParticleCore,
    target: Point,
    completed: bool,
}

impl TouchParticle {
    pub fn create(target: Point) -> Self {
        TouchParticle {
            target,
            core: ParticleCore::create(200.),
            completed: false,
        }
    }

    fn draw_particle(&self, ctx: &CanvasRenderingContext2d, target: Point) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let origin = Point::of(target.x * width, target.y * height);
        let rate = self.core.rate_with(Easing::ease_in_out_cubic);

        let x = origin.x as f64 - (4.0 * rate);
        let y = origin.y as f64 - (4.0 * rate);

        let opacity = 0.8 - (0.8 * rate);
        let color = String::from("rgba(255,255,255,") + &opacity.to_string() + ")";
        let wakucolor = String::from("rgba(150,150,150,") + &opacity.to_string() + ")";

        ctx.begin_path();
        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x, y, WIDTH + 8.0 * rate, HEIGHT + 8.0 * rate);
        ctx.set_stroke_style(&wakucolor.into());
        ctx.rect(x, y, WIDTH + 8.0 * rate, HEIGHT + 8.0 * rate);
        ctx.stroke();
    }
}

impl ParticleEntity for TouchParticle {
    fn draw(&mut self, context: &crate::Ctx) {
        if self.completed {
            return;
        }
        let ctx = context.canvas_ctx;
        let state = context.state;
        if !self.core.is_enter() {
            self.core.start_at(Date::new_0().get_time());
        }

        self.draw_particle(ctx, self.target);
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

    fn complete(&self, _: &crate::Ctx) {}
    fn started(&self, _: &crate::Ctx) {}
}
