use super::*;
use crate::board::*;
use crate::log;
use crate::store::*;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Canvas {
    ctx: CanvasRenderingContext2d,
    body: Rc<HtmlCanvasElement>,
    particles: Vec<Box<dyn Particle>>,
    colors: Colors,
    width: f64,
    height: f64,
}

impl Canvas {
    pub fn create(canvas: Rc<HtmlCanvasElement>) -> Self {
        let ctx = canvas.get_context("2d").unwrap().unwrap();
        let ctx = JsCast::dyn_into::<CanvasRenderingContext2d>(ctx).unwrap();

        canvas.set_width(500);
        canvas.set_height(500);

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        Canvas {
            colors: Colors::create(),
            particles: Vec::new(),
            body: canvas,
            ctx,
            width,
            height,
        }
    }

    pub fn get_particles(&self) -> &Vec<Box<dyn Particle>> {
        &self.particles
    }

    pub fn draw_particle(&mut self, p: Box<dyn Particle>) {
        self.particles.push(p);
    }

    pub fn draw_particles(&mut self, state: &State, action: &mut ActionDispacher) {
        self.particles.iter_mut().for_each(|p| {
            if !p.is_drawed() {
                p.start(state, action);
            }

            p.draw(&self.ctx, state, action);

            if p.is_finish() {
                p.finish(state, action);
            }
        });

        self.particles.retain_mut(|p| !p.is_finish());
    }

    pub fn with_point(&mut self, point: (i32, i32)) -> Point {
        let width = WIDTH as i32;
        let height = HEIGHT as i32;
        let x = point.0 / width;
        let y = point.1 / height;
        Point::of(x as usize, y as usize)
    }

    pub fn render(&mut self, state: &State, action: &mut ActionDispacher) {
        self.ctx.clear_rect(0.0, 0.0, self.width, self.height);

        {
            self.ctx.begin_path();

            state.blocks.each(|(point, block)| {
                let width = WIDTH;
                let height = HEIGHT;
                let x = point.x as f64 * width;
                let y = point.y as f64 * height;
                let color = match block {
                    Some(x) => self.colors.get(x.kind),
                    None => "#ffffff",
                };

                if let Some(block) = block {
                    let id = &block.id.to_string();
                    if state.changing.get(id).is_none()
                        && state.deleting.get(id).is_none()
                        && state.falling.get(id).is_none()
                    {
                        BlockShape::create((x, y), color).draw(&self.ctx);
                    }
                }
            });

            self.ctx.stroke();

            self.draw_particles(state, action);
        }
    }
}
