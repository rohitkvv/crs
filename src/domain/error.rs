use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccreditationStatusError;

impl Error for AccreditationStatusError {
    fn description(&self) -> &str {
        "failed to parse accreditation status"
    }
}

impl std::fmt::Display for AccreditationStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string was invalid, allowed values are `pending`, `active`, `expired`, or `revoked`".fmt(f)
    }
}
