use super::*;
use js_sys::Date;

pub struct DeleteParticle {
    pub core: ParticleCore,
    delete: Vec<Point>,
    on: bool,
    completed: bool,
}

impl DeleteParticle {
    pub fn create(delete: Vec<Point>) -> Self {
        DeleteParticle {
            delete,
            on: false,
            core: ParticleCore::create(500.),
            completed: false,
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

impl ParticleEntity for DeleteParticle {
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

        self.delete.iter().for_each(|p| {
            if let Some(block) = state.blocks.pick(*p) {
                let color = colors.get(block.kind);
                self.delete_draw(ctx, *p, color);
            }
        });

        self.on = !self.on;

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
        context.action_dispacher.delete(self.delete.clone());
    }

    fn started(&self, context: &crate::Ctx) {
        context.action_dispacher.will_delete(self.delete.clone());
    }
}
