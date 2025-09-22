use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {

}

impl Stats {
    pub fn load() -> Self {
        Self {}
    }
}
