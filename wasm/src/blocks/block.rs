use crate::uuid;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Block {
    pub id: String,
    pub kind: u8,
}

impl Block {
    pub fn of(kind: u8) -> Self {
        Block { id: uuid(), kind }
    }

    pub fn equals(&self, other: &Block) -> bool {
        self.id == other.id
    }

    pub fn same_kinds(&self, other: &Block) -> bool {
        self.kind == other.kind
    }
}
