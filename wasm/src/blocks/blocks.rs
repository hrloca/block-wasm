use super::*;

pub type Cell = Option<Block>;
pub type BlockBoard = Board<Cell>;
pub type Builder = fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard;
pub type Processor = fn(&BlockBoard) -> BlockBoard;
pub type MoveList = Vec<Move>;
pub type PointList = Vec<Point>;
pub type BlockList = Vec<Block>;

pub fn delete(blocks: &BlockBoard, points: &PointList) -> BlockBoard {
    blocks.update(bla(points))
}

pub fn move_to(blocks: &BlockBoard, moves: &Vec<Move>) -> BlockBoard {
    blocks.update(mov(moves))
}

pub fn change(blocks: &BlockBoard, a: Point, b: Point) -> BlockBoard {
    blocks.update(cp(&vec![Move::of(a, b), Move::of(b, a)]))
}

pub fn blank(blocks: &BlockBoard, points: &Vec<Point>) -> BlockBoard {
    blocks.update(bla(points))
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
            Some(x) => next.insert(x.to, *mayblock).insert(x.from, None),
        },
    }
}

fn cp(moves: &MoveList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match moves.iter().find(|x| x.from == (cur)) {
            None => next,
            Some(x) => next.insert(x.to, *mayblock),
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
