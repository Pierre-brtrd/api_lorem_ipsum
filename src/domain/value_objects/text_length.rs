use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Validate)]
pub struct TextLength {
    #[validate(range(
        min = 1,
        max = 1000,
        message = "La longueur du texte doit être entre 1 à 1000"
    ))]
    value: u32,
}

impl TextLength {
    pub fn new(value: u32) -> Result<Self, String> {
        let text_length = Self { value };

        match text_length.validate() {
            Ok(_) => Ok(text_length),
            Err(errors) => {
                let message = errors
                    .field_errors()
                    .get("value")
                    .and_then(|errs| errs.first())
                    .and_then(|err| err.message.as_ref())
                    .map(|msg| msg.to_string())
                    .unwrap_or_else(|| "Longueur de texte invalide".to_string());
                Err(message)
            }
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl<'de> Deserialize<'de> for TextLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        TextLength::new(value).map_err(serde::de::Error::custom)
    }
}
