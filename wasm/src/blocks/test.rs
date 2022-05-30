#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    #[should_panic]
    fn shoud_panic() {
        let pos = Pos::from(0, 0);
        pos.sub(1, 1);
    }

    #[test]
    fn shoud_pos() {
        let pos = Pos::from(5, 5);
        let add = pos.add(1, 1);
        assert_eq!(add.x, 6);
        assert_eq!(add.y, 6);
        let sub = pos.sub(2, 5);
        assert_eq!(sub.x, 3);
        assert_eq!(sub.y, 0);
    }

    #[test]
    fn should_do_table() {
        let tabel = Table::create(vec![
            vec![1, 1, 2, 2],
            vec![2, 2, 1, 1],
            vec![1, 3, 1, 1],
            vec![1, 3, 1, 1],
        ]);

        let startpoint_has = tabel.has(Pos::from(0, 0));
        assert_eq!(startpoint_has(Dir::Top), false);
        assert_eq!(startpoint_has(Dir::Bottom), true);
        assert_eq!(startpoint_has(Dir::Left), false);
        assert_eq!(startpoint_has(Dir::Right), true);

        let endpoint_has = tabel.has(Pos::from(3, 3));
        assert_eq!(endpoint_has(Dir::Top), true);
        assert_eq!(endpoint_has(Dir::Bottom), false);
        assert_eq!(endpoint_has(Dir::Left), true);
        assert_eq!(endpoint_has(Dir::Right), false);
    }

    // fn to_block_table(v: Vec<Vec<u8>>) -> Table<Block> {
    //     Table::create(
    //         v.into_iter()
    //             .map(|row| row.into_iter().map(Block::create).collect())
    //             .collect(),
    //     )
    // }
}
