use super::*;
use js_sys::Math::random;

pub type Cell = Option<Block>;
pub type BlockBoard = Board<Cell>;
pub type Builder = fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard;
pub type Processor = fn(&BlockBoard) -> BlockBoard;
pub type MoveList = Vec<Move>;
pub type PointList = Vec<Point>;
pub type BlockList = Vec<Block>;

pub const TOTAL_BLOCK_KIND: u8 = 7;
// pub const BOARD_COL: u8 = 11;
// pub const BOARD_RAW: u8 = 6;
pub const BOARD_COL: u8 = 10;
pub const BOARD_RAW: u8 = 10;

pub fn create() -> BlockBoard {
    Board::init(Size::of(BOARD_RAW.into(), BOARD_COL.into()), |_| {
        let kind = (random() * TOTAL_BLOCK_KIND as f64).ceil();
        Some(Block::of(kind as u8))
    })
}

pub fn delete(blocks: &BlockBoard, points: &PointList) -> BlockBoard {
    blocks.update(bla(points))
}

pub fn move_to(blocks: &BlockBoard, moves: &Vec<Move>) -> BlockBoard {
    blocks.update(mov(moves))
}

pub fn change(blocks: &BlockBoard, a: Point, b: Point) -> BlockBoard {
    blocks.update(cp(&vec![Move::of(a, b), Move::of(b, a)]))
}

pub fn extract_group(blocks: &BlockBoard) {
    let result: Vec<Vec<String>> = vec![];
    blocks.fold(&result, move |acc, (point, _)| {
        scan_blocks(&acc, point, Point::of(100, 100), blocks)
    });
}

pub fn scan_blocks<'a>(
    result: &'a Vec<Vec<String>>,
    current: Point,
    from: Point,
    blocks: &BlockBoard,
) -> &'a Vec<Vec<String>> {
    let top = blocks.top(current);
    let bottom = blocks.bottom(current);
    let left = blocks.left(current);
    let right = blocks.right(current);

    let block = blocks.pick(current);

    let scan = |block: &Block, target: Option<(Point, &Option<Block>)>| {
        if let Some((point, target)) = target {
            if point == from {
                return;
            }
            if let Some(target_block) = target {
                if block.same_kinds(target_block) {
                    dbg!(point);
                    scan_blocks(result, point, current, blocks);
                }
            };
        }
    };

    if let Some(block) = block {
        scan(block, top);
        scan(block, bottom);
        scan(block, left);
        scan(block, right);
    }

    result
}

// -------------------------------------

fn del(block_list: &BlockList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(block) => match block_list.iter().find(|b| b.equals(block)) {
            None => next,
            _ => next.insert(cur, None),
        },
    }
}

fn bla(points: &PointList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match points.iter().find(|x| x.equals(cur)) {
            None => next,
            _ => next.insert(cur, None),
        },
    }
}

fn mov(moves: &MoveList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match moves.iter().find(|x| x.from == (cur)) {
            None => next,
            Some(x) if has(&next, x.to) => next,
            Some(x) => next.insert(x.to, mayblock.clone()).insert(x.from, None),
        },
    }
}

fn cp(moves: &MoveList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match moves.iter().find(|x| x.from == (cur)) {
            None => next,
            Some(x) => next.insert(x.to, mayblock.clone()),
        },
    }
}

fn has(blocks: &BlockBoard, point: Point) -> bool {
    match blocks.pick(point) {
        None => false,
        Some(_) => true,
    }
}

pub fn inspect(blocks: &BlockBoard) {
    blocks.inspect(|(_, cell)| match cell {
        None => print!("{:^10}", "-"),
        Some(x) => print!("{:^10}", x.kind),
    });
}

pub fn synthesize(
    pros: &Vec<Builder>,
) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |next, (point, mayblock)| {
        pros.iter()
            .fold(next, |acc, cur| cur(acc, (point, mayblock)))
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Move {
    pub from: Point,
    pub to: Point,
}

impl Move {
    pub fn of(from: Point, to: Point) -> Move {
        Move { from, to }
    }
}
