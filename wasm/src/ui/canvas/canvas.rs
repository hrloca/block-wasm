use super::*;
use crate::store::*;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Canvas<P>
where
    P: Particle,
{
    body: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    ctx_for_particle: CanvasRenderingContext2d,
    particles: Vec<P>,
    colors: BlockColors,
}

impl<P> Canvas<P>
where
    P: Particle,
{
    pub fn create(canvas_element: HtmlCanvasElement) -> Self {
        let ctx = canvas_element.get_context("2d").unwrap().unwrap();
        let ctx = JsCast::dyn_into::<CanvasRenderingContext2d>(ctx).unwrap();

        let ctx_for_particle = canvas_element.get_context("2d").unwrap().unwrap();
        let ctx_for_particle =
            JsCast::dyn_into::<CanvasRenderingContext2d>(ctx_for_particle).unwrap();

        Canvas {
            colors: BlockColors::create(),
            body: canvas_element,
            particles: Vec::new(),
            ctx_for_particle,
            ctx,
        }
    }

    pub fn initialize(self) -> Self {
        self.body.set_width(500);
        self.body.set_height(500);
        self
    }

    pub fn export(&self) -> &HtmlCanvasElement {
        &self.body
    }

    pub fn get_particles(&self) -> &Vec<P> {
        &self.particles
    }

    pub fn draw_particle(&mut self, p: P) {
        self.particles.push(p);
    }

    pub fn draw_particles(&mut self, state: &State, action: &mut ActionDispacher) {
        self.particles
            .iter_mut()
            .for_each(|p| p.draw(&self.ctx_for_particle, state, action));

        self.particles.retain_mut(|p| {
            if !p.is_finish() {
                return true;
            }
            p.finish(state, action);
            false
        });
    }

    pub fn render(&mut self, state: &State, action: &mut ActionDispacher) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.body.width().into(),
            self.body.height().into(),
        );

        self.draw_particles(state, action);

        {
            self.ctx.begin_path();
            state.blocks.each(|(point, block)| {
                let width = 80.0;
                let height = 80.0;
                let x = point.x as f64 * width;
                let y = point.y as f64 * height;
                let color = match block {
                    Some(x) => self.colors.get(x.kind),
                    None => "#ffffff",
                };

                if let Some(block) = block {
                    if let Some(_) = state.locked.get(&block.id.to_string()) {
                    } else {
                        self.ctx.set_fill_style(&color.into());
                        self.ctx.rect(x, y, width, height);
                        self.ctx.fill_rect(x, y, width, height);
                        self.ctx.set_fill_style(&"rgb(0,0,0)".into());
                        self.ctx
                            .fill_text(&block.kind.to_string(), x + width / 2.0, y + height / 2.0)
                            .unwrap();
                    }
                }
            });

            self.ctx.stroke();
        }
    }
}
