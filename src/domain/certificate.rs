use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{dto::certificate_dto::CertificateDto, model::CertificateModel};

use super::{
    assessment::Assessment, base::Id, error::CertificateParseError, organization::Organization,
    person::Person, validity::Validity,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Certificate {
    pub id: Id,
    pub user_id: Id,
    // The person who received the certificate
    pub recipient: Person,
    pub account_id: u32,
    pub product_id: u32,
    pub name: String,
    pub description: String,
    // An organization that issued the certificate
    pub authority: Organization,
    pub validity: Option<Validity>,
    pub assessment: Assessment,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

fn respond_with_json<T: Serialize>(obj: T) -> HttpResponse<BoxBody> {
    match serde_json::to_string(&obj) {
        Ok(body) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body),
        Err(_) => HttpResponse::BadRequest().body("Unable to serialize the response"),
    }
}

/// Implement the responder for the Certificate
impl Responder for Certificate {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        respond_with_json(self)
    }
}

pub struct Certificates(pub Vec<Certificate>);

impl Responder for Certificates {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        respond_with_json(self.0)
    }
}

impl TryFrom<CertificateModel> for Certificate {
    type Error = CertificateParseError;

    fn try_from(certificate: CertificateModel) -> Result<Self, Self::Error> {
        Ok(Certificate {
            id: match Id::parse(certificate.certificate_id.into()) {
                Ok(id) => id,
                Err(invalid_id_error) => panic!("{}", invalid_id_error),
            },
            user_id: match Id::parse(certificate.user_id.into()) {
                Ok(id) => id,
                Err(invalid_id_error) => panic!("{}", invalid_id_error),
            },
            recipient: Person::default(),
            account_id: certificate.account_id,
            product_id: certificate.product_id,
            name: "".to_string(),
            description: "".to_string(),
            authority: Organization::default(),
            validity: None,
            assessment: Assessment {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
            },
            created_date: certificate.created_date.into(),
            updated_date: certificate.updated_date.map(|dt| dt.into()),
        })
    }
}

impl TryFrom<CertificateDto> for Certificate {
    type Error = CertificateParseError;

    fn try_from(certificate: CertificateDto) -> Result<Self, Self::Error> {
        Ok(Certificate {
            id: Id::parse(Uuid::new_v4()).unwrap(),
            user_id: match Id::parse(certificate.user_id) {
                Ok(id) => id,
                Err(invalid_id_error) => panic!("{}", invalid_id_error),
            },
            recipient: Person::default(),
            account_id: certificate.account_id,
            product_id: certificate.product_id,
            name: "".to_string(),
            description: "".to_string(),
            authority: Organization::default(),
            validity: None,
            assessment: Assessment {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
            },
            created_date: Utc::now(),
            updated_date: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mongodb::bson::{DateTime, Uuid as BsonUuid};
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    use crate::{
        domain::certificate::Certificate,
        dto::{certificate_dto::CertificateDto, certificate_metadata_dto::CertificateMetadataDto},
        model::{CertificateMetadataModel, CertificateModel},
    };

    #[test]
    fn parse_certificate_dto_should_succeed() {
        let certificate_dto = CertificateDto {
            user_id: Uuid::new_v4(),
            account_id: 1,
            product_id: 1,
            metadata: CertificateMetadataDto {
                score: 0,
                progress: 0.5,
                acquired_date: None,
                accreditation: None,
            },
        };
        let user_id = certificate_dto.user_id;
        let certificate = Certificate::try_from(certificate_dto).unwrap();

        assert_eq!(certificate.user_id.as_uuid(), user_id);
        assert!(certificate.created_date.timestamp() > 0);
    }

    #[test]
    fn parse_certificate_model_should_succeed() {
        let certificate_model = CertificateModel {
            certificate_id: BsonUuid::default(),
            user_id: BsonUuid::default(),
            account_id: 1,
            product_id: 1,
            metadata: CertificateMetadataModel {
                score: 0,
                progress: 0.5,
            },
            created_date: DateTime::from_chrono(Utc::now()),
            updated_date: None,
        };
        let certificate_id = certificate_model.certificate_id;
        let certificate = Certificate::try_from(certificate_model).unwrap();

        assert_eq!(
            BsonUuid::from_uuid_1(certificate.id.as_uuid()),
            certificate_id
        );
    }
}
