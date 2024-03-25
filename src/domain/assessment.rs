use serde::{Deserialize, Serialize};

use super::base::{AssessmentResult, Score};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assessment {
    pub score: Option<Score>,
    pub progress: f32,
    pub result: AssessmentResult,
}
