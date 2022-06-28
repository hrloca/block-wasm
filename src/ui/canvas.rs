use wasm_bindgen::JsCast;
use web_sys::*;

mod block;
mod field;
mod particle;

pub use block::*;
pub use field::*;
pub use particle::*;

const CANVAS_NAME: &str = "board";

pub struct Canvas {
    pub ctx: CanvasRenderingContext2d,
    pub el: HtmlCanvasElement,
}

impl Canvas {
    pub fn create(el: HtmlCanvasElement) -> Self {
        el.set_attribute("id", CANVAS_NAME).unwrap();
        let ctx = el.get_context("2d").unwrap().unwrap();
        let ctx = JsCast::dyn_into::<CanvasRenderingContext2d>(ctx).unwrap();

        Canvas { ctx, el }
    }
}
