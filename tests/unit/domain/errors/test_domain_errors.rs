use api_lorem_ipsum::domain::errors::DomainError;

#[cfg(test)]
mod domain_error_tests {
    use super::*;

    #[test]
    fn test_invalid_text_length_error() {
        let error = DomainError::invalid_text_length(1500, 1, 1000);
        assert_eq!(
            error.to_string(),
            "Longueur de texte invalide: 1500 (doit être entre 1 et 1000)"
        );
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_unknown_category_error() {
        let error = DomainError::unknown_category("super-long");
        assert_eq!(
            error.to_string(),
            "Catégorie de longueur inconnue: 'super-long'"
        );
        assert!(!error.is_validation_error());
        assert!(error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_invalid_html_tag_error() {
        let error = DomainError::invalid_html_tag("invalid-tag");
        assert_eq!(error.to_string(), "Tag HTML invalide: 'invalid-tag'");
        assert!(!error.is_validation_error());
        assert!(error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_empty_html_tags_error() {
        let error = DomainError::EmptyHtmlTags;
        assert_eq!(error.to_string(), "Liste de tags HTML vide");
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_incompatible_html_tags_error() {
        let error = DomainError::IncompatibleHtmlTags {
            tag1: "ul".to_string(),
            tag2: "ol".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Tags HTML incompatibles: 'ul' et 'ol' ne peuvent pas être utilisés ensemble"
        );
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_unknown_format_error() {
        let error = DomainError::unknown_format("xml");
        assert_eq!(error.to_string(), "Format de texte inconnu: 'xml'");
        assert!(!error.is_validation_error());
        assert!(error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_unknown_unit_error() {
        let error = DomainError::unknown_unit("lines");
        assert_eq!(error.to_string(), "Unité de génération inconnue: 'lines'");
        assert!(!error.is_validation_error());
        assert!(error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_invalid_count_error() {
        let error = DomainError::invalid_count(0, 1, 1000);
        assert_eq!(
            error.to_string(),
            "Nombre d'éléments invalide: 0 (doit être entre 1 et 1000)"
        );
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_incompatible_unit_with_html_error() {
        let error = DomainError::IncompatibleUnitWithHtml {
            unit: "mots".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Combinaison invalide: mots avec tags HTML"
        );
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_excessive_count_error() {
        let error = DomainError::ExcessiveCount {
            unit: "paragraphes".to_string(),
            count: 150,
            max: 100,
            category: "short".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Trop de paragraphes demandés: 150 (maximum 100 pour la catégorie 'short')"
        );
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_too_many_words_error() {
        let error = DomainError::TooManyWords {
            count: 15000,
            max: 10000,
        };
        assert_eq!(
            error.to_string(),
            "Trop de mots demandés: 15000 (maximum 10000)"
        );
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_too_many_sentences_error() {
        let error = DomainError::TooManySentences {
            count: 600,
            max: 500,
        };
        assert_eq!(
            error.to_string(),
            "Trop de phrases demandées: 600 (maximum 500)"
        );
        assert!(error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_malformed_url_error() {
        let error = DomainError::malformed_url("/api/invalid");
        assert_eq!(
            error.to_string(),
            "URL malformée: '/api/invalid' (format attendu: /api/count/category/tags...)"
        );
        assert!(!error.is_validation_error());
        assert!(error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_missing_html_tags_error() {
        let error = DomainError::MissingHtmlTags;
        assert_eq!(error.to_string(), "Au moins un tag HTML doit être spécifié");
        assert!(!error.is_validation_error());
        assert!(error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_invalid_url_number_error() {
        let error = DomainError::InvalidUrlNumber {
            value: "abc".to_string(),
        };
        assert_eq!(error.to_string(), "Nombre invalide dans l'URL: 'abc'");
        assert!(!error.is_validation_error());
        assert!(error.is_parsing_error());
        assert!(!error.is_cache_error());
    }

    #[test]
    fn test_cache_size_exceeded_error() {
        let error = DomainError::CacheSizeExceeded {
            size: 6_000_000,
            max: 5_000_000,
        };
        assert_eq!(
            error.to_string(),
            "Taille de cache excessive: 6000000 bytes (max: 5000000 bytes)"
        );
        assert!(!error.is_validation_error());
        assert!(!error.is_parsing_error());
        assert!(error.is_cache_error());
    }

    #[test]
    fn test_error_conversion_to_string() {
        let error = DomainError::unknown_category("test");
        let error_string: String = error.into();
        assert_eq!(error_string, "Catégorie de longueur inconnue: 'test'");
    }

    #[test]
    fn test_error_classification_completeness() {
        // Test que chaque variant est classifié dans au moins une catégorie
        let validation_errors = vec![
            DomainError::InvalidTextLength {
                value: 1500,
                min: 1,
                max: 1000,
            },
            DomainError::InvalidElementCount {
                count: 0,
                min: 1,
                max: 1000,
            },
            DomainError::EmptyHtmlTags,
            DomainError::IncompatibleHtmlTags {
                tag1: "ul".to_string(),
                tag2: "ol".to_string(),
            },
            DomainError::IncompatibleUnitWithHtml {
                unit: "mots".to_string(),
            },
            DomainError::ExcessiveCount {
                unit: "paragraphes".to_string(),
                count: 150,
                max: 100,
                category: "short".to_string(),
            },
            DomainError::TooManyWords {
                count: 15000,
                max: 10000,
            },
            DomainError::TooManySentences {
                count: 600,
                max: 500,
            },
            DomainError::ValidationFailed {
                details: "test".to_string(),
            },
        ];

        for error in validation_errors {
            assert!(
                error.is_validation_error(),
                "Error should be validation: {error:?}"
            );
        }

        let parsing_errors = vec![
            DomainError::UnknownLengthCategory {
                category: "test".to_string(),
            },
            DomainError::InvalidHtmlTag {
                tag: "test".to_string(),
            },
            DomainError::UnknownTextFormat {
                format: "test".to_string(),
            },
            DomainError::UnknownGenerationUnit {
                unit: "test".to_string(),
            },
            DomainError::MalformedUrl {
                url: "test".to_string(),
            },
            DomainError::InvalidUrlParameter {
                parameter: "test".to_string(),
                value: "test".to_string(),
            },
            DomainError::InvalidUrlNumber {
                value: "test".to_string(),
            },
            DomainError::MissingHtmlTags,
            DomainError::JsonDeserializationError {
                message: "test".to_string(),
            },
            DomainError::InvalidJsonValue {
                field: "test".to_string(),
                value: "test".to_string(),
            },
        ];

        for error in parsing_errors {
            assert!(
                error.is_parsing_error(),
                "Error should be parsing: {error:?}"
            );
        }

        let cache_errors = vec![
            DomainError::InvalidCacheConfig {
                reason: "test".to_string(),
            },
            DomainError::CacheSizeExceeded {
                size: 1000,
                max: 500,
            },
        ];

        for error in cache_errors {
            assert!(error.is_cache_error(), "Error should be cache: {error:?}");
        }
    }

    #[test]
    fn test_helper_constructors() {
        // Test que les helper methods créent les bonnes erreurs
        let text_length_error = DomainError::invalid_text_length(1500, 1, 1000);
        if let DomainError::InvalidTextLength { value, min, max } = text_length_error {
            assert_eq!(value, 1500);
            assert_eq!(min, 1);
            assert_eq!(max, 1000);
        } else {
            panic!("Wrong error type");
        }

        let category_error = DomainError::unknown_category("test");
        if let DomainError::UnknownLengthCategory { category } = category_error {
            assert_eq!(category, "test");
        } else {
            panic!("Wrong error type");
        }

        let url_error = DomainError::malformed_url("/invalid");
        if let DomainError::MalformedUrl { url } = url_error {
            assert_eq!(url, "/invalid");
        } else {
            panic!("Wrong error type");
        }
    }
}
