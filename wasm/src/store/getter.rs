use super::*;
use crate::board::*;

pub struct Getter<'a> {
    state: &'a State,
}

impl<'a> Getter<'a> {
    pub fn new(state: &'a State) -> Self {
        Getter { state }
    }
}
