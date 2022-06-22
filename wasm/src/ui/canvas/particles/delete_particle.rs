use super::*;
use crate::board::*;
use crate::uuid;
use js_sys::Date;
use std::cell::Cell;

type CallBack = Box<dyn Fn(Vec<Point>)>;

pub struct DeleteParticle {
    total: f64,
    started: Cell<Option<f64>>,
    delete: Vec<Point>,
    finished: CallBack,
    start: CallBack,
    colors: Colors,
    off: Cell<bool>,
    drawed: Cell<bool>,
    id: String,
}

impl DeleteParticle {
    pub fn create(delete: Vec<Point>, start: CallBack, finished: CallBack) -> Self {
        DeleteParticle {
            id: uuid(),
            colors: Colors::create(),
            off: Cell::new(false),
            delete,
            started: Cell::new(None),
            total: 500.0,
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

    fn delete_draw(&self, ctx: &CanvasRenderingContext2d, point: Point, color: &str) {
        let x = point.x as f64 * WIDTH;
        let y = point.y as f64 * HEIGHT;

        if !self.off.get() {
            BlockShape::create((x, y), color).draw(ctx);
        }
    }
}

impl Particle for DeleteParticle {
    fn name(&self) -> String {
        String::from("delete_particle")
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn is_drawed(&self) -> bool {
        self.drawed.get()
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d, state: &State) {
        if !self.drawed.get() {
            self.drawed.set(true);
            self.started.set(Some(Date::new_0().get_time()));
        }
        self.delete.iter().for_each(|p| {
            let block = state.blocks.pick(*p).as_ref().unwrap();
            let color = self.colors.get(block.kind);
            self.delete_draw(ctx, *p, color);
        });

        self.off.set(!self.off.get());
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&self, _: &State) {
        (self.finished)(self.delete.clone());
    }

    fn start(&self, _: &State) {
        (self.start)(self.delete.clone());
    }
}
