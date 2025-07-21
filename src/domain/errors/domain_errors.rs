use thiserror::Error;

/// Erreurs du domaine métier de l'API Lorem Ipsum
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    // === Value Objects Errors ===
    #[error("Longueur de texte invalide: {value} (doit être entre {min} et {max})")]
    InvalidTextLength { value: u32, min: u32, max: u32 },

    #[error("Catégorie de longueur inconnue: '{category}'")]
    UnknownLengthCategory { category: String },

    #[error("Tag HTML invalide: '{tag}'")]
    InvalidHtmlTag { tag: String },

    #[error("Liste de tags HTML vide")]
    EmptyHtmlTags,

    #[error("Tags HTML incompatibles: '{tag1}' et '{tag2}' ne peuvent pas être utilisés ensemble")]
    IncompatibleHtmlTags { tag1: String, tag2: String },

    #[error("Format de texte inconnu: '{format}'")]
    UnknownTextFormat { format: String },

    #[error("Unité de génération inconnue: '{unit}'")]
    UnknownGenerationUnit { unit: String },

    // === Entity Errors ===
    #[error("Nombre d'éléments invalide: {count} (doit être entre {min} et {max})")]
    InvalidElementCount { count: u32, min: u32, max: u32 },

    #[error("Combinaison invalide: {unit} avec tags HTML")]
    IncompatibleUnitWithHtml { unit: String },

    #[error("Trop de {unit} demandés: {count} (maximum {max} pour la catégorie '{category}')")]
    ExcessiveCount {
        unit: String,
        count: u32,
        max: u32,
        category: String,
    },

    #[error("Trop de mots demandés: {count} (maximum {max})")]
    TooManyWords { count: u32, max: u32 },

    #[error("Trop de phrases demandées: {count} (maximum {max})")]
    TooManySentences { count: u32, max: u32 },

    // === URL Parsing Errors ===
    #[error("URL malformée: '{url}' (format attendu: /api/count/category/tags...)")]
    MalformedUrl { url: String },

    #[error("Paramètre manquant dans l'URL: {parameter}")]
    MissingUrlParameter { parameter: String },

    #[error("Paramètre URL invalide: {parameter}='{value}'")]
    InvalidUrlParameter { parameter: String, value: String },

    #[error("Au moins un tag HTML doit être spécifié")]
    MissingHtmlTags,

    #[error("Nombre invalide dans l'URL: '{value}'")]
    InvalidUrlNumber { value: String },

    // === Cache Errors ===
    #[error("Configuration de cache invalide: {reason}")]
    InvalidCacheConfig { reason: String },

    #[error("Taille de cache excessive: {size} bytes (max: {max} bytes)")]
    CacheSizeExceeded { size: usize, max: usize },

    // === Business Rules Errors ===
    #[error("Règle métier violée: {rule}")]
    BusinessRuleViolation { rule: String },

    #[error("Validation métier échouée: {details}")]
    ValidationFailed { details: String },

    // === JSON/Serialization Errors ===
    #[error("Erreur de désérialisation JSON: {message}")]
    JsonDeserializationError { message: String },

    #[error("Valeur JSON invalide pour {field}: {value}")]
    InvalidJsonValue { field: String, value: String },
}

impl DomainError {
    /// Crée une erreur de longueur de texte invalide
    pub fn invalid_text_length(value: u32, min: u32, max: u32) -> Self {
        Self::InvalidTextLength { value, min, max }
    }

    /// Crée une erreur de catégorie inconnue
    pub fn unknown_category(category: impl Into<String>) -> Self {
        Self::UnknownLengthCategory {
            category: category.into(),
        }
    }

    /// Crée une erreur de tag HTML invalide
    pub fn invalid_html_tag(tag: impl Into<String>) -> Self {
        Self::InvalidHtmlTag { tag: tag.into() }
    }

    /// Crée une erreur de format inconnu
    pub fn unknown_format(format: impl Into<String>) -> Self {
        Self::UnknownTextFormat {
            format: format.into(),
        }
    }

    /// Crée une erreur d'unité inconnue
    pub fn unknown_unit(unit: impl Into<String>) -> Self {
        Self::UnknownGenerationUnit { unit: unit.into() }
    }

    /// Crée une erreur de count invalide
    pub fn invalid_count(count: u32, min: u32, max: u32) -> Self {
        Self::InvalidElementCount { count, min, max }
    }

    /// Crée une erreur d'URL malformée
    pub fn malformed_url(url: impl Into<String>) -> Self {
        Self::MalformedUrl { url: url.into() }
    }

    /// Crée une erreur de paramètre manquant
    pub fn missing_parameter(parameter: impl Into<String>) -> Self {
        Self::MissingUrlParameter {
            parameter: parameter.into(),
        }
    }

    /// Vérifie si l'erreur est liée à la validation
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidTextLength { .. }
                | Self::InvalidElementCount { .. }
                | Self::EmptyHtmlTags
                | Self::IncompatibleHtmlTags { .. }
                | Self::IncompatibleUnitWithHtml { .. }
                | Self::ExcessiveCount { .. }
                | Self::TooManyWords { .. }
                | Self::TooManySentences { .. }
                | Self::ValidationFailed { .. }
        )
    }

    /// Vérifie si l'erreur est liée au parsing
    pub fn is_parsing_error(&self) -> bool {
        matches!(
            self,
            Self::UnknownLengthCategory { .. }
                | Self::InvalidHtmlTag { .. }
                | Self::UnknownTextFormat { .. }
                | Self::UnknownGenerationUnit { .. }
                | Self::MalformedUrl { .. }
                | Self::MissingUrlParameter { .. }
                | Self::InvalidUrlParameter { .. }
                | Self::MissingHtmlTags
                | Self::InvalidUrlNumber { .. }
                | Self::JsonDeserializationError { .. }
                | Self::InvalidJsonValue { .. }
        )
    }

    /// Vérifie si l'erreur est liée au cache
    pub fn is_cache_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidCacheConfig { .. } | Self::CacheSizeExceeded { .. }
        )
    }
}

/// Type alias pour les résultats du domaine
pub type DomainResult<T> = Result<T, DomainError>;

// Implémentation des conversions automatiques
impl From<DomainError> for String {
    fn from(error: DomainError) -> Self {
        error.to_string()
    }
}
