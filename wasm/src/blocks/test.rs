#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn playground() {
        // #[rustfmt::skip]
    }

    #[test]
    fn ブロックを削除できる() {
        #[rustfmt::skip]
        let board = Board::from(vec![
            vec![b(1), b(2), b(3)],
            vec![b(4), b(5), b(6)],
            vec![b(7), b(8), b(9)],
        ]);

        let deleted = delete(
            &board,
            &vec![
                board.pick(Point::of(0, 0)).unwrap(),
                board.pick(Point::of(1, 0)).unwrap(),
                board.pick(Point::of(2, 0)).unwrap(),
            ],
        );

        inspect(&deleted);
    }

    #[test]
    fn ブロックを移動できる() {
        #[rustfmt::skip]
        let board = Board::from(vec![
            vec![b(1), b(2), b(3)],
            vec![b(4), b(5), b(6)],
            vec![None, None, None],
        ]);

        let moved = move_to(
            &board,
            &vec![
                Move::of(Point::of(0, 0), Point::of(2, 2)),
                Move::of(Point::of(1, 0), Point::of(1, 2)),
                Move::of(Point::of(2, 0), Point::of(0, 2)),
            ],
        );

        assert_eq!(moved.pick(Point::of(2, 2)).unwrap().kind, 1);
    }

    #[test]
    fn ブロックを交換できる() {
        #[rustfmt::skip]
        let board = Board::from(vec![
            vec![b(1), b(2), b(3)],
            vec![b(4), b(5), b(6)],
            vec![b(7), b(8), b(9)],
        ]);

        let changed = change(&board, Point::of(0, 0), Point::of(2, 2));

        assert_eq!(changed.pick(Point::of(0, 0)).unwrap().kind, 9);
        assert_eq!(changed.pick(Point::of(2, 2)).unwrap().kind, 1);
    }

    fn b(kind: u8) -> Option<Block> {
        Some(Block::of(kind))
    }
}
