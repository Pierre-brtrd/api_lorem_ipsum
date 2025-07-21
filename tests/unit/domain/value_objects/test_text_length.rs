use api_lorem_ipsum::domain::errors::DomainError;
use api_lorem_ipsum::domain::value_objects::text_length::TextLength;
use serde_json;

#[cfg(test)]
mod text_length_tests {
    use super::*;

    // Tests de création et validation
    #[test]
    fn should_create_valid_text_length() {
        let length = TextLength::new(50);
        assert!(length.is_ok());
        assert_eq!(length.unwrap().value(), 50);
    }

    #[test]
    fn should_create_minimum_valid_length() {
        let length = TextLength::new(1);
        assert!(length.is_ok());
        assert_eq!(length.unwrap().value(), 1);
    }

    #[test]
    fn should_create_maximum_valid_length() {
        let length = TextLength::new(1000);
        assert!(length.is_ok());
        assert_eq!(length.unwrap().value(), 1000);
    }

    #[test]
    fn should_reject_zero_length() {
        let result = TextLength::new(0);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(
            error,
            DomainError::InvalidTextLength {
                value: 0,
                min: 1,
                max: 1000
            }
        ));
    }

    #[test]
    fn should_reject_too_large_length() {
        let result = TextLength::new(1001);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(
            error,
            DomainError::InvalidTextLength {
                value: 1001,
                min: 1,
                max: 1000
            }
        ));
    }

    #[test]
    fn should_reject_very_large_values() {
        let invalid_values = [1001, 2000, 5000, 10000, u32::MAX];

        for value in invalid_values {
            let result = TextLength::new(value);
            assert!(
                result.is_err(),
                "Expected value {value} to be invalid but it was accepted"
            );
        }
    }

    #[test]
    fn should_provide_descriptive_error_messages() {
        let result = TextLength::new(0);
        assert!(result.is_err());

        let error_message = result.unwrap_err();
        let error_string = error_message.to_string();
        assert!(
            error_string.len() > 10,
            "Error message should be descriptive, got: '{error_string}'",
        );

        // Should contain French message
        assert!(error_string.to_lowercase().contains("longueur"));
    }

    // Tests d'égalité et traits dérivés
    #[test]
    fn should_implement_equality() {
        let length1 = TextLength::new(100).unwrap();
        let length2 = TextLength::new(100).unwrap();
        let length3 = TextLength::new(200).unwrap();

        assert_eq!(length1, length2);
        assert_ne!(length1, length3);
    }

    #[test]
    fn should_implement_clone() {
        let original = TextLength::new(250).unwrap();
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.value(), cloned.value());
    }

    #[test]
    fn should_implement_debug() {
        let length = TextLength::new(42).unwrap();
        let debug_str = format!("{length:?}");

        assert!(debug_str.contains("TextLength"));
        assert!(debug_str.contains("42"));
    }

    // Tests d'accesseur
    #[test]
    fn should_provide_value_accessor() {
        let test_cases = [1, 50, 100, 500, 999, 1000];

        for expected_value in test_cases {
            let length = TextLength::new(expected_value).unwrap();
            assert_eq!(length.value(), expected_value);
        }
    }

    #[test]
    fn should_keep_value_private() {
        // Ce test vérifie que la valeur n'est accessible que via l'accesseur
        let length = TextLength::new(123).unwrap();

        // La seule façon d'accéder à la valeur devrait être via value()
        assert_eq!(length.value(), 123);

        // Si on pouvait accéder directement, ce serait un problème d'encapsulation
        // length.value (sans parenthèses) devrait être une erreur de compilation
    }

    // Tests de sérialisation JSON (Serde)
    #[test]
    fn should_serialize_to_json() {
        let length = TextLength::new(150).unwrap();
        let json = serde_json::to_string(&length).unwrap();

        // Devrait sérialiser comme un simple nombre
        assert_eq!(json, "150");
    }

    #[test]
    fn should_deserialize_from_valid_json() {
        let json = "75";
        let length: TextLength = serde_json::from_str(json).unwrap();

        assert_eq!(length.value(), 75);
    }

    #[test]
    fn should_reject_invalid_json_values() {
        let invalid_json_cases = [
            "0",     // Trop petit
            "1001",  // Trop grand
            "99999", // Beaucoup trop grand
        ];

        for json_case in invalid_json_cases {
            let result = serde_json::from_str::<TextLength>(json_case);
            assert!(
                result.is_err(),
                "Expected JSON '{json_case}' to be invalid but it was accepted",
            );
        }
    }

    #[test]
    fn should_reject_malformed_json() {
        let malformed_json_cases = [
            "\"not_a_number\"",
            "null",
            "true",
            "[]",
            "{}",
            r#"{"value": 100}"#, // Structure d'objet, pas un simple nombre
        ];

        for json_case in malformed_json_cases {
            let result = serde_json::from_str::<TextLength>(json_case);
            assert!(
                result.is_err(),
                "Expected malformed JSON '{json_case}' to be rejected"
            );
        }
    }

    #[test]
    fn should_handle_json_serialization_roundtrip() {
        let test_values = [1, 25, 100, 500, 750, 1000];

        for value in test_values {
            let original = TextLength::new(value).unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: TextLength = serde_json::from_str(&json).unwrap();

            assert_eq!(original, deserialized);
            assert_eq!(original.value(), deserialized.value());
        }
    }

    // Tests des limites et cas spéciaux
    #[test]
    fn should_handle_boundary_values_correctly() {
        // Valeurs limites valides
        let valid_boundaries = [1, 1000];
        for value in valid_boundaries {
            let result = TextLength::new(value);
            assert!(result.is_ok(), "Boundary value {value} should be valid");
        }

        // Valeurs limites invalides
        let invalid_boundaries = [0, 1001];
        for value in invalid_boundaries {
            let result = TextLength::new(value);
            assert!(result.is_err(), "Boundary value {value} should be invalid");
        }
    }

    #[test]
    fn should_handle_common_use_case_values() {
        // Valeurs typiques qu'on pourrait voir dans une vraie utilisation
        let common_values = [5, 10, 25, 50, 100, 200, 300, 500];

        for value in common_values {
            let length = TextLength::new(value).unwrap();
            assert_eq!(length.value(), value);

            // Test que ces valeurs survivent à la sérialisation
            let json = serde_json::to_string(&length).unwrap();
            let deserialized: TextLength = serde_json::from_str(&json).unwrap();
            assert_eq!(length, deserialized);
        }
    }

    // Tests de consistance métier
    #[test]
    fn should_maintain_business_rule_consistency() {
        // Vérifier que les règles métier sont cohérentes
        // Min = 1, Max = 1000

        // Test de la règle minimum
        assert!(TextLength::new(1).is_ok());
        assert!(TextLength::new(0).is_err());

        // Test de la règle maximum
        assert!(TextLength::new(1000).is_ok());
        assert!(TextLength::new(1001).is_err());

        // Test d'une plage raisonnable
        for value in 1..=1000 {
            if value % 100 == 0 {
                // Test chaque centaine pour la performance
                assert!(
                    TextLength::new(value).is_ok(),
                    "Value {value} within valid range should be accepted"
                );
            }
        }
    }

    #[test]
    fn should_preserve_immutability() {
        // Une fois créé, un TextLength ne peut pas être modifié
        let length = TextLength::new(42).unwrap();
        let original_value = length.value();

        // Cloner ne devrait pas affecter l'original
        let _cloned = length.clone();
        assert_eq!(length.value(), original_value);

        // La valeur devrait rester stable
        assert_eq!(length.value(), 42);
        assert_eq!(length.value(), original_value);
    }
}
