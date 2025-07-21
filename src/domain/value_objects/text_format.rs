use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextFormat {
    PlainText,
    Markdown,
    HTML,
}

impl TextFormat {
    pub fn to_display_name(&self) -> &'static str {
        match self {
            Self::PlainText => "Texte brut",
            Self::Markdown => "Markdown",
            Self::HTML => "HTML",
        }
    }

    pub fn to_api_name(&self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::Markdown => "markdown",
            Self::HTML => "html",
        }
    }
}

impl FromStr for TextFormat {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plain_text" => Ok(Self::PlainText),
            "markdown" => Ok(Self::Markdown),
            "html" => Ok(Self::HTML),
            _ => Err(DomainError::unknown_format(s)),
        }
    }
}
