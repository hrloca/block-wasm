#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn should_do_next_handling() {
        let blocks = to_blocks(vec![
            vec![1, 1, 2, 2],
            vec![2, 2, 1, 1],
            vec![1, 3, 1, 1],
            vec![1, 3, 1, 1],
        ]);

        let try_next = next_pos(&blocks);

        assert_eq!(try_next((0, 0), Dir::Top), None);
        assert_eq!(try_next((0, 0), Dir::Bottom), Some((1, 0)));
        assert_eq!(try_next((0, 0), Dir::Right), Some((0, 1)));
        assert_eq!(try_next((0, 0), Dir::Left), None);
    }

    #[test]
    fn should_connecting() {
        let conected = connect(&to_blocks(vec![
            vec![1, 1, 2, 2],
            vec![2, 2, 1, 1],
            vec![1, 3, 1, 1],
            vec![1, 3, 1, 1],
        ]));

        assert!(conected[0][0].connect.right);
        assert!(conected[1][0].connect.right);
        assert!(conected[1][1].connect.left);
        assert!(conected[2][2].connect.top);
        assert!(conected[2][2].connect.top);
        assert!(conected[2][2].connect.bottom);
        assert!(conected[2][2].connect.right);
    }

    #[test]
    fn should_next() {
        let maynext = maybe_next(2, 5);
        assert_eq!(maynext, Some(3));
        let maynone = maybe_next(4, 5);
        assert_eq!(maynone, None);
    }

    #[test]
    fn should_none_over_max() {
        let maynone = maybe_next(5, 5);
        assert_eq!(maynone, None);
        let maynone = maybe_next(8, 5);
        assert_eq!(maynone, None);
    }

    #[test]
    fn should_prev() {
        let maynone = maybe_prev(5);
        assert_eq!(maynone, Some(4));
        let maynone = maybe_prev(1);
        assert_eq!(maynone, Some(0));
    }

    #[test]
    fn should_none_less_0() {
        let maynone = maybe_prev(0);
        assert_eq!(maynone, None);
    }

    fn to_blocks(v: Vec<Vec<u8>>) -> BlocksBody {
        v.into_iter()
            .map(|row| row.into_iter().map(Block::create).collect())
            .collect()
    }
}
