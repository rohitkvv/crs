use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assessment {
    pub score: u32,
    pub progress: f32,
}
