use super::super::super::Easing;
use super::*;
use crate::board::*;
use crate::uuid;
use js_sys::Date;

pub struct TouchParticle {
    total: f64,
    started: Option<f64>,
    target: Point,
    drawed: bool,
    id: String,
}

impl TouchParticle {
    pub fn create(target: Point) -> Self {
        TouchParticle {
            target,
            started: None,
            total: 300.0,
            drawed: false,
            id: uuid(),
        }
    }

    fn elapsed(&self) -> f64 {
        if let Some(started) = self.started {
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

    fn draw_particle(&self, ctx: &CanvasRenderingContext2d, target: Point) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let origin = Point::of(target.x * width, target.y * height);
        let rate = self.rate();

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

impl Particle for TouchParticle {
    fn name(&self) -> String {
        String::from("touch_particle")
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, _: &State) {
        if !self.drawed {
            self.drawed = true;
            self.started = Some(Date::new_0().get_time());
        }
        let target = self.target;
        self.draw_particle(ctx, target);
    }
    fn is_drawed(&self) -> bool {
        self.drawed
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&self, _: &State) {}

    fn start(&self, _: &State) {}
}
