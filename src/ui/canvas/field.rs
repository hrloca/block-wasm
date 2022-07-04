use super::*;
use crate::blocks::*;
use crate::board::*;
use crate::store::*;
use std::f64;
use web_sys::*;

pub struct Field {
    colors: Colors,
    width: f64,
    height: f64,
}

impl Field {
    pub fn create(
        canvas: &HtmlCanvasElement,
        row: u8,
        col: u8,
        block_width: f64,
        block_height: f64,
    ) -> Self {
        let width = block_width * col as f64;
        let height = block_height * row as f64;
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        canvas.set_class_name("absolute inset-2/4 translate-y-[-50%] translate-x-[-50%]");

        Field {
            colors: Colors::create(),
            width,
            height,
        }
    }

    pub fn offset_to_point(point: (i32, i32)) -> Point {
        let width = WIDTH as i32;
        let height = HEIGHT as i32;
        let x = point.0 / width;
        let y = point.1 / height;
        Point::of(x as usize, y as usize)
    }

    pub fn render(&self, context: &crate::Ctx) {
        let ctx = context.canvas_ctx;
        let state = context.state;
        ctx.clear_rect(0.0, 0.0, self.width, self.height);

        {
            ctx.begin_path();

            state.blocks.each(|(point, block)| {
                let width = WIDTH;
                let height = HEIGHT;
                let x = point.x as f64 * width;
                let y = point.y as f64 * height;
                let color = match block {
                    Some(x) => self.colors.get(x.kind),
                    None => "#ffffff",
                };

                if !state.changing_point.contains(&point)
                    && !state.deleting_point.contains(&point)
                    && !state.falling_point.contains(&point)
                {
                    BlockShape::create((x, y), color).draw(&ctx);
                }
            });

            if is_over(&state.blocks) {
                ctx.set_fill_style(&"rgba(255,255,255,0.7)".into());
                ctx.fill_rect(0.0, 0.0, self.width, self.height);
                ctx.set_fill_style(&"rgba(100,100,100,1)".into());
                ctx.set_font("bold 36px selif");
                ctx.fill_text("finish!", self.width / 2. - 48., self.height / 2.);
            }
            ctx.stroke();
        }
    }
}
