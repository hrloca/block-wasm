use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub kind: u8,
    pub group: Option<u8>,
}

pub type BlockRow = Vec<Block>;
pub type Blocks = Vec<BlockRow>;
pub type BlockAdress = (usize, usize);

#[derive(Debug, Clone)]
pub struct Board {
    pub row: usize,
    pub col: usize,
    pub kinds: u8,
    pub blocks: Blocks,
}

impl Board {
    fn create(row: usize, col: usize) -> Board {
        let kinds = 4;
        Board {
            kinds,
            row,
            col,
            blocks: vec![
                vec![
                    Block {
                        kind: thread_rng().gen_range(1..kinds),
                        group: None,
                    };
                    col
                ];
                row
            ],
        }
    }

    fn blocks_random(kinds: u8) -> Blocks {
        vec![
            vec![
                Block {
                    kind: thread_rng().gen_range(1..kinds),
                    group: None,
                };
                4
            ];
            4
        ]
    }
}

pub fn connect(blocks: Blocks) -> Blocks {
    blocks
        .iter()
        .enumerate()
        .map(|(col, row_block)| {
            row_block
                .iter()
                .enumerate()
                .map(|(row, &block)| {
                    let currnet = (col, row);
                    println!("{:?}, {:?}", currnet, blocks[currnet.0][currnet.1]);
                    Block {
                        kind: block.kind,
                        group: block.group,
                    }
                })
                .collect()
        })
        .collect()
}

pub fn maybe_next(num: usize, max: usize) -> Option<usize> {
    match num {
        x if x + 1 >= max => None,
        _ => Some(num + 1),
    }
}

pub fn maybe_prev(num: usize) -> Option<usize> {
    match num {
        0 => None,
        _ => Some(num - 1),
    }
}
