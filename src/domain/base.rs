use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::InvalidIdError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Id(Uuid);

impl Id {
    pub fn parse(id: Uuid) -> Result<Id, InvalidIdError> {
        if Uuid::is_nil(&id) || Uuid::is_max(&id) {
            Err(InvalidIdError)
        } else {
            Ok(Id(id))
        }
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}
