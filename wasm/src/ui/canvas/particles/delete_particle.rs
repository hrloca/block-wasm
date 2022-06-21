use super::*;
use crate::board::*;
use js_sys::Date;

type CallBack = Box<dyn Fn(&mut ActionDispacher, Vec<Point>)>;

pub struct DeleteParticle {
    total: f64,
    created: f64,
    delete: Vec<Point>,
    finished: CallBack,
    start: CallBack,
    colors: Colors,
    off: bool,
    drawed: bool,
}

impl DeleteParticle {
    pub fn create(delete: Vec<Point>, start: CallBack, finished: CallBack) -> Self {
        DeleteParticle {
            colors: Colors::create(),
            off: false,
            delete,
            created: Date::new_0().get_time(),
            total: 500.0,
            finished,
            start,
            drawed: false,
        }
    }

    fn elapsed(&self) -> f64 {
        let now = Date::new_0().get_time();
        now - self.created
    }

    fn delete_draw(&self, ctx: &CanvasRenderingContext2d, point: Point, color: &str) {
        let x = point.x as f64 * WIDTH;
        let y = point.y as f64 * HEIGHT;

        if !self.off {
            BlockShape::create((x, y), color).draw(ctx);
        }
    }
}

impl Particle for DeleteParticle {
    fn name(&self) -> String {
        String::from("delete_particle")
    }
    fn is_drawed(&self) -> bool {
        self.drawed
    }
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State) {
        if !self.drawed {
            self.drawed = true;
        }
        self.delete.iter().for_each(|p| {
            let block = state.blocks.pick(*p).as_ref().unwrap();
            let color = self.colors.get(block.kind);
            self.delete_draw(ctx, *p, color);
        });

        self.off = !self.off;
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.delete.clone());
    }

    fn start(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.start)(action, self.delete.clone());
    }
}
