#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn should_grouping() {
        connect(to_blocks(vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ]));

        assert!(true);
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

    fn to_blocks(v: Vec<Vec<u8>>) -> Blocks {
        v.iter()
            .map(|row| {
                row.iter()
                    .map(|num| Block {
                        kind: *num,
                        group: None,
                    })
                    .collect()
            })
            .collect()
    }
}
