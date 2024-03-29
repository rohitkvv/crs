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

#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidIdError;

impl Error for InvalidIdError {
    fn description(&self) -> &str {
        "failed to parse uuid"
    }
}

impl std::fmt::Display for InvalidIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided UUID is not valid".fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidEmailError;

impl Error for InvalidEmailError {
    fn description(&self) -> &str {
        "failed to parse email"
    }
}

impl std::fmt::Display for InvalidEmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided email is not valid".fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidPhoneError;

impl Error for InvalidPhoneError {
    fn description(&self) -> &str {
        "failed to parse phone number"
    }
}

impl std::fmt::Display for InvalidPhoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided phone number is not valid".fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateParseError;

impl Error for CertificateParseError {
    fn description(&self) -> &str {
        "failed to parse certificate"
    }
}

impl std::fmt::Display for CertificateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "unable to parse into a valid certificate".fmt(f)
    }
}
