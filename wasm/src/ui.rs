use crate::dom;
use crate::store;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct UI {
    document: Document,
    window: Window,
    body: HtmlElement,
    canvas: HtmlCanvasElement,
    context: Rc<CanvasRenderingContext2d>,
}

impl UI {
    pub fn new() -> UI {
        let window = dom::window();
        let document = dom::document();
        let body = dom::body();
        let canvas = document.create_element("canvas").unwrap();
        let canvas = JsCast::dyn_into::<HtmlCanvasElement>(canvas).unwrap();
        let context = canvas.get_context("2d").unwrap().unwrap();
        let context = JsCast::dyn_into::<CanvasRenderingContext2d>(context).unwrap();

        UI {
            window,
            document,
            body,
            canvas,
            context: Rc::new(context),
        }
    }

    pub fn draw(&self, state: &store::State) {
        self.context.as_ref().clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
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

    pub fn render(&self, el: Element) {
        self.body.append_child(&el).unwrap();
    }

    pub fn set_canvas(&self) {
        self.canvas.set_width(500);
        self.canvas.set_height(500);
        self.body.append_child(&self.canvas).unwrap();
    }

    pub fn el(&self, name: &str, childs: Option<Vec<Element>>) -> Element {
        let el = self.document.create_element(name).unwrap();
        if let Some(li) = childs {
            li.iter().for_each(|child| {
                el.append_child(child).unwrap();
            });
        }
        el
    }
}
