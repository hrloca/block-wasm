use super::super::*;
use super::*;
use crate::board::*;

pub fn change(canvas: &mut Canvas, action: &mut ActionDispacher, a: Point, b: Point) {
    action.will_change(a, b);
    canvas.draw_particle(Box::new(ui::ChangeParticle::create(
        a,
        b,
        Box::new(|action, a, b| {
            action.change(a, b);
        }),
    )));

    canvas.draw_particle(Box::new(ui::TouchParticle::create(a, Box::new(|_, _| {}))));
}
