use std::collections::HashMap;

pub struct BlockColors {
    repos: HashMap<u8, String>,
}

impl BlockColors {
    pub fn create() -> BlockColors {
        BlockColors {
            repos: HashMap::from([
                (1, String::from("#ff80ab")),
                (2, String::from("#8c9eff")),
                (3, String::from("#b9f6ca")),
                (4, String::from("#ffff8d")),
                (5, String::from("#ffd180")),
            ]),
        }
    }

    pub fn get(&self, id: u8) -> &str {
        self.repos.get(&id).unwrap()
    }
}
