use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize, Serializer};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct TextLength {
    #[validate(range(
        min = 1,
        max = 1000,
        message = "La longueur du texte doit être entre 1 à 1000"
    ))]
    value: u32,
}

impl TextLength {
    pub fn new(value: u32) -> DomainResult<Self> {
        if value == 0 {
            return Err(DomainError::invalid_text_length(value, 1, 1000));
        }
        if value > 1000 {
            return Err(DomainError::invalid_text_length(value, 1, 1000));
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

// Sérialisation personnalisée pour sérialiser comme un simple nombre
impl Serialize for TextLength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.value)
    }
}

impl<'de> Deserialize<'de> for TextLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        TextLength::new(value).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
