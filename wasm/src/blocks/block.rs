// use uuid::Uuid;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Block {
    pub id: usize,
    pub kind: u8,
}

impl Block {
    pub fn of(kind: u8) -> Block {
        Block { id: 0, kind }
    }

    pub fn a(id: usize, kind: u8) -> Option<Block> {
        Some(Block { id, kind })
    }

    pub fn equals(&self, other: &Block) -> bool {
        self.id == other.id
    }

    pub fn same_kinds(&self, other: &Block) -> bool {
        self.kind == other.kind
    }
}
