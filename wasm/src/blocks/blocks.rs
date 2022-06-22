use super::*;
use crate::uuid;
use rand::prelude::*;
use std::collections::HashMap;

pub type Cell = Option<Block>;
pub type BlockBoard = Board<Cell>;
pub type Builder = fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard;
pub type Processor = fn(&BlockBoard) -> BlockBoard;
pub type MoveList = Vec<Move>;
pub type PointList = Vec<Point>;
pub type BlockList = Vec<Block>;

pub const TOTAL_BLOCK_KIND: u8 = 4;
pub const BOARD_COL: u8 = 3; // 11
pub const BOARD_RAW: u8 = 3; // 6
pub const SHOULD_CONNECT_WITH_DELETE: usize = 3;

pub fn create() -> BlockBoard {
    Board::init(Size::of(BOARD_RAW.into(), BOARD_COL.into()), |_| {
        let mut rng = rand::thread_rng();
        let kind = (rng.gen::<f64>() * TOTAL_BLOCK_KIND as f64).ceil();
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

pub fn fall(blocks: &BlockBoard) -> BlockBoard {
    let (next, _) = fall_scanning(&blocks);
    next
}

pub fn fall_scanning(blocks: &BlockBoard) -> (BlockBoard, Vec<(Point, Point)>) {
    let mut next = blocks.clone();
    let mut fall_set: Vec<(Point, Point)> = vec![];
    println!("落ち判定 開始-------------");
    blocks.fold_rev(
        (&mut next, &mut fall_set),
        move |(next, fall_set), (point, _)| fall_scanner(fall_set, next, point, point),
    );
    println!("終了-------------");

    (next, fall_set)
}

fn fall_scanner<'a>(
    fall_set: &'a mut Vec<(Point, Point)>,
    blocks: &'a mut BlockBoard,
    from_point: Point,
    current_point: Point,
) -> (&'a mut BlockBoard, &'a mut Vec<(Point, Point)>) {
    let may_from_block = blocks.pick(from_point);
    let next = blocks.bottom(current_point);

    match may_from_block {
        Some(from_block) => match next {
            Some((next_point, may_next_block)) => match may_next_block {
                Some(_) => {
                    if from_point == current_point {
                        return (blocks, fall_set);
                    }
                    blocks.insert(current_point, Some(from_block.clone()));
                    blocks.insert(from_point, None);
                    let move_to = (from_point, current_point);
                    fall_set.push(move_to);
                    println!("[ 確定 ] {:?} が {:?}", from_point, current_point);
                    (blocks, fall_set)
                }
                None => {
                    println!("[ 下降 ] {:?} を 下({:?}) へ", from_point, next_point);
                    fall_scanner(fall_set, blocks, from_point, next_point)
                }
            },
            None => {
                if from_point == current_point {
                    return (blocks, fall_set);
                }
                blocks.insert(current_point, Some(from_block.clone()));
                blocks.insert(from_point, None);
                let move_to = (from_point, current_point);
                fall_set.push(move_to);
                println!(
                    "[ 確定 ] ボード下端に到達. {:?} が {:?}",
                    from_point, current_point
                );
                (blocks, fall_set)
            }
        },
        None => (blocks, fall_set),
    }
}

pub fn scanning(blocks: &BlockBoard) -> (Groups, GroupIds, Kinds) {
    let groups: HashMap<String, Vec<(String, Point)>> = HashMap::new();
    let group_ids: HashMap<String, String> = HashMap::new();
    let kinds: HashMap<String, u8> = HashMap::new();
    let mut scan_result = (groups, group_ids, kinds);
    blocks.fold(&mut scan_result, move |acc, (point, _)| {
        delete_scanner(acc, point, blocks)
    });

    scan_result
}

/*
 delete_points: [(x, y)];
 score: { kinds: delete_num };
*/

pub fn delete_points(gps: &Groups) -> Vec<Point> {
    let mut delete_points: Vec<Point> = vec![];
    for (_, blocks) in gps {
        match blocks.len() {
            x if x >= SHOULD_CONNECT_WITH_DELETE => {
                for bl in blocks.iter() {
                    delete_points.push(bl.1);
                }
            }
            _ => (),
        }
    }
    delete_points
}

type Groups = HashMap<String, Vec<(String, Point)>>;
type GroupIds = HashMap<String, String>;
type Kinds = HashMap<String, u8>;

/*
Groups {
    group_id: [(id, Point)],
}

GroupIds {
    id: group_id
}

Kinds {
    group_id: kind
}
*/

pub fn delete_scanner<'a>(
    store: &'a mut (Groups, GroupIds, Kinds),
    current: Point,
    blocks: &BlockBoard,
) -> &'a mut (Groups, GroupIds, Kinds) {
    let block = blocks.pick(current);
    let top = blocks.top(current);
    let bottom = blocks.bottom(current);
    let left = blocks.left(current);
    let right = blocks.right(current);

    let mut scan = |block: &Block, target: Option<(Point, &Option<Block>)>| {
        if let Some((target_point, may_target_block)) = target {
            if let Some(target_block) = may_target_block {
                // -
                if block.same_kinds(target_block) {
                    if let None = store.1.get(&target_block.id) {
                        let gid = match store.1.get(&block.id) {
                            None => uuid(),
                            Some(id) => id.clone(),
                        };

                        let data = (target_block.id.clone(), target_point);
                        match store.0.get_mut(&gid) {
                            Some(blocks) => {
                                blocks.push(data);
                            }
                            None => {
                                store.0.insert(gid.clone(), vec![data]);
                            }
                        };

                        let kind = target_block.kind;
                        store.2.insert(gid.clone(), kind);
                        store.1.insert(target_block.id.clone(), gid);
                        delete_scanner(store, target_point, blocks);
                    }
                }
                // -
            };
        }
    };

    if let Some(block) = block {
        scan(block, top);
        scan(block, bottom);
        scan(block, left);
        scan(block, right);
    }

    store
}

// -------------------------------------

fn del(block_list: &BlockList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |mut next, (cur, mayblock)| match mayblock {
        None => next,
        Some(block) => match block_list.iter().find(|b| b.equals(block)) {
            None => next,
            _ => {
                next.insert(cur, None);
                next
            }
        },
    }
}

fn bla(points: &PointList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |mut next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match points.iter().find(|x| x.equals(cur)) {
            None => next,
            _ => {
                next.insert(cur, None);
                next
            }
        },
    }
}

fn mov(moves: &MoveList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |mut next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match moves.iter().find(|x| x.from == (cur)) {
            None => next,
            Some(x) if has(&next, x.to) => next,
            Some(x) => {
                next.insert(x.to, mayblock.clone());
                next.insert(x.from, None);
                next
            }
        },
    }
}

fn cp(moves: &MoveList) -> impl Fn(BlockBoard, (Point, &Option<Block>)) -> BlockBoard + '_ {
    |mut next, (cur, mayblock)| match mayblock {
        None => next,
        Some(_) => match moves.iter().find(|x| x.from == (cur)) {
            None => next,
            Some(x) => {
                next.insert(x.to, mayblock.clone());
                next
            }
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
