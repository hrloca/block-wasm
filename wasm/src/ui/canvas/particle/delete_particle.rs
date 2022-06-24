use super::*;
use js_sys::Date;

pub struct NewDeleteParticle {
    pub core: ParticleCore,
    delete: Vec<Point>,
    on: bool,
}

impl NewDeleteParticle {
    pub fn create(delete: Vec<Point>) -> Self {
        NewDeleteParticle {
            delete,
            on: false,
            core: ParticleCore::create(500.),
        }
    }

    fn delete_draw(&self, ctx: &CanvasRenderingContext2d, point: Point, color: &str) {
        let x = point.x as f64 * WIDTH;
        let y = point.y as f64 * HEIGHT;

        if !self.on {
            BlockShape::create((x, y), color).draw(ctx);
        }
    }
}

impl ParticleEntity for NewDeleteParticle {
    fn draw(&mut self, ctx: &CanvasRenderingContext2d, state: &State) {
        let colors = Colors::create();
        if !self.core.is_enter() {
            self.core.start_at(Date::new_0().get_time());
        }

        self.delete.iter().for_each(|p| {
            let block = state.blocks.pick(*p).as_ref().unwrap();
            let color = colors.get(block.kind);
            self.delete_draw(ctx, *p, color);
        });

        self.on = !self.on;
    }

    fn is_complete(&self) -> bool {
        self.core.is_exit()
    }

    fn is_started(&self) -> bool {
        self.core.is_enter()
    }
}
