use api_lorem_ipsum::domain::errors::DomainError;
use api_lorem_ipsum::domain::value_objects::text_length_category::{
    HtmlComplexity, TextLengthCategory,
};

#[cfg(test)]
mod text_length_category_tests {
    use super::*;

    // Tests de création et validation
    #[test]
    fn should_create_all_category_variants() {
        let short = TextLengthCategory::Short;
        let medium = TextLengthCategory::Medium;
        let long = TextLengthCategory::Long;

        assert_eq!(short, TextLengthCategory::Short);
        assert_eq!(medium, TextLengthCategory::Medium);
        assert_eq!(long, TextLengthCategory::Long);
    }

    // Tests de parsing URL
    #[test]
    fn should_parse_valid_url_names() {
        let test_cases = [
            ("short", TextLengthCategory::Short),
            ("medium", TextLengthCategory::Medium),
            ("long", TextLengthCategory::Long),
        ];

        for (input, expected) in test_cases {
            let result = TextLengthCategory::from_url_name(input).unwrap();
            assert_eq!(result, expected, "Failed to parse '{input}'");
        }
    }

    #[test]
    fn should_parse_case_insensitive() {
        let test_cases = ["SHORT", "Medium", "LONG", "short", "medium", "long"];

        for case in test_cases {
            let result = TextLengthCategory::from_url_name(case);
            assert!(result.is_ok(), "Should parse case insensitive: '{case}'");
        }
    }

    #[test]
    fn should_parse_french_aliases() {
        let test_cases = [
            ("court", TextLengthCategory::Short),
            ("moyen", TextLengthCategory::Medium),
            ("s", TextLengthCategory::Short),
            ("m", TextLengthCategory::Medium),
            ("l", TextLengthCategory::Long),
        ];

        for (input, expected) in test_cases {
            let result = TextLengthCategory::from_url_name(input).unwrap();
            assert_eq!(result, expected, "Failed to parse French alias '{input}'");
        }
    }

    #[test]
    fn should_reject_invalid_url_names() {
        let invalid_names = ["invalid", "tiny", "huge", "extra", ""];

        for name in invalid_names {
            let result = TextLengthCategory::from_url_name(name);
            assert!(result.is_err(), "Should reject invalid name: '{name}'");
            let error = result.unwrap_err();
            assert!(matches!(
                error,
                DomainError::UnknownLengthCategory { category } if category == name
            ));
        }
    }

    // Tests de conversion vers URL
    #[test]
    fn should_convert_to_url_names() {
        let test_cases = [
            (TextLengthCategory::Short, "short"),
            (TextLengthCategory::Medium, "medium"),
            (TextLengthCategory::Long, "long"),
        ];

        for (category, expected) in test_cases {
            assert_eq!(category.to_url_name(), expected);
        }
    }

    // Tests de noms d'affichage
    #[test]
    fn should_provide_french_display_names() {
        let test_cases = [
            (TextLengthCategory::Short, "Court"),
            (TextLengthCategory::Medium, "Moyen"),
            (TextLengthCategory::Long, "Long"),
        ];

        for (category, expected) in test_cases {
            assert_eq!(category.display_name(), expected);
        }
    }

    // Tests des ranges de mots
    #[test]
    fn should_provide_word_ranges() {
        assert_eq!(TextLengthCategory::Short.word_range(), 50..=150);
        assert_eq!(TextLengthCategory::Medium.word_range(), 150..=300);
        assert_eq!(TextLengthCategory::Long.word_range(), 300..=500);
    }

    #[test]
    fn should_provide_non_overlapping_ranges() {
        let short_max = *TextLengthCategory::Short.word_range().end();
        let medium_min = *TextLengthCategory::Medium.word_range().start();
        let medium_max = *TextLengthCategory::Medium.word_range().end();
        let long_min = *TextLengthCategory::Long.word_range().start();

        // Vérifier que les ranges se chevauchent correctement pour une transition fluide
        assert_eq!(short_max, medium_min); // 150 = 150
        assert_eq!(medium_max, long_min); // 300 = 300
    }

    // Tests des ranges HTML
    #[test]
    fn should_adjust_word_range_for_html_complexity() {
        let category = TextLengthCategory::Medium; // 150..=300

        let simple = category.word_range_for_html(HtmlComplexity::Simple);
        let moderate = category.word_range_for_html(HtmlComplexity::Moderate);
        let complex = category.word_range_for_html(HtmlComplexity::Complex);

        assert_eq!(simple, 150..=300); // Pas de changement
        assert_eq!(moderate, 120..=240); // ~20% de réduction
        assert_eq!(complex, 90..=180); // ~40% de réduction

        // Vérifier que les ranges sont cohérents
        assert!(*moderate.start() > *complex.start());
        assert!(*moderate.end() > *complex.end());
    }

    // Tests des ranges de phrases
    #[test]
    fn should_provide_sentence_ranges() {
        assert_eq!(TextLengthCategory::Short.sentence_range(), 3..=8);
        assert_eq!(TextLengthCategory::Medium.sentence_range(), 8..=15);
        assert_eq!(TextLengthCategory::Long.sentence_range(), 15..=25);
    }

    // Tests des ranges de caractères
    #[test]
    fn should_provide_character_ranges() {
        let short_chars = TextLengthCategory::Short.character_range();
        let medium_chars = TextLengthCategory::Medium.character_range();
        let long_chars = TextLengthCategory::Long.character_range();

        // Estimation: 6 caractères par mot
        assert_eq!(short_chars, 300..=900); // 50*6..=150*6
        assert_eq!(medium_chars, 900..=1800); // 150*6..=300*6
        assert_eq!(long_chars, 1800..=3000); // 300*6..=500*6
    }

    // Tests de génération aléatoire
    #[test]
    fn should_generate_word_count_in_range() {
        for _ in 0..10 {
            // Test multiple fois pour la randomisation
            let short_count = TextLengthCategory::Short.generate_word_count();
            let medium_count = TextLengthCategory::Medium.generate_word_count();
            let long_count = TextLengthCategory::Long.generate_word_count();

            assert!(TextLengthCategory::Short
                .word_range()
                .contains(&short_count));
            assert!(TextLengthCategory::Medium
                .word_range()
                .contains(&medium_count));
            assert!(TextLengthCategory::Long.word_range().contains(&long_count));
        }
    }

    #[test]
    fn should_generate_word_count_for_html_complexity() {
        for _ in 0..5 {
            let category = TextLengthCategory::Medium;

            let simple_count = category.generate_word_count_for_html(HtmlComplexity::Simple);
            let moderate_count = category.generate_word_count_for_html(HtmlComplexity::Moderate);
            let complex_count = category.generate_word_count_for_html(HtmlComplexity::Complex);

            assert!(category
                .word_range_for_html(HtmlComplexity::Simple)
                .contains(&simple_count));
            assert!(category
                .word_range_for_html(HtmlComplexity::Moderate)
                .contains(&moderate_count));
            assert!(category
                .word_range_for_html(HtmlComplexity::Complex)
                .contains(&complex_count));
        }
    }

    #[test]
    fn should_generate_sentence_count_in_range() {
        for _ in 0..10 {
            let short_sentences = TextLengthCategory::Short.generate_sentence_count();
            let medium_sentences = TextLengthCategory::Medium.generate_sentence_count();
            let long_sentences = TextLengthCategory::Long.generate_sentence_count();

            assert!(TextLengthCategory::Short
                .sentence_range()
                .contains(&short_sentences));
            assert!(TextLengthCategory::Medium
                .sentence_range()
                .contains(&medium_sentences));
            assert!(TextLengthCategory::Long
                .sentence_range()
                .contains(&long_sentences));
        }
    }

    // Tests de compatibilité
    #[test]
    fn should_validate_word_count_compatibility() {
        // Dans les ranges
        assert!(TextLengthCategory::Short.is_compatible_with_word_count(100));
        assert!(TextLengthCategory::Medium.is_compatible_with_word_count(200));
        assert!(TextLengthCategory::Long.is_compatible_with_word_count(400));

        // Aux limites
        assert!(TextLengthCategory::Short.is_compatible_with_word_count(50));
        assert!(TextLengthCategory::Short.is_compatible_with_word_count(150));
        assert!(TextLengthCategory::Medium.is_compatible_with_word_count(150));
        assert!(TextLengthCategory::Medium.is_compatible_with_word_count(300));

        // Hors des ranges
        assert!(!TextLengthCategory::Short.is_compatible_with_word_count(200));
        assert!(!TextLengthCategory::Medium.is_compatible_with_word_count(50));
        assert!(!TextLengthCategory::Long.is_compatible_with_word_count(100));
    }

    // Tests de recommandation
    #[test]
    fn should_recommend_category_for_word_count() {
        assert_eq!(
            TextLengthCategory::recommend_for_word_count(100),
            Some(TextLengthCategory::Short)
        );
        assert_eq!(
            TextLengthCategory::recommend_for_word_count(200),
            Some(TextLengthCategory::Medium)
        );
        assert_eq!(
            TextLengthCategory::recommend_for_word_count(400),
            Some(TextLengthCategory::Long)
        );

        // Aux limites - devrait choisir la première catégorie compatible
        assert_eq!(
            TextLengthCategory::recommend_for_word_count(150),
            Some(TextLengthCategory::Short)
        );
        assert_eq!(
            TextLengthCategory::recommend_for_word_count(300),
            Some(TextLengthCategory::Medium)
        );

        // Hors range
        assert_eq!(TextLengthCategory::recommend_for_word_count(10), None);
        assert_eq!(TextLengthCategory::recommend_for_word_count(1000), None);
    }

    // Tests de validation métier
    #[test]
    fn should_validate_reasonable_paragraph_counts() {
        // Cas valides
        assert!(TextLengthCategory::Short
            .validate_paragraph_count(50)
            .is_ok());
        assert!(TextLengthCategory::Medium
            .validate_paragraph_count(25)
            .is_ok());
        assert!(TextLengthCategory::Long
            .validate_paragraph_count(10)
            .is_ok());

        // Cas limites valides
        assert!(TextLengthCategory::Short
            .validate_paragraph_count(100)
            .is_ok());
        assert!(TextLengthCategory::Medium
            .validate_paragraph_count(50)
            .is_ok());
        assert!(TextLengthCategory::Long
            .validate_paragraph_count(20)
            .is_ok());
    }

    #[test]
    fn should_reject_excessive_paragraph_counts() {
        // Trop de paragraphes
        assert!(TextLengthCategory::Short
            .validate_paragraph_count(101)
            .is_err());
        assert!(TextLengthCategory::Medium
            .validate_paragraph_count(51)
            .is_err());
        assert!(TextLengthCategory::Long
            .validate_paragraph_count(21)
            .is_err());

        // Zéro paragraphe
        assert!(TextLengthCategory::Short
            .validate_paragraph_count(0)
            .is_err());
        assert!(TextLengthCategory::Medium
            .validate_paragraph_count(0)
            .is_err());
        assert!(TextLengthCategory::Long
            .validate_paragraph_count(0)
            .is_err());
    }

    // Tests des variants
    #[test]
    fn should_list_all_variants() {
        let variants = TextLengthCategory::all_variants();

        assert_eq!(variants.len(), 3);
        assert!(variants.contains(&TextLengthCategory::Short));
        assert!(variants.contains(&TextLengthCategory::Medium));
        assert!(variants.contains(&TextLengthCategory::Long));
    }

    // Tests des traits
    #[test]
    fn should_implement_display() {
        assert_eq!(format!("{}", TextLengthCategory::Short), "Court");
        assert_eq!(format!("{}", TextLengthCategory::Medium), "Moyen");
        assert_eq!(format!("{}", TextLengthCategory::Long), "Long");
    }

    #[test]
    fn should_implement_debug() {
        let debug_string = format!("{:?}", TextLengthCategory::Medium);
        assert!(debug_string.contains("Medium"));
    }

    #[test]
    fn should_implement_copy_clone() {
        let original = TextLengthCategory::Long;
        let copied = original;
        let cloned = original; // Copy trait, pas besoin de .clone()

        assert_eq!(original, copied);
        assert_eq!(original, cloned);
    }

    #[test]
    fn should_implement_equality() {
        assert_eq!(TextLengthCategory::Short, TextLengthCategory::Short);
        assert_ne!(TextLengthCategory::Short, TextLengthCategory::Medium);
        assert_ne!(TextLengthCategory::Medium, TextLengthCategory::Long);
    }

    #[test]
    fn should_implement_ordering() {
        assert!(TextLengthCategory::Short < TextLengthCategory::Medium);
        assert!(TextLengthCategory::Medium < TextLengthCategory::Long);
        assert!(TextLengthCategory::Short < TextLengthCategory::Long);

        // Test de tri
        let mut categories = vec![
            TextLengthCategory::Long,
            TextLengthCategory::Short,
            TextLengthCategory::Medium,
        ];
        categories.sort();
        assert_eq!(
            categories,
            vec![
                TextLengthCategory::Short,
                TextLengthCategory::Medium,
                TextLengthCategory::Long
            ]
        );
    }

    #[test]
    fn should_implement_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(TextLengthCategory::Short);
        set.insert(TextLengthCategory::Medium);
        set.insert(TextLengthCategory::Long);
        set.insert(TextLengthCategory::Short); // Duplicate

        assert_eq!(set.len(), 3); // Pas de doublons
    }

    // Tests de sérialisation JSON
    #[test]
    fn should_serialize_to_json() {
        let short_json = serde_json::to_string(&TextLengthCategory::Short).unwrap();
        let medium_json = serde_json::to_string(&TextLengthCategory::Medium).unwrap();
        let long_json = serde_json::to_string(&TextLengthCategory::Long).unwrap();

        assert_eq!(short_json, "\"short\"");
        assert_eq!(medium_json, "\"medium\"");
        assert_eq!(long_json, "\"long\"");
    }

    #[test]
    fn should_deserialize_from_json() {
        let short: TextLengthCategory = serde_json::from_str("\"short\"").unwrap();
        let medium: TextLengthCategory = serde_json::from_str("\"medium\"").unwrap();
        let long: TextLengthCategory = serde_json::from_str("\"long\"").unwrap();

        assert_eq!(short, TextLengthCategory::Short);
        assert_eq!(medium, TextLengthCategory::Medium);
        assert_eq!(long, TextLengthCategory::Long);
    }

    #[test]
    fn should_handle_json_roundtrip() {
        let categories = [
            TextLengthCategory::Short,
            TextLengthCategory::Medium,
            TextLengthCategory::Long,
        ];

        for original in categories {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: TextLengthCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }
    }

    #[test]
    fn should_reject_invalid_json() {
        let invalid_jsons = ["\"invalid\"", "\"tiny\"", "\"huge\"", "null", "123"];

        for json in invalid_jsons {
            let result: Result<TextLengthCategory, _> = serde_json::from_str(json);
            assert!(result.is_err(), "Should reject invalid JSON: {json}");
        }
    }
}

#[cfg(test)]
mod html_complexity_tests {
    use super::*;

    #[test]
    fn should_create_html_complexity_variants() {
        let simple = HtmlComplexity::Simple;
        let moderate = HtmlComplexity::Moderate;
        let complex = HtmlComplexity::Complex;

        assert_eq!(simple, HtmlComplexity::Simple);
        assert_eq!(moderate, HtmlComplexity::Moderate);
        assert_eq!(complex, HtmlComplexity::Complex);
    }

    #[test]
    fn should_implement_debug() {
        let debug_string = format!("{:?}", HtmlComplexity::Moderate);
        assert!(debug_string.contains("Moderate"));
    }

    #[test]
    fn should_implement_equality() {
        assert_eq!(HtmlComplexity::Simple, HtmlComplexity::Simple);
        assert_ne!(HtmlComplexity::Simple, HtmlComplexity::Complex);
    }

    #[test]
    fn should_implement_copy_clone() {
        let original = HtmlComplexity::Complex;
        let copied = original;
        let cloned = original; // Copy trait, pas besoin de .clone()

        assert_eq!(original, copied);
        assert_eq!(original, cloned);
    }
}
