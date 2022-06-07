#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Block {
    pub kind: u8,
}

impl Block {
    pub fn of(kind: u8) -> Block {
        Block { kind }
    }

    pub fn is_same_kind(&self, other: &Block) -> bool {
        self.kind == other.kind
    }
}
