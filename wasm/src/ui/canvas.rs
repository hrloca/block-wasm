use crate::store;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Canvas {
    body: HtmlCanvasElement,
    context: Rc<CanvasRenderingContext2d>,
}

impl Canvas {
    pub fn create(canvas_element: HtmlCanvasElement) -> Canvas {
        let context = canvas_element.get_context("2d").unwrap().unwrap();
        let context = JsCast::dyn_into::<CanvasRenderingContext2d>(context).unwrap();

        Canvas {
            body: canvas_element,
            context: Rc::new(context),
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

    pub fn render(&self, state: &store::State) {
        self.context.as_ref().clear_rect(
            0.0,
            0.0,
            self.body.width().into(),
            self.body.height().into(),
        );

        {
            let ctx_for_blocks = self.context.clone();
            self.context.as_ref().begin_path();
            state.blocks.each(|(point, block)| {
                let ctx = ctx_for_blocks.as_ref();
                let width = 100.0;
                let height = 100.0;
                let x = point.x as f64 * width;
                let y = point.y as f64 * height;
                ctx.set_fill_style(&"rgb(100,100,100)".into());
                ctx.rect(x, y, width, height);
                ctx.fill_rect(x, y, width, height);

                ctx.set_fill_style(&"rgb(255,255,255)".into());
                if let Some(block) = block {
                    ctx.fill_text(&block.kind.to_string(), x + width / 2.0, y + height / 2.0)
                        .unwrap();
                }
            });
            self.context.as_ref().stroke();
        }
    }
}
