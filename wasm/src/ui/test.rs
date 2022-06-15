use crate::uuid;

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn playground() {
        // #[rustfmt::skip]
        let id = uuid();
        dbg!(id);
    }
}
