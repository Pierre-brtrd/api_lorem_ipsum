/// Test rapide pour valider l'implémentation DomainError
/// Les autres tests seront corrigés dans une prochaine itération
#[cfg(test)]
mod domain_error_integration_test {
    use api_lorem_ipsum::domain::entities::GenerationRequest;
    use api_lorem_ipsum::domain::errors::DomainError;
    use api_lorem_ipsum::domain::value_objects::{
        GenerationUnit, HtmlTags, TextFormat, TextLengthCategory,
    };

    #[test]
    fn test_domain_error_integration() {
        // Test que les Value Objects utilisent bien DomainError
        let invalid_category = TextLengthCategory::from_url_name("invalid");
        assert!(invalid_category.is_err());
        if let Err(error) = invalid_category {
            assert!(matches!(error, DomainError::UnknownLengthCategory { .. }));
        }

        // Test que GenerationRequest utilise bien DomainError
        let invalid_request = GenerationRequest::new(
            0, // Count invalide
            TextLengthCategory::Short,
            HtmlTags::from_url_parts(&["headers"]).unwrap(),
            TextFormat::HTML,
            GenerationUnit::Paragraphs,
        );
        assert!(invalid_request.is_err());
        if let Err(error) = invalid_request {
            assert!(matches!(
                error,
                DomainError::InvalidElementCount { count: 0, .. }
            ));
        }

        // Test conversion String
        let error = DomainError::unknown_category("test");
        let error_string: String = error.into();
        assert!(error_string.contains("Catégorie de longueur inconnue"));

        // Test classification
        let parsing_error = DomainError::unknown_format("xml");
        assert!(parsing_error.is_parsing_error());
        assert!(!parsing_error.is_validation_error());
        assert!(!parsing_error.is_cache_error());

        let validation_error = DomainError::invalid_count(0, 1, 100);
        assert!(validation_error.is_validation_error());
        assert!(!validation_error.is_parsing_error());
        assert!(!validation_error.is_cache_error());
    }

    #[test]
    fn test_error_helpers() {
        let text_error = DomainError::invalid_text_length(1500, 1, 1000);
        assert_eq!(
            text_error.to_string(),
            "Longueur de texte invalide: 1500 (doit être entre 1 et 1000)"
        );

        let url_error = DomainError::malformed_url("/invalid");
        assert_eq!(
            url_error.to_string(),
            "URL malformée: '/invalid' (format attendu: /api/count/category/tags...)"
        );

        let unit_error = DomainError::unknown_unit("lines");
        assert_eq!(
            unit_error.to_string(),
            "Unité de génération inconnue: 'lines'"
        );
    }
}
