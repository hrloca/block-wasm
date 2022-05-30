use super::block::*;
use super::table::*;

#[derive(Debug, Clone)]
pub struct Blocks {
    pub table: Table<Block>,
}

impl Blocks {
    pub fn create(table: Table<Block>) -> Blocks {
        Blocks { table }
    }

    pub fn connect(&self) -> Blocks {
        Blocks::create(self.table.rebuild(|cood, block| Block {
            connect: self.table.around(cood).map(|cell| match cell {
                Some(i) => i.is_same_with(block),
                None => false,
            }),
            ..*block
        }))
    }

    pub fn change(&self, from: Pos, to: Pos) -> Blocks {
        Blocks::create(self.table.rebuild(|cood, block| match cood {
            x if x.equals(from) => self.table.pick(to).unwrap().clone(),
            x if x.equals(to) => self.table.pick(from).unwrap().clone(),
            _ => block.clone(),
        }))
    }
}
