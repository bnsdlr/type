use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {

}

impl Settings {
    pub fn load() -> Self {
        Self {}
    }
}
