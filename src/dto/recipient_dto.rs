use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RecipientDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}

impl RecipientDto {
    /// Validates the recipient
    /// # Returns
    /// **true** if the recipient is valid, otherwise **false**
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::recipient_dto::RecipientDto;
    /// use uuid::Uuid;
    ///
    /// let recipient = RecipientDto {
    ///     id: Uuid::new_v4(),
    ///     first_name: "firstName".to_string(),
    ///     last_name: "lastName".to_string(),
    ///     email: "test@email.com".to_string(),
    ///     phone: "12345678".to_string()
    /// };
    /// assert_eq!(recipient.is_valid(), true);
    /// ```
    /// ---
    ///
    /// ```
    ///
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::recipient_dto::RecipientDto;
    /// use uuid::Uuid;
    ///
    /// let recipient = RecipientDto {
    ///     id: Uuid::nil(),
    ///     first_name: "firstName".to_string(),
    ///     last_name: "lastName".to_string(),
    ///     email: "test@email.com".to_string(),
    ///     phone: "12345678".to_string()
    /// };
    /// assert_eq!(recipient.is_valid(), false);
    /// ```
    pub fn is_valid(&self) -> bool {
        self.id != Uuid::nil()
            && !self.first_name.is_empty()
            && !self.last_name.is_empty()
            && !self.email.is_empty()
    }
}