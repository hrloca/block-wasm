use super::*;
use crate::board::*;
use crate::log;
use crate::store::*;
use js_sys::Date;

type Finished = Box<dyn Fn(&mut ActionDispacher, Point, Point)>;
pub struct FallParticle {
    total: f64,
    created: f64,
    from: Point,
    to: Point,
    finished: Finished,
}
