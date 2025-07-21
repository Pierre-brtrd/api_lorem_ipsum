use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenerationUnit {
    Words,
    Sentences,
    Paragraphs,
}

impl GenerationUnit {
    pub fn to_display_name(&self) -> &'static str {
        match self {
            Self::Words => "mots",
            Self::Sentences => "phrases",
            Self::Paragraphs => "paragraphes",
        }
    }

    pub fn to_api_name(&self) -> &'static str {
        match self {
            Self::Words => "words",
            Self::Sentences => "sentences",
            Self::Paragraphs => "paragraphs",
        }
    }
}

impl FromStr for GenerationUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "words" => Ok(Self::Words),
            "sentences" => Ok(Self::Sentences),
            "paragraphs" => Ok(Self::Paragraphs),
            _ => Err(format!("Invalid generation unit: {s}")),
        }
    }
}
