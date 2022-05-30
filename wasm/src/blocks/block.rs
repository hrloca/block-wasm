use super::table::*;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub kind: u8,
    pub connect: Around<bool>,
}

impl Block {
    pub fn is_same_with(&self, other: &Block) -> bool {
        self.kind == other.kind
    }
}
