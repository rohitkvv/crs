use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::base::{Address, Email, Id, Phone};

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    pub id: Id,
    pub name: String,
    pub email: Email,
    pub phone: Phone,
    pub address: Address,
}

impl Organization {
    /// **Temporary default implementation**
    pub fn default() -> Self {
        Organization {
            id: Id::parse(Uuid::new_v4()).unwrap(),
            name: "".to_string(),
            email: Email::parse("test@email.com".to_string()).unwrap(),
            phone: Phone::parse("987654321".to_string()).unwrap(),
            address: Address::default(),
        }
    }
}
