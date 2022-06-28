use std::collections::HashMap;
use web_sys::*;

pub const WIDTH: f64 = 50.0;
pub const HEIGHT: f64 = 50.0;

pub struct BlockShape<'a> {
    origin: (f64, f64),
    color: &'a str,
}

impl<'a> BlockShape<'a> {
    pub fn create(origin: (f64, f64), color: &'a str) -> Self {
        BlockShape { origin, color }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let (x, y) = self.origin;
        ctx.begin_path();
        ctx.set_fill_style(&self.color.into());
        ctx.fill_rect(x, y, WIDTH, HEIGHT);
        ctx.stroke();
    }
}

pub struct Colors {
    repos: HashMap<u8, String>,
}

impl Colors {
    pub fn create() -> Self {
        Colors {
            repos: HashMap::from([
                (1, String::from("#ff80ab")),
                (2, String::from("#ffff8d")),
                (3, String::from("#b9f6ca")),
                (4, String::from("#8c9eff")),
                (5, String::from("#ffd180")),
                (6, String::from("#888888")),
                (7, String::from("#6a5acd")),
            ]),
        }
    }

    pub fn get(&self, id: u8) -> &str {
        self.repos.get(&id).unwrap()
    }
}
