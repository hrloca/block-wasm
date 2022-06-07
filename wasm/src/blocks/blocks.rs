use super::*;

pub type Cell = Option<Block>;
pub type Blocks = Board<Cell>;
pub type Builder = fn(Blocks, (Point, &Option<Block>)) -> Blocks;
pub type Processor = fn(&Blocks) -> Blocks;
pub type MoveList = Vec<Move>;
pub type PointList = Vec<Point>;

pub fn delete(blocks: &Blocks, points: &Vec<Point>) -> Blocks {
    blocks.update(del(points))
}

pub fn move_to(blocks: &Blocks, moves: &Vec<Move>) -> Blocks {
    blocks.update(mov(moves))
}

pub fn change(blocks: &Blocks, a: Point, b: Point) -> Blocks {
    blocks.update(cp(&vec![Move::of(a, b), Move::of(b, a)]))
}

// -------------------------------------

fn del(points: &PointList) -> impl Fn(Blocks, (Point, &Option<Block>)) -> Blocks + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match points.iter().find(|x| x.equals(cur)) {
            None => next,
            _ => next.insert(cur, None),
        },
    }
}

fn mov(moves: &MoveList) -> impl Fn(Blocks, (Point, &Option<Block>)) -> Blocks + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match moves.iter().find(|x| x.from == (cur)) {
            None => next,
            Some(x) if has(&next, x.to) => next,
            Some(x) => next.insert(x.to, *mayblock).insert(x.from, None),
        },
    }
}

fn cp(moves: &MoveList) -> impl Fn(Blocks, (Point, &Option<Block>)) -> Blocks + '_ {
    |next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match moves.iter().find(|x| x.from == (cur)) {
            None => next,
            Some(x) => next.insert(x.to, *mayblock),
        },
    }
}

fn has(blocks: &Blocks, point: Point) -> bool {
    match blocks.pick(point) {
        None => false,
        Some(_) => true,
    }
}

pub fn inspect(blocks: &Blocks) {
    blocks.inspect(|(_, cell)| match cell {
        None => print!("{:^10}", "-"),
        Some(x) => print!("{:^10}", x.kind),
    });
}

pub fn synthesize(pros: &Vec<Builder>) -> impl Fn(Blocks, (Point, &Option<Block>)) -> Blocks + '_ {
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
