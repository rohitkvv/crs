use mongodb::{
    bson::{doc, DateTime, Uuid},
    options::ClientOptions,
    results::InsertOneResult,
    Client, Database,
};

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{domain::certificate::Certificate, helpers::SaveType};

pub const DB_NAME: &str = "crs";

pub struct CrsState {
    pub db: Option<Database>,
}

pub async fn init_db() -> Option<Database> {
    info!("Initializing CRS DB!");

    if let Ok(uri) = dotenvy::var("DB_CONN_STRING") {
        let mut client_opts = ClientOptions::parse(uri)
            .await
            .expect("Unable to create ClientOptions");
        client_opts.app_name = Some("CRS".to_string());
        match Client::with_options(client_opts) {
            Ok(client) => {
                let db = client.database(DB_NAME);
                return Some(db);
            }
            Err(err) => error!("Unable to initialize DB. {}", err.to_string()),
        }
    }

    error!("DB connection string is invalid!");
    None
}

pub async fn store_one(db: &Database, doc: &CertificateModel) -> Option<InsertOneResult> {
    let coll = db.collection::<CertificateModel>("certificates");
    match coll.insert_one(doc, None).await {
        Ok(insert_one_result) => Some(insert_one_result),
        Err(_) => None,
    }
}

pub async fn find_certificate_by_id(
    db: &Database,
    certificate_id: uuid::Uuid,
) -> Option<CertificateModel> {
    let coll = db.collection::<CertificateModel>("certificates");
    match coll
        .find_one(
            doc! {"certificate_id": Uuid::from_uuid_1(certificate_id)},
            None,
        )
        .await
    {
        Ok(find_one_result) => find_one_result,
        Err(_) => None,
    }
}

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
    pub id: Uuid,
    pub name: String,
    pub institution: String,
    pub start_date: DateTime,
    pub end_date: Option<DateTime>,
    pub status: String,
}

impl CertificateModel {
    pub fn from_domain(certificate: &Certificate, save_type: SaveType) -> CertificateModel {
        CertificateModel {
            certificate_id: Uuid::default(),
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
                        id: Uuid::from_uuid_1(accreditation.id),
                        name: accreditation.name.clone(),
                        institution: accreditation.institution.clone(),
                        start_date: DateTime::from_chrono(accreditation.start_date),
                        end_date: accreditation
                            .end_date
                            .map(DateTime::from_chrono),
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
