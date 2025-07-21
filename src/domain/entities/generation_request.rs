use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::value_objects::{
    GenerationUnit, HtmlComplexity, HtmlTags, TextFormat, TextLengthCategory,
};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::Duration;

/// Entity représentant une demande de génération de contenu Lorem Ipsum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// Nombre d'éléments à générer (paragraphes, phrases, mots)
    count: u32,
    /// Catégorie de longueur (Short, Medium, Long)
    length_category: TextLengthCategory,
    /// Tags HTML à intégrer
    html_tags: HtmlTags,
    /// Format de sortie (Plain, HTML, Markdown)
    format: TextFormat,
    /// Unité de génération (Words, Sentences, Paragraphs)
    unit: GenerationUnit,

    // Configuration du cache
    /// Seed pour génération déterministe (reproductible)
    generation_seed: Option<u64>,
    /// Flag pour activer/désactiver le cache
    cache_enabled: bool,
    /// TTL custom pour cette request
    cache_ttl: Option<Duration>,
}

/// Stratégie de cache pour les requests de génération
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheStrategy {
    /// Cache normal avec TTL par défaut (1 heure)
    Default,
    /// Cache long pour les requests fréquentes
    LongTerm { ttl_hours: u32 },
    /// Cache en mémoire seulement (pas persistant)
    MemoryOnly,
    /// Pas de cache pour cette request
    NoCache,
    /// Cache avec refresh automatique
    AutoRefresh { interval_minutes: u32 },
}

/// Analyse de cachabilité d'une request
#[derive(Debug, Clone, PartialEq)]
pub struct CacheAnalysis {
    pub is_cacheable: bool,
    pub estimated_size_bytes: usize,
    pub strategy: CacheStrategy,
    pub hit_probability: f32,
    pub cache_key: String,
}

impl GenerationRequest {
    /// Constructeur principal avec cache activé par défaut
    pub fn new(
        count: u32,
        length_category: TextLengthCategory,
        html_tags: HtmlTags,
        format: TextFormat,
        unit: GenerationUnit,
    ) -> DomainResult<Self> {
        Self::with_cache_config(count, length_category, html_tags, format, unit, true, None)
    }

    /// Constructeur avec contrôle complet du cache
    pub fn with_cache_config(
        count: u32,
        length_category: TextLengthCategory,
        html_tags: HtmlTags,
        format: TextFormat,
        unit: GenerationUnit,
        cache_enabled: bool,
        custom_seed: Option<u64>,
    ) -> DomainResult<Self> {
        // Validation des règles métier
        Self::validate_business_rules(count, &length_category, &html_tags, &unit)?;

        Ok(Self {
            count,
            length_category,
            html_tags,
            format,
            unit,
            generation_seed: custom_seed,
            cache_enabled,
            cache_ttl: None,
        })
    }

    /// Constructeur pour génération déterministe (même seed = même résultat)
    pub fn deterministic(
        count: u32,
        length_category: TextLengthCategory,
        html_tags: HtmlTags,
        format: TextFormat,
        unit: GenerationUnit,
        seed: u64,
    ) -> DomainResult<Self> {
        Self::with_cache_config(
            count,
            length_category,
            html_tags,
            format,
            unit,
            true,
            Some(seed),
        )
    }

    /// Parse depuis une URL comme `/api/10/long/headers/link`
    pub fn from_url_path(path: &str) -> DomainResult<Self> {
        let parts: Vec<&str> = path.trim_start_matches("/api/").split('/').collect();

        if parts.len() < 2 {
            return Err(DomainError::malformed_url(path));
        }

        // Parse count
        let count = parts[0]
            .parse::<u32>()
            .map_err(|_| DomainError::InvalidUrlNumber {
                value: parts[0].to_string(),
            })?;

        // Parse length category
        let length_category = TextLengthCategory::from_url_name(parts[1])?;

        // Parse HTML tags (optionnel)
        let html_tags = if parts.len() > 2 {
            HtmlTags::from_url_parts(&parts[2..])?
        } else {
            return Err(DomainError::MissingHtmlTags);
        };

        // Par défaut: HTML et Paragraphs
        Self::new(
            count,
            length_category,
            html_tags,
            TextFormat::HTML,
            GenerationUnit::Paragraphs,
        )
    }

    /// Parse depuis URL avec paramètres de cache
    pub fn from_url_with_cache(
        path: &str,
        query_params: &HashMap<String, String>,
    ) -> DomainResult<Self> {
        let mut request = Self::from_url_path(path)?;

        // Configuration du cache depuis query params
        if let Some(cache_hint) = query_params.get("cache") {
            request.cache_enabled = cache_hint != "false" && cache_hint != "0";
        }

        if let Some(seed_str) = query_params.get("seed") {
            request.generation_seed = seed_str.parse().ok();
        }

        if let Some(ttl_str) = query_params.get("ttl") {
            if let Ok(seconds) = ttl_str.parse::<u64>() {
                request.cache_ttl = Some(Duration::from_secs(seconds));
            }
        }

        // Format override
        if let Some(format_str) = query_params.get("format") {
            request.format = TextFormat::from_str(format_str)?;
        }

        // Unit override
        if let Some(unit_str) = query_params.get("unit") {
            request.unit = GenerationUnit::from_str(unit_str)?;
        }

        Ok(request)
    }

    // Getters
    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn length_category(&self) -> TextLengthCategory {
        self.length_category
    }

    pub fn html_tags(&self) -> &HtmlTags {
        &self.html_tags
    }

    pub fn format(&self) -> TextFormat {
        self.format
    }

    pub fn unit(&self) -> GenerationUnit {
        self.unit
    }

    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }

    pub fn generation_seed(&self) -> Option<u64> {
        self.generation_seed
    }

    // Cache methods
    /// Génère une clé de cache unique et stable
    pub fn cache_key(&self) -> String {
        format!(
            "gen:{}:{}:{}:{}:{}:{}",
            self.count,
            self.length_category.to_url_name(),
            self.format.to_api_name(),
            self.unit.to_api_name(),
            self.html_tags.to_url_string(),
            self.generation_seed.unwrap_or(0)
        )
    }

    /// Version hash compacte de la clé de cache
    pub fn cache_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Génère un seed stable pour reproduction
    pub fn generate_stable_seed(&self) -> u64 {
        self.generation_seed.unwrap_or_else(|| {
            let mut hasher = DefaultHasher::new();
            self.count.hash(&mut hasher);
            self.length_category.hash(&mut hasher);
            self.html_tags.hash(&mut hasher);
            self.format.hash(&mut hasher);
            self.unit.hash(&mut hasher);
            hasher.finish()
        })
    }

    /// Configure la génération pour être reproductible
    pub fn with_deterministic_generation(&mut self) {
        if self.generation_seed.is_none() {
            self.generation_seed = Some(self.generate_stable_seed());
        }
    }

    /// Détermine la stratégie de cache appropriée
    pub fn cache_strategy(&self) -> CacheStrategy {
        if !self.cache_enabled {
            return CacheStrategy::NoCache;
        }

        match (self.count, self.length_category, self.html_tags.len()) {
            // Petites requests simples = cache long
            (1..=5, TextLengthCategory::Short, 0..=2) => CacheStrategy::LongTerm { ttl_hours: 24 },

            // Requests moyennes = cache normal
            (1..=20, TextLengthCategory::Medium, 0..=5) => CacheStrategy::Default,

            // Grosses requests = cache mémoire seulement
            (50.., _, _) => CacheStrategy::MemoryOnly,

            // Requests très complexes = auto-refresh
            (_, _, 6..) => CacheStrategy::AutoRefresh {
                interval_minutes: 60,
            },

            // Autres = cache normal
            _ => CacheStrategy::Default,
        }
    }

    /// Vérifie si la request peut utiliser le cache
    pub fn is_cacheable(&self) -> bool {
        self.cache_enabled
            && self.count <= 100 // Limite business
            && self.html_tags.len() <= 15 // Pas trop complexe
            && self.estimated_cache_size_bytes() < 5_000_000 // 5MB max
    }

    /// Estime la taille en cache (en bytes)
    pub fn estimated_cache_size_bytes(&self) -> usize {
        let base_size = self.count as usize;
        let word_estimate = *self.length_category.word_range().end() as usize;
        let html_multiplier = if self.html_tags.is_empty() { 1 } else { 2 };
        let format_multiplier = match self.format {
            TextFormat::PlainText => 1,
            TextFormat::HTML => 3, // Balises HTML ajoutent du volume
            TextFormat::Markdown => 2,
        };

        // Estimation: 6 chars/mot + overhead HTML/Markdown
        base_size * word_estimate * 6 * html_multiplier * format_multiplier
    }

    /// Détermine si la request vaut le cache
    pub fn should_cache(&self) -> bool {
        self.is_cacheable()
            && self.estimated_cache_size_bytes() < 1_000_000 // 1MB max par défaut
            && self.count >= 2 // Pas de cache pour 1 élément
    }

    /// Estime la probabilité de cache hit basée sur les patterns
    pub fn estimate_hit_probability(&self) -> f32 {
        match (self.count, self.length_category, self.html_tags.len()) {
            // Patterns très communs
            (1..=5, TextLengthCategory::Short, 0..=1) => 0.9,
            (1..=10, TextLengthCategory::Medium, 0..=2) => 0.7,
            (1..=3, TextLengthCategory::Long, 0..=1) => 0.5,

            // Patterns moyens
            (6..=20, TextLengthCategory::Short, 0..=3) => 0.4,
            (11..=25, TextLengthCategory::Medium, 0..=4) => 0.3,

            // Patterns rares
            (21.., _, _) => 0.1,
            (_, _, 5..) => 0.1,

            // Autres
            _ => 0.2,
        }
    }

    /// Analyse complète de cachabilité
    pub fn cache_analysis(&self) -> CacheAnalysis {
        CacheAnalysis {
            is_cacheable: self.is_cacheable(),
            estimated_size_bytes: self.estimated_cache_size_bytes(),
            strategy: self.cache_strategy(),
            hit_probability: self.estimate_hit_probability(),
            cache_key: self.cache_key(),
        }
    }

    /// Détermine la complexité HTML pour ajustement des métriques
    pub fn html_complexity(&self) -> HtmlComplexity {
        match self.html_tags.len() {
            0..=2 => HtmlComplexity::Simple,
            3..=5 => HtmlComplexity::Moderate,
            _ => HtmlComplexity::Complex,
        }
    }

    /// Génère le nombre de mots ajusté selon le contexte HTML
    pub fn generate_word_count(&self) -> u32 {
        self.length_category
            .generate_word_count_for_html(self.html_complexity())
    }

    /// Validation des règles métier
    fn validate_business_rules(
        count: u32,
        length_category: &TextLengthCategory,
        html_tags: &HtmlTags,
        unit: &GenerationUnit,
    ) -> DomainResult<()> {
        // Validation du count
        if count == 0 {
            return Err(DomainError::invalid_count(count, 1, 1000));
        }

        if count > 1000 {
            return Err(DomainError::invalid_count(count, 1, 1000));
        }

        // Validation de la compatibilité length_category + count
        length_category.validate_paragraph_count(count)?;

        // Validation de la compatibilité HTML tags
        html_tags.validate_compatibility()?;

        // Validation spécifique selon l'unité
        match unit {
            GenerationUnit::Words => {
                if count > 10000 {
                    return Err(DomainError::TooManyWords { count, max: 10000 });
                }
            }
            GenerationUnit::Sentences => {
                if count > 500 {
                    return Err(DomainError::TooManySentences { count, max: 500 });
                }
            }
            GenerationUnit::Paragraphs => {
                // Déjà validé par length_category.validate_paragraph_count
            }
        }

        // Validation de la compatibilité HTML tags + unité
        if matches!(unit, GenerationUnit::Words) && !html_tags.is_empty() {
            return Err(DomainError::IncompatibleUnitWithHtml {
                unit: "mots".to_string(),
            });
        }

        Ok(())
    }
}

impl std::fmt::Display for GenerationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} ({})",
            self.count,
            self.unit.to_display_name(),
            self.length_category.display_name(),
            self.format.to_display_name()
        )
    }
}

impl Default for CacheStrategy {
    fn default() -> Self {
        Self::Default
    }
}
