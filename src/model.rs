use mongodb::bson::{doc, DateTime, Uuid};
use serde::{Deserialize, Serialize};

use crate::{domain::certificate::Certificate, helpers::SaveType};

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateModel {
    pub certificate_id: Uuid,
    pub user_id: Uuid,
    pub account_id: u32,
    pub product_id: u32,
    pub metadata: CertificateMetadataModel,
    pub created_date: DateTime,
    pub updated_date: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CertificateMetadataModel {
    pub score: u32,
    pub progress: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccreditationModel {
    pub name: String,
    pub institution: String,
    pub start_date: DateTime,
    pub end_date: Option<DateTime>,
    pub status: String,
}

impl CertificateModel {
    pub fn from_domain(certificate: &Certificate, save_type: SaveType) -> CertificateModel {
        CertificateModel {
            certificate_id: Uuid::from_uuid_1(certificate.id.as_uuid()),
            user_id: Uuid::from_uuid_1(certificate.user_id.as_uuid()),
            account_id: certificate.account_id,
            product_id: certificate.product_id,
            metadata: CertificateMetadataModel {
                score: certificate.assessment.score,
                progress: certificate.assessment.progress,
            },
            created_date: DateTime::from_chrono(certificate.created_date),
            updated_date: match save_type {
                SaveType::Insert => None,
                SaveType::Update => Some(DateTime::now()),
            },
        }
    }
}
