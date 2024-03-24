use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::base::{Email, Id, Name, Phone};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: Id,
    pub name: Name,
    pub email: Email,
    pub phone: Option<Phone>,
}

impl Person {
    /// **Temporary default implementation**
    pub fn default() -> Self {
        Person {
            id: Id::parse(Uuid::new_v4()).unwrap(),
            name: Name {
                first_name: "".to_string(),
                middle_name: None,
                last_name: "".to_string(),
            },
            email: Email::parse("test@email.com".to_string()).unwrap(),
            phone: None,
        }
    }
}
