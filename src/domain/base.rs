use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::{InvalidEmailError, InvalidIdError, InvalidPhoneError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Id(pub Uuid);

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(middle_name) = &self.middle_name {
            write!(f, "{} {} {}", self.first_name, middle_name, self.last_name)
        } else {
            write!(f, "{} {}", self.first_name, self.last_name)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Email(pub String);

impl Email {
    pub fn parse(email: String) -> Result<Email, InvalidEmailError> {
        // Email validation based on regex
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if email.is_empty() || !re.is_match(&email) {
            Err(InvalidEmailError)
        } else {
            Ok(Email(email))
        }
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phone(String);

impl Phone {
    pub fn parse(phone: String) -> Result<Phone, InvalidPhoneError> {
        // Phone validation based on regex
        let re = regex::Regex::new(r"^\+?[\d\s]{3,}$").unwrap();
        if phone.is_empty() || !re.is_match(&phone) {
            Err(InvalidPhoneError)
        } else {
            Ok(Phone(phone))
        }
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub building_number: String,
    pub country: String,
    pub postal_code: String,
}

impl Address {
    /// **Temporary new implementation**
    fn new() -> Self {
        Address {
            street: "".to_string(),
            city: "".to_string(),
            building_number: "".to_string(),
            country: "".to_string(),
            postal_code: "".to_string(),
        }
    }
}

impl Default for Address {
    fn default() -> Self {
        Address::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    pub value: u32,
    pub max: u32,
    pub min: u32,
    pub passing_score: u32,
}

impl Score {
    pub fn new(value: u32, max: u32, min: u32, passing_score: u32) -> Self {
        Score {
            value,
            max,
            min,
            passing_score,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AssessmentResult {
    Fail,
    Pass,
}
