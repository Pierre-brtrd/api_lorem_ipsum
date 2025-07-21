use api_lorem_ipsum::domain::value_objects::generation_unit::GenerationUnit;
use serde_json;

#[cfg(test)]
mod generation_unit_tests {
    use super::*;

    // Tests de création et méthodes de base
    #[test]
    fn should_create_words_unit() {
        let unit = GenerationUnit::Words;
        assert_eq!(unit.to_display_name(), "mots");
        assert_eq!(unit.to_api_name(), "words");
    }

    #[test]
    fn should_create_sentences_unit() {
        let unit = GenerationUnit::Sentences;
        assert_eq!(unit.to_display_name(), "phrases");
        assert_eq!(unit.to_api_name(), "sentences");
    }

    #[test]
    fn should_create_paragraphs_unit() {
        let unit = GenerationUnit::Paragraphs;
        assert_eq!(unit.to_display_name(), "paragraphes");
        assert_eq!(unit.to_api_name(), "paragraphs");
    }

    // Tests d'égalité et traits dérivés
    #[test]
    fn should_implement_equality() {
        let unit1 = GenerationUnit::Words;
        let unit2 = GenerationUnit::Words;
        let unit3 = GenerationUnit::Sentences;

        assert_eq!(unit1, unit2);
        assert_ne!(unit1, unit3);
    }

    #[test]
    fn should_implement_copy_clone() {
        let unit = GenerationUnit::Words;
        let copied = unit;

        assert_eq!(unit, copied);
    }

    #[test]
    fn should_implement_debug() {
        let unit = GenerationUnit::Words;
        let debug_str = format!("{unit:?}");
        assert_eq!(debug_str, "Words");
    }

    // Tests de parsing (FromStr)
    #[test]
    fn should_parse_valid_lowercase_strings() {
        assert_eq!(
            "words".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Words
        );
        assert_eq!(
            "sentences".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Sentences
        );
        assert_eq!(
            "paragraphs".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Paragraphs
        );
    }

    #[test]
    fn should_parse_valid_uppercase_strings() {
        assert_eq!(
            "WORDS".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Words
        );
        assert_eq!(
            "SENTENCES".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Sentences
        );
        assert_eq!(
            "PARAGRAPHS".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Paragraphs
        );
    }

    #[test]
    fn should_parse_valid_mixed_case_strings() {
        assert_eq!(
            "Words".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Words
        );
        assert_eq!(
            "Sentences".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Sentences
        );
        assert_eq!(
            "Paragraphs".parse::<GenerationUnit>().unwrap(),
            GenerationUnit::Paragraphs
        );
    }

    #[test]
    fn should_reject_invalid_strings() {
        let invalid_inputs = [
            "invalid",
            "",
            "word", // singulier au lieu de pluriel
            "sentence",
            "paragraph",
            "mot",
            "phrase",
            "paragraphe",
            "123",
            "words123",
            " words ",
        ];

        for input in invalid_inputs {
            assert!(
                input.parse::<GenerationUnit>().is_err(),
                "Expected '{input}' to be invalid but it parsed successfully"
            );
        }
    }

    #[test]
    fn should_provide_descriptive_error_messages() {
        let result = "invalid".parse::<GenerationUnit>();
        assert!(result.is_err());

        let error_message = result.unwrap_err();
        assert!(error_message.contains("Invalid generation unit"));
        assert!(error_message.contains("invalid"));
    }

    // Tests de sérialisation JSON (Serde)
    #[test]
    fn should_serialize_to_json() {
        let words = GenerationUnit::Words;
        let sentences = GenerationUnit::Sentences;
        let paragraphs = GenerationUnit::Paragraphs;

        assert_eq!(serde_json::to_string(&words).unwrap(), "\"words\"");
        assert_eq!(serde_json::to_string(&sentences).unwrap(), "\"sentences\"");
        assert_eq!(
            serde_json::to_string(&paragraphs).unwrap(),
            "\"paragraphs\""
        );
    }

    #[test]
    fn should_deserialize_from_json() {
        let words_json = "\"words\"";
        let sentences_json = "\"sentences\"";
        let paragraphs_json = "\"paragraphs\"";

        let words: GenerationUnit = serde_json::from_str(words_json).unwrap();
        let sentences: GenerationUnit = serde_json::from_str(sentences_json).unwrap();
        let paragraphs: GenerationUnit = serde_json::from_str(paragraphs_json).unwrap();

        assert_eq!(words, GenerationUnit::Words);
        assert_eq!(sentences, GenerationUnit::Sentences);
        assert_eq!(paragraphs, GenerationUnit::Paragraphs);
    }

    #[test]
    fn should_handle_json_serialization_roundtrip() {
        let units = [
            GenerationUnit::Words,
            GenerationUnit::Sentences,
            GenerationUnit::Paragraphs,
        ];

        for original_unit in units {
            let json = serde_json::to_string(&original_unit).unwrap();
            let deserialized_unit: GenerationUnit = serde_json::from_str(&json).unwrap();
            assert_eq!(original_unit, deserialized_unit);
        }
    }

    #[test]
    fn should_reject_invalid_json() {
        let invalid_json_inputs = [
            "\"invalid\"",
            "\"word\"",
            "\"sentence\"",
            "null",
            "123",
            "true",
            "{}",
            "[]",
        ];

        for json_input in invalid_json_inputs {
            let result = serde_json::from_str::<GenerationUnit>(json_input);
            assert!(
                result.is_err(),
                "Expected '{json_input}' to be invalid JSON but it parsed successfully",
            );
        }
    }

    // Tests exhaustifs pour éviter les oublis
    #[test]
    fn should_handle_all_variants_display_names() {
        let units = [
            GenerationUnit::Words,
            GenerationUnit::Sentences,
            GenerationUnit::Paragraphs,
        ];

        for unit in units {
            let display_name = unit.to_display_name();
            let api_name = unit.to_api_name();

            assert!(
                !display_name.is_empty(),
                "Display name should not be empty for {unit:?}",
            );
            assert!(
                !api_name.is_empty(),
                "API name should not be empty for {unit:?}",
            );

            // Vérifier que les noms sont cohérents
            assert!(
                display_name.len() > 2,
                "Display name seems too short for {unit:?}",
            );
            assert!(api_name.len() > 2, "API name seems too short for {unit:?}",);
        }
    }

    #[test]
    fn should_maintain_consistency_between_parsing_and_api_names() {
        let units = [
            GenerationUnit::Words,
            GenerationUnit::Sentences,
            GenerationUnit::Paragraphs,
        ];

        for unit in units {
            let api_name = unit.to_api_name();
            let parsed_unit = api_name.parse::<GenerationUnit>().unwrap();
            assert_eq!(
                unit, parsed_unit,
                "Parsing API name '{api_name}' should return the same unit {unit:?}"
            );
        }
    }

    // Tests de propriétés métier
    #[test]
    fn should_have_unique_display_names() {
        let words_display = GenerationUnit::Words.to_display_name();
        let sentences_display = GenerationUnit::Sentences.to_display_name();
        let paragraphs_display = GenerationUnit::Paragraphs.to_display_name();

        assert_ne!(words_display, sentences_display);
        assert_ne!(words_display, paragraphs_display);
        assert_ne!(sentences_display, paragraphs_display);
    }

    #[test]
    fn should_have_unique_api_names() {
        let words_api = GenerationUnit::Words.to_api_name();
        let sentences_api = GenerationUnit::Sentences.to_api_name();
        let paragraphs_api = GenerationUnit::Paragraphs.to_api_name();

        assert_ne!(words_api, sentences_api);
        assert_ne!(words_api, paragraphs_api);
        assert_ne!(sentences_api, paragraphs_api);
    }

    #[test]
    fn should_use_french_display_names() {
        assert_eq!(GenerationUnit::Words.to_display_name(), "mots");
        assert_eq!(GenerationUnit::Sentences.to_display_name(), "phrases");
        assert_eq!(GenerationUnit::Paragraphs.to_display_name(), "paragraphes");
    }

    #[test]
    fn should_use_english_api_names() {
        assert_eq!(GenerationUnit::Words.to_api_name(), "words");
        assert_eq!(GenerationUnit::Sentences.to_api_name(), "sentences");
        assert_eq!(GenerationUnit::Paragraphs.to_api_name(), "paragraphs");
    }
}
