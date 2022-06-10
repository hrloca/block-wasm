use crate::board::*;
use crate::console_log;
use crate::store::State;
use js_sys::Date;
use std::collections::HashMap;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::*;

struct Colors {
    repos: HashMap<u8, String>,
}

impl Colors {
    pub fn create() -> Colors {
        Colors {
            repos: HashMap::from([
                (1, String::from("#ff80ab")),
                (2, String::from("#8c9eff")),
                (3, String::from("#b9f6ca")),
                (4, String::from("#ffff8d")),
                (5, String::from("#ffd180")),
            ]),
        }
    }

    pub fn get(&self, id: u8) -> &str {
        self.repos.get(&id).unwrap()
    }
}

pub trait Particle {
    fn draw(&self) -> ();
    fn is_finish(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct ChangeParticle {
    total: f64,
    created: f64,
}

impl ChangeParticle {
    pub fn new(a: Point, to_b: Point) -> ChangeParticle {
        ChangeParticle {
            created: Date::new_0().get_time(),
            total: 1000.0,
        }
    }
}

impl Particle for ChangeParticle {
    fn draw(&self) {}
    fn is_finish(&self) -> bool {
        let now = Date::new_0().get_time();
        let elapsed = now - self.created;
        console_log!("{:?}", elapsed);
        elapsed > self.total
    }
}

pub struct Canvas<P>
where
    P: Particle + Copy,
{
    body: HtmlCanvasElement,
    main: Rc<CanvasRenderingContext2d>,
    particles: Vec<P>,
    colors: Colors,
}

impl<P> Canvas<P>
where
    P: Particle + Copy,
{
    pub fn create(canvas_element: HtmlCanvasElement) -> Self {
        let context = canvas_element.get_context("2d").unwrap().unwrap();
        let context = JsCast::dyn_into::<CanvasRenderingContext2d>(context).unwrap();

        Canvas {
            colors: Colors::create(),
            body: canvas_element,
            particles: vec![],
            main: Rc::new(context),
        }
    }

    pub fn initialize(self) -> Self {
        self.body.set_width(500);
        self.body.set_height(500);
        self
    }

    pub fn run_particles(&mut self) {
        self.filter_particles();
    }

    fn filter_particles(&mut self) {
        let new_iter: Vec<P> = self
            .particles
            .iter()
            .filter(|&x| !x.is_finish())
            .map(|&x| x)
            .collect();

        self.particles = new_iter;
    }

    pub fn set_particles(&mut self, p: P) {
        self.particles.push(p);
    }

    pub fn get_particles(&self) -> &Vec<P> {
        &self.particles
    }

    pub fn export(&self) -> &HtmlCanvasElement {
        &self.body
    }

    pub fn render(&mut self, state: &State) {
        self.run_particles();
        self.main.clear_rect(
            0.0,
            0.0,
            self.body.width().into(),
            self.body.height().into(),
        );

        {
            let ctx = Rc::clone(&self.main);
            ctx.begin_path();
            state.blocks.each(|(point, block)| {
                let width = 100.0;
                let height = 100.0;
                let x = point.x as f64 * width;
                let y = point.y as f64 * height;
                let color = match block {
                    Some(x) => self.colors.get(x.kind),
                    None => "#ffffff",
                };

                ctx.set_fill_style(&color.into());
                ctx.rect(x, y, width, height);
                ctx.fill_rect(x, y, width, height);

                ctx.set_fill_style(&"rgb(0,0,0)".into());
                if let Some(block) = block {
                    ctx.fill_text(&block.kind.to_string(), x + width / 2.0, y + height / 2.0)
                        .unwrap();
                }
            });

            self.main.stroke();
        }
    }
}
