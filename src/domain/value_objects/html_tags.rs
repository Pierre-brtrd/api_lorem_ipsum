use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HtmlTags {
    tags: Vec<HtmlTag>,
}

impl HtmlTags {
    pub fn new(tags: Vec<HtmlTag>) -> DomainResult<Self> {
        if tags.is_empty() {
            return Err(DomainError::EmptyHtmlTags);
        }

        let mut unique_tags = tags;
        unique_tags.sort();
        unique_tags.dedup();

        Ok(Self { tags: unique_tags })
    }

    pub fn from_url_parts(parts: &[&str]) -> DomainResult<Self> {
        if parts.is_empty() {
            return Err(DomainError::MissingHtmlTags);
        }

        let mut tags = Vec::new();
        for part in parts {
            let tag = HtmlTag::from_url_name(part)?;
            tags.push(tag);
        }

        Self::new(tags)
    }

    pub fn single(tag: HtmlTag) -> DomainResult<Self> {
        Self::new(vec![tag])
    }

    pub fn tags(&self) -> &[HtmlTag] {
        &self.tags
    }

    pub fn contains(&self, tag: &HtmlTag) -> bool {
        self.tags.contains(tag)
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tags.len()
    }

    pub fn combine(&self, other: &Self) -> DomainResult<Self> {
        let mut combined = self.tags.clone();
        combined.extend(other.tags.iter().cloned());
        Self::new(combined)
    }

    pub fn validate_compatibility(&self) -> DomainResult<()> {
        for tag in &self.tags {
            for other_tag in &self.tags {
                if tag != other_tag && !tag.is_compatible_with(other_tag) {
                    return Err(DomainError::IncompatibleHtmlTags {
                        tag1: tag.to_url_name().to_string(),
                        tag2: other_tag.to_url_name().to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn count_block_elements(&self) -> usize {
        self.tags
            .iter()
            .filter(|tag| tag.is_block_element())
            .count()
    }

    pub fn count_inline_elements(&self) -> usize {
        self.tags
            .iter()
            .filter(|tag| tag.is_inline_element())
            .count()
    }

    // Pour le debug et logging
    pub fn to_url_string(&self) -> String {
        self.tags
            .iter()
            .map(|tag| tag.to_url_name())
            .collect::<Vec<_>>()
            .join("/")
    }
}

impl fmt::Display for HtmlTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.to_url_string())
    }
}

impl From<HtmlTag> for HtmlTags {
    fn from(tag: HtmlTag) -> Self {
        Self::single(tag).expect("Un seul tag HTML devrait toujours être valide")
    }
}

impl TryFrom<Vec<&str>> for HtmlTags {
    type Error = DomainError;

    fn try_from(parts: Vec<&str>) -> Result<Self, Self::Error> {
        Self::from_url_parts(&parts)
    }
}

impl Serialize for HtmlTags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let tag_names: Vec<&str> = self.tags.iter().map(|tag| tag.to_url_name()).collect();
        tag_names.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HtmlTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tag_names: Vec<String> = Vec::deserialize(deserializer)?;
        let tags: Result<Vec<HtmlTag>, _> = tag_names
            .iter()
            .map(|name| HtmlTag::from_url_name(name))
            .collect();

        let tags = tags.map_err(serde::de::Error::custom)?;
        HtmlTags::new(tags).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HtmlTag {
    Header,
    Link,
    Ul,
    Ol,
    Dl,
    Pre,
    Code,
    Blockquote,
    Div,
}

impl HtmlTag {
    pub fn from_url_name(name: &str) -> DomainResult<Self> {
        match name.to_lowercase().as_str() {
            "headers" | "header" => Ok(Self::Header),
            "link" | "links" => Ok(Self::Link),
            "ul" | "unordered" => Ok(Self::Ul),
            "ol" | "ordered" => Ok(Self::Ol),
            "dl" | "definition" => Ok(Self::Dl),
            "pre" | "preformatted" => Ok(Self::Pre),
            "code" => Ok(Self::Code),
            "blockquote" | "quote" => Ok(Self::Blockquote),
            "div" => Ok(Self::Div),
            _ => Err(DomainError::invalid_html_tag(name)),
        }
    }

    pub fn to_url_name(&self) -> &'static str {
        match self {
            Self::Header => "headers",
            Self::Link => "link",
            Self::Ul => "ul",
            Self::Ol => "ol",
            Self::Dl => "dl",
            Self::Pre => "pre",
            Self::Code => "code",
            Self::Blockquote => "blockquote",
            Self::Div => "div",
        }
    }

    pub fn is_block_element(&self) -> bool {
        matches!(
            self,
            Self::Div | Self::Blockquote | Self::Pre | Self::Ul | Self::Ol | Self::Dl
        )
    }

    pub fn is_inline_element(&self) -> bool {
        matches!(self, Self::Link | Self::Code)
    }

    pub fn requires_content(&self) -> bool {
        !matches!(self, Self::Div) // Div peut être vide
    }

    pub fn is_compatible_with(&self, other: &HtmlTag) -> bool {
        // Règles métier : certains tags ne vont pas ensemble
        match (self, other) {
            (Self::Ul, Self::Ol) | (Self::Ol, Self::Ul) => false, // Pas de listes mixtes
            _ => true,
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::Header,
            Self::Link,
            Self::Ul,
            Self::Ol,
            Self::Dl,
            Self::Pre,
            Self::Code,
            Self::Blockquote,
            Self::Div,
        ]
    }
}

impl fmt::Display for HtmlTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_url_name())
    }
}
