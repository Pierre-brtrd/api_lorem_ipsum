use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextLengthCategory {
    Short,  // ~50-150 mots par paragraphe
    Medium, // ~150-300 mots
    Long,   // ~300-500 mots
}

impl TextLengthCategory {
    /// Parse une catégorie depuis le nom d'URL (case insensitive)
    pub fn from_url_name(name: &str) -> DomainResult<Self> {
        match name.to_lowercase().as_str() {
            "short" | "court" | "s" => Ok(Self::Short),
            "medium" | "moyen" | "m" => Ok(Self::Medium),
            "long" | "l" => Ok(Self::Long),
            _ => Err(DomainError::unknown_category(name)),
        }
    }

    /// Convertit vers le nom d'URL standard
    pub fn to_url_name(&self) -> &'static str {
        match self {
            Self::Short => "short",
            Self::Medium => "medium",
            Self::Long => "long",
        }
    }

    /// Nom d'affichage en français
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Short => "Court",
            Self::Medium => "Moyen",
            Self::Long => "Long",
        }
    }

    /// Range de mots pour cette catégorie (contexte par défaut)
    pub fn word_range(&self) -> RangeInclusive<u32> {
        match self {
            Self::Short => 50..=150,
            Self::Medium => 150..=300,
            Self::Long => 300..=500,
        }
    }

    /// Range ajusté selon la complexité HTML
    pub fn word_range_for_html(&self, html_complexity: HtmlComplexity) -> RangeInclusive<u32> {
        let base_range = self.word_range();
        let (start, end) = (*base_range.start(), *base_range.end());

        match html_complexity {
            HtmlComplexity::Simple => base_range,
            HtmlComplexity::Moderate => {
                // Réduction de ~20% pour compenser les balises
                let new_start = (start as f32 * 0.8) as u32;
                let new_end = (end as f32 * 0.8) as u32;
                new_start..=new_end
            }
            HtmlComplexity::Complex => {
                // Réduction de ~40% pour les structures complexes
                let new_start = (start as f32 * 0.6) as u32;
                let new_end = (end as f32 * 0.6) as u32;
                new_start..=new_end
            }
        }
    }

    /// Range de phrases estimé
    pub fn sentence_range(&self) -> RangeInclusive<u32> {
        match self {
            Self::Short => 3..=8,
            Self::Medium => 8..=15,
            Self::Long => 15..=25,
        }
    }

    /// Range de caractères estimé (avec espaces)
    pub fn character_range(&self) -> RangeInclusive<u32> {
        let word_range = self.word_range();
        // Estimation: 6 caractères par mot en moyenne (français)
        let start = *word_range.start() * 6;
        let end = *word_range.end() * 6;
        start..=end
    }

    /// Génère un nombre de mots aléatoire dans le range approprié
    pub fn generate_word_count(&self) -> u32 {
        let range = self.word_range();
        fastrand::u32(range)
    }

    /// Génère un nombre de mots aléatoire avec contexte HTML
    pub fn generate_word_count_for_html(&self, html_complexity: HtmlComplexity) -> u32 {
        let range = self.word_range_for_html(html_complexity);
        fastrand::u32(range)
    }

    /// Génère un nombre de phrases aléatoire
    pub fn generate_sentence_count(&self) -> u32 {
        let range = self.sentence_range();
        fastrand::u32(range)
    }

    /// Valide la compatibilité avec un nombre de mots donné
    pub fn is_compatible_with_word_count(&self, word_count: u32) -> bool {
        self.word_range().contains(&word_count)
    }

    /// Recommande une catégorie basée sur un nombre de mots
    pub fn recommend_for_word_count(word_count: u32) -> Option<Self> {
        if Self::Short.is_compatible_with_word_count(word_count) {
            Some(Self::Short)
        } else if Self::Medium.is_compatible_with_word_count(word_count) {
            Some(Self::Medium)
        } else if Self::Long.is_compatible_with_word_count(word_count) {
            Some(Self::Long)
        } else {
            None // Hors des ranges supportés
        }
    }

    /// Tous les variants disponibles
    pub fn all_variants() -> Vec<Self> {
        vec![Self::Short, Self::Medium, Self::Long]
    }

    /// Valide que la catégorie peut générer le nombre de paragraphes demandé
    pub fn validate_paragraph_count(&self, paragraph_count: u32) -> DomainResult<()> {
        if paragraph_count == 0 {
            return Err(DomainError::invalid_count(paragraph_count, 1, u32::MAX));
        }

        // Validation métier: éviter les combinaisons non-sens
        match (self, paragraph_count) {
            (Self::Long, count) if count > 20 => Err(DomainError::ExcessiveCount {
                unit: "paragraphes".to_string(),
                count,
                max: 20,
                category: "long".to_string(),
            }),
            (Self::Medium, count) if count > 50 => Err(DomainError::ExcessiveCount {
                unit: "paragraphes".to_string(),
                count,
                max: 50,
                category: "medium".to_string(),
            }),
            (Self::Short, count) if count > 100 => Err(DomainError::ExcessiveCount {
                unit: "paragraphes".to_string(),
                count,
                max: 100,
                category: "short".to_string(),
            }),
            _ => Ok(()),
        }
    }
}

/// Complexité HTML pour ajustement des ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlComplexity {
    Simple,   // Pas de balises ou balises simples (p, br)
    Moderate, // Quelques balises (h1-h6, a, strong, em)
    Complex,  // Structures complexes (ul, ol, dl, tables, etc.)
}

impl std::fmt::Display for TextLengthCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
