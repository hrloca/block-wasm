#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn サイズが取得できる() {
        let board = Board::init(Size::of(3, 2), |_| 0);
        assert_eq!(board.size().width, 3);
        assert_eq!(board.size().height, 2);
    }

    #[test]
    fn ポイントの存在が判定できる() {
        let board = Board::init(Size::of(3, 3), |_| 0);
        println!("{:?}", board);
        assert!(board.has(Point::of(0, 0)));
        assert!(board.has(Point::of(2, 2)));
        assert!(!board.has(Point::of(2, 3)));
    }

    #[test]
    fn 要素を取得できる() {
        #[rustfmt::skip]
        let board = Board::from(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        assert_eq!(*board.pick(Point::of(0, 0)), 1);
        assert_eq!(*board.pick(Point::of(0, 1)), 4);
        assert_eq!(*board.pick(Point::of(0, 2)), 7);
        assert_eq!(*board.pick(Point::of(2, 2)), 9);
    }

    #[test]
    #[should_panic]
    fn 範囲外の要素の取得はパニック() {
        #[rustfmt::skip]
        Board::from(vec![
            vec![1, 2],
            vec![3, 4],
        ]).pick(Point::of(2, 2));
    }

    #[test]
    fn 要素を挿入できる() {
        #[rustfmt::skip]
        let mut board = Board::from(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        board.insert(Point::of(0, 0), 100);
        board.insert(Point::of(0, 1), 200);

        assert_eq!(*board.pick(Point::of(0, 0)), 100);
        assert_eq!(*board.pick(Point::of(0, 1)), 200);
    }
}
