use rand::prelude::*;

pub enum Dir {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Connect {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Connect {
    pub fn init() -> Connect {
        Connect {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub kind: u8,
    pub connect: Connect,
}

impl Block {
    pub fn create(kind: u8) -> Block {
        Block {
            kind,
            connect: Connect::init(),
        }
    }
    pub fn compare_kind(&self, other: &Block) -> bool {
        self.kind == other.kind
    }
}

pub type BlockRow = Vec<Block>;
pub type BlocksBody = Vec<BlockRow>;
pub type BlockAdress = (usize, usize);
type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub struct Blocks {
    pub row: usize,
    pub col: usize,
    pub kinds: u8,
    pub body: BlocksBody,
}

impl Blocks {
    // random thread_rng().gen_range(1..kinds)
    pub fn connect(self) -> Blocks {
        Blocks {
            body: connect(&self.body),
            ..self
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.body[0].len(), self.body.len())
    }
}

fn max(body: &BlocksBody) -> Pos {
    (body[0].len(), body.len())
}

pub fn block(body: &BlocksBody, pos: Pos) -> Option<&Block> {
    let (rowmax, colmax) = max(body);
    match pos {
        (row, col) if row >= rowmax || col >= colmax => None,
        _ => Some(&body[pos.0][pos.1]),
    }
}

pub fn group(body: BlocksBody) {
    let mut clone = body.clone();
}

pub fn grouping(body: &mut BlocksBody, current: (usize, usize), kind: Option<u8>) {
    let kind = kind.unwrap_or(0);
}

pub fn connect(blocks: &BlocksBody) -> BlocksBody {
    let (row_max, col_max) = max(blocks);
    blocks
        .iter()
        .enumerate()
        .map(|(row, row_block)| {
            row_block
                .iter()
                .enumerate()
                .map(|(col, &block)| {
                    let connect = Connect {
                        top: match maybe_prev(row) {
                            None => false,
                            Some(x) => block.kind == blocks[x][col].kind,
                        },
                        bottom: match maybe_next(row, row_max) {
                            None => false,
                            Some(x) => block.kind == blocks[x][col].kind,
                        },
                        left: match maybe_prev(col) {
                            None => false,
                            Some(x) => block.kind == blocks[row][x].kind,
                        },
                        right: match maybe_next(col, col_max) {
                            None => false,
                            Some(x) => block.kind == blocks[row][x].kind,
                        },
                    };

                    Block {
                        kind: block.kind,
                        connect: connect,
                    }
                })
                .collect()
        })
        .collect()
}

pub fn try_next_pos(body: &BlocksBody, pos: Pos, dir: Dir) -> Option<Pos> {
    let (row_max, col_max) = max(body);
    match dir {
        Dir::Top => try_row((maybe_prev(pos.0), pos.1)),
        Dir::Bottom => try_row((maybe_next(pos.0, row_max), pos.1)),
        Dir::Right => try_col((pos.0, maybe_next(pos.1, col_max))),
        Dir::Left => try_col((pos.0, maybe_prev(pos.1))),
    }
}

pub fn next_pos(body: &BlocksBody) -> impl Fn(Pos, Dir) -> Option<Pos> + '_ {
    move |pos: Pos, dir: Dir| -> Option<Pos> { try_next_pos(body, pos, dir) }
}

pub fn try_row(pos: (Option<usize>, usize)) -> Option<Pos> {
    match pos.0 {
        None => None,
        Some(row) => Some((row, pos.1)),
    }
}

pub fn try_col(pos: (usize, Option<usize>)) -> Option<Pos> {
    match pos.1 {
        None => None,
        Some(col) => Some((pos.0, col)),
    }
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
