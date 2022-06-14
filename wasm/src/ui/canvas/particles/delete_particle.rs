use super::super::*;
use super::*;
use crate::board::*;
use js_sys::Date;

type Finished = Box<dyn Fn(&mut ActionDispacher, Vec<Point>)>;

pub struct DeleteParticle {
    total: f64,
    created: f64,
    delete: Vec<Point>,
    finished: Finished,
    colors: BlockColors,
    off: bool,
}

impl DeleteParticle {
    pub fn create(delete: Vec<Point>, finished: Finished) -> Self {
        DeleteParticle {
            colors: BlockColors::create(),
            off: false,
            delete,
            created: Date::new_0().get_time(),
            total: 500.0,
            finished,
        }
    }

    fn elapsed(&self) -> f64 {
        let now = Date::new_0().get_time();
        now - self.created
    }

    fn delete_draw(&self, ctx: &CanvasRenderingContext2d, point: Point, kind: u8, color: &str) {
        let width = 80;
        let height = 80;
        let width_f64 = width as f64;
        let height_f64 = height as f64;

        let x = point.x as f64 * width_f64;
        let y = point.y as f64 * height_f64;

        if !self.off {
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(x, y, width_f64, height_f64);
            ctx.set_fill_style(&"rgb(0,0,0)".into());
            ctx.fill_text(&kind.to_string(), x + width_f64 / 2.0, y + height_f64 / 2.0)
                .unwrap();
        }
    }
}

impl Particle for DeleteParticle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State, _: &mut ActionDispacher) {
        self.delete.iter().for_each(|p| {
            let block = state.blocks.pick(*p).unwrap();
            let color = self.colors.get(block.kind);
            self.delete_draw(ctx, *p, block.kind, color);
        });

        self.off = !self.off;
    }

    fn is_finish(&self) -> bool {
        self.elapsed() > self.total
    }

    fn finish(&mut self, _: &State, action: &mut ActionDispacher) {
        (self.finished)(action, self.delete.clone());
    }
}
