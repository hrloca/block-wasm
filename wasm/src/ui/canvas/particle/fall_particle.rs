use super::super::super::Easing;
use super::*;
use crate::board::*;
use js_sys::Date;

const G: f64 = 9.80665;

fn px_to_m(px: f64) -> f64 {
    px / 500.0
}

fn elapsed(px: f64) -> f64 {
    let h = px_to_m(px);
    (2.0 * h / G).sqrt() * 1000.0
}

pub struct NewFallParticle {
    pub core: ParticleCore,
    from: Point,
    to: Point,
}

impl NewFallParticle {
    pub fn create(from: Point, to: Point) -> Self {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let _from = Point::of(from.x * width, from.y * height);
        let _to = Point::of(to.x * width, to.y * height);

        let distance = _to.y - _from.y;
        let total = elapsed(distance as f64);

        NewFallParticle {
            from,
            to,
            core: ParticleCore::create(total),
        }
    }

    fn draw_block(&self, ctx: &CanvasRenderingContext2d, from: Point, _: Point, color: &str) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let from = Point::of(from.x * width, from.y * height);

        // TODO: 400.0が適当数値だからちゃんとする
        let dis = (self.core.elapsed() / 1000.0).powf(2.0) * G / 2.0 * 400.0;

        let x = from.x as f64;
        let y = from.y as f64 + dis;

        BlockShape::create((x, y), color).draw(ctx);
    }
}

impl ParticleEntity for NewFallParticle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State) {
        let colors = Colors::create();
        if !self.core.is_enter() {
            self.core.start_at(Date::new_0().get_time());
        }

        let target_point = self.from;
        let block = state.blocks.pick(target_point).as_ref().unwrap();
        let color = colors.get(block.kind);
        self.draw_block(ctx, target_point, self.to, color);
    }

    fn is_complete(&self) -> bool {
        self.core.is_exit()
    }

    fn is_started(&self) -> bool {
        self.core.is_enter()
    }
}
