use serde::Deserialize;
use uuid::Uuid;

use super::certificate_metadata_dto::CertificateMetadataDto;

#[derive(Deserialize)]
pub struct CertificateDto {
    pub user_id: Uuid,
    pub account_id: u32,
    pub product_id: u32,
    pub metadata: CertificateMetadataDto,
}
