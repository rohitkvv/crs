use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Uuid},
    options::ClientOptions,
    results::InsertOneResult,
    Client, Database,
};

use log::{error, info};

use crate::model::CertificateModel;

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
    coll.find_one(
        doc! {"certificate_id": Uuid::from_uuid_1(certificate_id)},
        None,
    )
    .await
    .unwrap_or_default()
}

pub async fn find_certificates_by_user_id(
    db: &Database,
    user_id: uuid::Uuid,
) -> Option<Vec<CertificateModel>> {
    let coll = db.collection::<CertificateModel>("certificates");
    match coll
        .find(doc! {"user_id": Uuid::from_uuid_1(user_id)}, None)
        .await
    {
        Ok(find_result) => {
            let mut certificates = Vec::new();
            let mut cursor = find_result;
            while let Some(result) = cursor.try_next().await.unwrap() {
                certificates.push(result);
            }
            Some(certificates)
        }
        Err(_) => None,
    }
}
