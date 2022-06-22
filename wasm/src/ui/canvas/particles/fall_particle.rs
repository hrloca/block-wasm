use super::*;
use crate::board::*;
use crate::uuid;
use js_sys::Date;

type CallBack = Box<dyn Fn(Point, Point)>;

const G: f64 = 9.80665;

fn px_to_m(px: f64) -> f64 {
    px / 500.0
}

fn elapsed(px: f64) -> f64 {
    let h = px_to_m(px);
    (2.0 * h / G).sqrt() * 1000.0
}

pub struct FallParticle {
    started: Option<f64>,
    from: Point,
    to: Point,
    finished: CallBack,
    colors: Colors,
    total: f64,
    start: CallBack,
    drawed: bool,
    id: String,
}

impl FallParticle {
    pub fn create(from: Point, to: Point, start: CallBack, finished: CallBack) -> Self {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let _from = Point::of(from.x * width, from.y * height);
        let _to = Point::of(to.x * width, to.y * height);

        let distance = _to.y - _from.y;
        let total = elapsed(distance as f64);

        FallParticle {
            id: uuid(),
            colors: Colors::create(),
            from,
            to,
            started: None,
            total,
            finished,
            start,
            drawed: false,
        }
    }

    fn elapsed(&self) -> f64 {
        if let Some(started) = self.started {
            let now = Date::new_0().get_time();
            return now - started;
        }
        0.0
    }

    fn draw_block(&self, ctx: &CanvasRenderingContext2d, from: Point, _: Point, color: &str) {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let from = Point::of(from.x * width, from.y * height);

        // TODO: 400.0が適当数値だからちゃんとする
        let dis = (self.elapsed() / 1000.0).powf(2.0) * G / 2.0 * 400.0;

        let x = from.x as f64;
        let y = from.y as f64 + dis;

        BlockShape::create((x, y), color).draw(ctx);
    }
}

impl Particle for FallParticle {
    fn name(&self) -> String {
        String::from("fall_particle")
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn is_drawed(&self) -> bool {
        self.drawed
    }
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State) {
        if !self.drawed {
            self.drawed = true;
            self.started = Some(Date::new_0().get_time());
        }
        let target_point = self.from;
        let block = state.blocks.pick(target_point).as_ref().unwrap();
        let color = self.colors.get(block.kind);
        self.draw_block(ctx, target_point, self.to, color);
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&self, _: &State) {
        (self.finished)(self.from, self.to);
    }
    fn start(&self, _: &State) {
        (self.start)(self.from, self.to);
    }
}
