use super::*;
use crate::store::*;
use crate::tools::store::Store;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Canvas<P>
where
    P: Particle,
{
    body: HtmlCanvasElement,
    main: Rc<CanvasRenderingContext2d>,
    particles: Vec<P>,
    colors: BlockColors,
}

impl<P> Canvas<P>
where
    P: Particle,
{
    pub fn create(canvas_element: HtmlCanvasElement) -> Self {
        let context = canvas_element.get_context("2d").unwrap().unwrap();
        let context = JsCast::dyn_into::<CanvasRenderingContext2d>(context).unwrap();

        Canvas {
            colors: BlockColors::create(),
            body: canvas_element,
            particles: Vec::new(),
            main: Rc::new(context),
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
            .for_each(|p| p.draw(state, action));

        self.particles.retain_mut(|p| {
            if !p.is_finish() {
                return true;
            }
            p.finish(state, action);
            false
        });
    }

    pub fn render(&mut self, state: &State, action: &mut ActionDispacher) {
        self.draw_particles(state, action);

        let ctx = &self.main;

        ctx.clear_rect(
            0.0,
            0.0,
            self.body.width().into(),
            self.body.height().into(),
        );

        {
            let ctx = &self.main;
            ctx.begin_path();
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
                        ctx.set_fill_style(&color.into());
                        ctx.rect(x, y, width, height);
                        ctx.fill_rect(x, y, width, height);
                        ctx.set_fill_style(&"rgb(0,0,0)".into());
                        ctx.fill_text(&block.kind.to_string(), x + width / 2.0, y + height / 2.0)
                            .unwrap();
                    }
                }
            });

            self.main.stroke();
        }
    }
}
