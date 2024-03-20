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
    pub acquired_date: Option<DateTime>,
    pub accreditation: Option<AccreditationModel>,
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
            certificate_id: Uuid::from_uuid_1(certificate.id),
            user_id: Uuid::from_uuid_1(certificate.user_id),
            account_id: certificate.account_id,
            product_id: certificate.product_id,
            metadata: CertificateMetadataModel {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
                acquired_date: certificate
                    .metadata
                    .acquired_date
                    .map(DateTime::from_chrono),
                accreditation: certificate
                    .metadata
                    .accreditation
                    .as_ref()
                    .map(|accreditation| AccreditationModel {
                        name: accreditation.name.clone(),
                        institution: accreditation.institution.clone(),
                        start_date: DateTime::from_chrono(accreditation.start_date),
                        end_date: accreditation.end_date.map(DateTime::from_chrono),
                        status: accreditation.status.to_string(),
                    }),
            },
            created_date: DateTime::from_chrono(certificate.created_date),
            updated_date: match save_type {
                SaveType::Insert => None,
                SaveType::Update => Some(DateTime::now()),
            },
        }
    }
}
