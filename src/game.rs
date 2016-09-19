use std::path::{PathBuf};

pub struct Game {
    assets: PathBuf,
}

impl Game {
    pub fn new(assets: PathBuf) -> Game {
        Game {
            assets: assets,
        }
    }
}
