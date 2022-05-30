use super::block::*;
use super::table::*;

#[derive(Debug, Clone)]
pub struct Blocks {
    pub table: Table<Block>,
}

impl Blocks {
    pub fn connect(&self) -> Table<Block> {
        self.table.rebuild(|cood, block| Block {
            connect: self.table.around(cood).map(|cell| {
                if let Some(i) = cell {
                    return i.kind == block.kind;
                }
                false
            }),
            ..*block
        })
    }
}
