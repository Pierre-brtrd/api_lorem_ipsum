use api_lorem_ipsum::domain::value_objects::html_tags::{HtmlTag, HtmlTags};

#[cfg(test)]
mod html_tags_tests {
    use super::*;

    // Tests de création et validation
    #[test]
    fn should_create_valid_html_tags() {
        let tags = vec![HtmlTag::Header, HtmlTag::Link];
        let html_tags = HtmlTags::new(tags).unwrap();

        assert_eq!(html_tags.len(), 2);
        assert!(html_tags.contains(&HtmlTag::Header));
        assert!(html_tags.contains(&HtmlTag::Link));
    }

    #[test]
    fn should_reject_empty_tags_list() {
        let result = HtmlTags::new(vec![]);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "La liste des balises HTML ne peut pas être vide"
        );
    }

    #[test]
    fn should_remove_duplicate_tags() {
        let tags = vec![
            HtmlTag::Header,
            HtmlTag::Link,
            HtmlTag::Header,
            HtmlTag::Div,
        ];
        let html_tags = HtmlTags::new(tags).unwrap();

        assert_eq!(html_tags.len(), 3); // Header dupliqué supprimé
        assert!(html_tags.contains(&HtmlTag::Header));
        assert!(html_tags.contains(&HtmlTag::Link));
        assert!(html_tags.contains(&HtmlTag::Div));
    }

    #[test]
    fn should_sort_tags_for_consistency() {
        let tags = vec![HtmlTag::Ul, HtmlTag::Header, HtmlTag::Code];
        let html_tags = HtmlTags::new(tags).unwrap();

        // Les tags devraient être triés selon l'ordre défini dans l'enum
        let sorted_tags = html_tags.tags();
        assert!(sorted_tags.windows(2).all(|w| w[0] <= w[1]));
    }

    // Tests de constructeurs alternatifs
    #[test]
    fn should_create_single_tag() {
        let html_tags = HtmlTags::single(HtmlTag::Header).unwrap();

        assert_eq!(html_tags.len(), 1);
        assert!(html_tags.contains(&HtmlTag::Header));
    }

    #[test]
    fn should_create_from_url_parts() {
        let parts = ["headers", "link", "ul"];
        let html_tags = HtmlTags::from_url_parts(&parts).unwrap();

        assert_eq!(html_tags.len(), 3);
        assert!(html_tags.contains(&HtmlTag::Header));
        assert!(html_tags.contains(&HtmlTag::Link));
        assert!(html_tags.contains(&HtmlTag::Ul));
    }

    #[test]
    fn should_reject_empty_url_parts() {
        let result = HtmlTags::from_url_parts(&[]);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Aucune balise HTML spécifiée dans l'URL"
        );
    }

    #[test]
    fn should_reject_invalid_url_parts() {
        let parts = ["headers", "invalid_tag", "link"];
        let result = HtmlTags::from_url_parts(&parts);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Tag HTML non reconnu: 'invalid_tag'"));
    }

    // Tests des accesseurs
    #[test]
    fn should_provide_tags_access() {
        let tags = vec![HtmlTag::Header, HtmlTag::Link];
        let html_tags = HtmlTags::new(tags).unwrap();

        let tags_slice = html_tags.tags();
        assert_eq!(tags_slice.len(), 2);
        assert!(tags_slice.contains(&HtmlTag::Header));
        assert!(tags_slice.contains(&HtmlTag::Link));
    }

    #[test]
    fn should_check_tag_containment() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();

        assert!(html_tags.contains(&HtmlTag::Header));
        assert!(html_tags.contains(&HtmlTag::Link));
        assert!(!html_tags.contains(&HtmlTag::Div));
    }

    #[test]
    fn should_provide_length_information() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link, HtmlTag::Div]).unwrap();

        assert_eq!(html_tags.len(), 3);
        assert!(!html_tags.is_empty());

        let empty_result = HtmlTags::new(vec![]);
        assert!(empty_result.is_err()); // Ne peut pas créer de collection vide
    }

    // Tests de combinaison
    #[test]
    fn should_combine_html_tags() {
        let tags1 = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();
        let tags2 = HtmlTags::new(vec![HtmlTag::Ul, HtmlTag::Div]).unwrap();

        let combined = tags1.combine(&tags2).unwrap();

        assert_eq!(combined.len(), 4);
        assert!(combined.contains(&HtmlTag::Header));
        assert!(combined.contains(&HtmlTag::Link));
        assert!(combined.contains(&HtmlTag::Ul));
        assert!(combined.contains(&HtmlTag::Div));
    }

    #[test]
    fn should_remove_duplicates_when_combining() {
        let tags1 = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();
        let tags2 = HtmlTags::new(vec![HtmlTag::Link, HtmlTag::Div]).unwrap();

        let combined = tags1.combine(&tags2).unwrap();

        assert_eq!(combined.len(), 3); // Link ne devrait apparaître qu'une fois
        assert!(combined.contains(&HtmlTag::Header));
        assert!(combined.contains(&HtmlTag::Link));
        assert!(combined.contains(&HtmlTag::Div));
    }

    // Tests de validation de compatibilité
    #[test]
    fn should_validate_compatible_tags() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link, HtmlTag::Div]).unwrap();

        assert!(html_tags.validate_compatibility().is_ok());
    }

    #[test]
    fn should_detect_incompatible_tags() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Ul, HtmlTag::Ol]).unwrap();

        let result = html_tags.validate_compatibility();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("ne sont pas compatibles"));
    }

    // Tests de comptage d'éléments
    #[test]
    fn should_count_block_elements() {
        let html_tags = HtmlTags::new(vec![
            HtmlTag::Header, // Non-block (ni block ni inline dans is_block_element)
            HtmlTag::Div,    // Block
            HtmlTag::Ul,     // Block
            HtmlTag::Link,   // Inline
            HtmlTag::Pre,    // Block
        ])
        .unwrap();

        assert_eq!(html_tags.count_block_elements(), 3); // Div, Ul, Pre
    }

    #[test]
    fn should_count_inline_elements() {
        let html_tags = HtmlTags::new(vec![
            HtmlTag::Header, // Ni block ni inline
            HtmlTag::Link,   // Inline
            HtmlTag::Code,   // Inline
            HtmlTag::Div,    // Block
        ])
        .unwrap();

        assert_eq!(html_tags.count_inline_elements(), 2); // Link, Code
    }

    // Tests de conversion URL
    #[test]
    fn should_convert_to_url_string() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link, HtmlTag::Ul]).unwrap();

        let url_string = html_tags.to_url_string();
        // Les tags sont triés, donc l'ordre devrait être prévisible
        assert!(url_string.contains("headers"));
        assert!(url_string.contains("link"));
        assert!(url_string.contains("ul"));
        assert!(url_string.contains("/"));
    }

    // Tests des traits
    #[test]
    fn should_implement_display() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();

        let display_string = format!("{html_tags}");
        assert!(display_string.starts_with("["));
        assert!(display_string.ends_with("]"));
        assert!(display_string.contains("headers"));
        assert!(display_string.contains("link"));
    }

    #[test]
    fn should_implement_clone() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();
        let cloned = html_tags.clone();

        assert_eq!(html_tags, cloned);
        assert_eq!(html_tags.len(), cloned.len());
    }

    #[test]
    fn should_implement_equality() {
        let tags1 = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();
        let tags2 = HtmlTags::new(vec![HtmlTag::Link, HtmlTag::Header]).unwrap(); // Ordre différent
        let tags3 = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Div]).unwrap();

        assert_eq!(tags1, tags2); // L'ordre ne devrait pas affecter l'égalité
        assert_ne!(tags1, tags3);
    }

    #[test]
    fn should_implement_debug() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header]).unwrap();

        let debug_string = format!("{html_tags:?}");
        assert!(debug_string.contains("HtmlTags"));
        assert!(debug_string.contains("Header"));
    }

    // Tests de conversion From/TryFrom
    #[test]
    fn should_convert_from_single_tag() {
        let html_tags: HtmlTags = HtmlTag::Header.into();

        assert_eq!(html_tags.len(), 1);
        assert!(html_tags.contains(&HtmlTag::Header));
    }

    #[test]
    fn should_try_from_vec_str() {
        let parts = vec!["headers", "link"];
        let html_tags: HtmlTags = parts.try_into().unwrap();

        assert_eq!(html_tags.len(), 2);
        assert!(html_tags.contains(&HtmlTag::Header));
        assert!(html_tags.contains(&HtmlTag::Link));
    }

    #[test]
    fn should_fail_try_from_invalid_vec_str() {
        let parts = vec!["invalid_tag"];
        let result: Result<HtmlTags, _> = parts.try_into();

        assert!(result.is_err());
    }

    // Tests de sérialisation JSON
    #[test]
    fn should_serialize_to_json() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();
        let json = serde_json::to_string(&html_tags).unwrap();

        // Devrait sérialiser comme un array de strings
        assert!(json.contains("\"headers\""));
        assert!(json.contains("\"link\""));
        assert!(json.starts_with("["));
        assert!(json.ends_with("]"));
    }

    #[test]
    fn should_deserialize_from_json() {
        let json = r#"["headers", "link", "ul"]"#;
        let html_tags: HtmlTags = serde_json::from_str(json).unwrap();

        assert_eq!(html_tags.len(), 3);
        assert!(html_tags.contains(&HtmlTag::Header));
        assert!(html_tags.contains(&HtmlTag::Link));
        assert!(html_tags.contains(&HtmlTag::Ul));
    }

    #[test]
    fn should_reject_invalid_json_tags() {
        let json = r#"["headers", "invalid_tag"]"#;
        let result: Result<HtmlTags, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn should_reject_empty_json_array() {
        let json = r#"[]"#;
        let result: Result<HtmlTags, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn should_handle_json_serialization_roundtrip() {
        let original = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link, HtmlTag::Ul]).unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: HtmlTags = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    // Tests de cas limites et edge cases
    #[test]
    fn should_handle_all_tag_variants() {
        let all_tags = HtmlTag::all_variants();
        let html_tags = HtmlTags::new(all_tags).unwrap();

        assert_eq!(html_tags.len(), 9); // Nombre total de variants

        // Vérifier que tous les tags sont présents
        for tag in HtmlTag::all_variants() {
            assert!(html_tags.contains(&tag));
        }
    }

    #[test]
    fn should_maintain_immutability() {
        let html_tags = HtmlTags::new(vec![HtmlTag::Header, HtmlTag::Link]).unwrap();
        let original_len = html_tags.len();

        // Obtenir une référence aux tags ne devrait pas permettre de modification
        let _tags_ref = html_tags.tags();

        // La longueur devrait rester la même
        assert_eq!(html_tags.len(), original_len);
    }

    #[test]
    fn should_provide_consistent_ordering() {
        // Créer plusieurs fois avec le même contenu mais ordre différent
        let tags1 = HtmlTags::new(vec![HtmlTag::Link, HtmlTag::Header, HtmlTag::Div]).unwrap();
        let tags2 = HtmlTags::new(vec![HtmlTag::Div, HtmlTag::Header, HtmlTag::Link]).unwrap();

        // L'ordre des tags dans la représentation interne devrait être consistant
        assert_eq!(tags1.tags(), tags2.tags());
        assert_eq!(tags1.to_url_string(), tags2.to_url_string());
    }
}

#[cfg(test)]
mod html_tag_tests {
    use super::*;

    // Tests de parsing URL
    #[test]
    fn should_parse_all_valid_url_names() {
        let test_cases = [
            ("headers", HtmlTag::Header),
            ("header", HtmlTag::Header),
            ("link", HtmlTag::Link),
            ("links", HtmlTag::Link),
            ("ul", HtmlTag::Ul),
            ("unordered", HtmlTag::Ul),
            ("ol", HtmlTag::Ol),
            ("ordered", HtmlTag::Ol),
            ("dl", HtmlTag::Dl),
            ("definition", HtmlTag::Dl),
            ("pre", HtmlTag::Pre),
            ("preformatted", HtmlTag::Pre),
            ("code", HtmlTag::Code),
            ("blockquote", HtmlTag::Blockquote),
            ("quote", HtmlTag::Blockquote),
            ("div", HtmlTag::Div),
        ];

        for (input, expected) in test_cases {
            let result = HtmlTag::from_url_name(input).unwrap();
            assert_eq!(result, expected, "Failed to parse '{input}'");
        }
    }

    #[test]
    fn should_handle_case_insensitive_parsing() {
        let test_cases = ["HEADERS", "Link", "UL", "oL", "DL"];

        for case in test_cases {
            let result = HtmlTag::from_url_name(case);
            assert!(result.is_ok(), "Should parse case insensitive: '{case}'");
        }
    }

    #[test]
    fn should_reject_invalid_url_names() {
        let invalid_names = ["invalid", "span", "p", "h1", "table", ""];

        for name in invalid_names {
            let result = HtmlTag::from_url_name(name);
            assert!(result.is_err(), "Should reject invalid name: '{name}'");
            assert!(result.unwrap_err().contains("Tag HTML non reconnu"));
        }
    }

    // Tests de conversion vers URL
    #[test]
    fn should_convert_to_url_names() {
        let test_cases = [
            (HtmlTag::Header, "headers"),
            (HtmlTag::Link, "link"),
            (HtmlTag::Ul, "ul"),
            (HtmlTag::Ol, "ol"),
            (HtmlTag::Dl, "dl"),
            (HtmlTag::Pre, "pre"),
            (HtmlTag::Code, "code"),
            (HtmlTag::Blockquote, "blockquote"),
            (HtmlTag::Div, "div"),
        ];

        for (tag, expected) in test_cases {
            assert_eq!(tag.to_url_name(), expected);
        }
    }

    // Tests de classification des éléments
    #[test]
    fn should_identify_block_elements() {
        let block_elements = [
            HtmlTag::Div,
            HtmlTag::Blockquote,
            HtmlTag::Pre,
            HtmlTag::Ul,
            HtmlTag::Ol,
            HtmlTag::Dl,
        ];

        for tag in block_elements {
            assert!(tag.is_block_element(), "{tag:?} should be a block element");
        }
    }

    #[test]
    fn should_identify_inline_elements() {
        let inline_elements = [HtmlTag::Link, HtmlTag::Code];

        for tag in inline_elements {
            assert!(
                tag.is_inline_element(),
                "{tag:?} should be an inline element"
            );
        }
    }

    #[test]
    fn should_identify_header_as_neither_block_nor_inline() {
        // Header est un cas spécial - ni vraiment block ni inline
        assert!(!HtmlTag::Header.is_block_element());
        assert!(!HtmlTag::Header.is_inline_element());
    }

    // Tests de compatibilité
    #[test]
    fn should_detect_incompatible_list_types() {
        assert!(!HtmlTag::Ul.is_compatible_with(&HtmlTag::Ol));
        assert!(!HtmlTag::Ol.is_compatible_with(&HtmlTag::Ul));
    }

    #[test]
    fn should_allow_most_tag_combinations() {
        let compatible_pairs = [
            (HtmlTag::Header, HtmlTag::Link),
            (HtmlTag::Div, HtmlTag::Code),
            (HtmlTag::Blockquote, HtmlTag::Link),
            (HtmlTag::Pre, HtmlTag::Code),
        ];

        for (tag1, tag2) in compatible_pairs {
            assert!(
                tag1.is_compatible_with(&tag2),
                "{tag1:?} should be compatible with {tag2:?}"
            );
            assert!(
                tag2.is_compatible_with(&tag1),
                "{tag2:?} should be compatible with {tag1:?}"
            );
        }
    }

    // Tests de besoins de contenu
    #[test]
    fn should_identify_tags_requiring_content() {
        let content_required = [
            HtmlTag::Header,
            HtmlTag::Link,
            HtmlTag::Ul,
            HtmlTag::Ol,
            HtmlTag::Dl,
            HtmlTag::Pre,
            HtmlTag::Code,
            HtmlTag::Blockquote,
        ];

        for tag in content_required {
            assert!(tag.requires_content(), "{tag:?} should require content");
        }
    }

    #[test]
    fn should_allow_empty_div() {
        assert!(
            !HtmlTag::Div.requires_content(),
            "Div should allow empty content"
        );
    }

    // Tests de tous les variants
    #[test]
    fn should_include_all_variants() {
        let all_variants = HtmlTag::all_variants();

        assert_eq!(all_variants.len(), 9);

        // Vérifier que tous les variants sont uniques
        let mut sorted_variants = all_variants.clone();
        sorted_variants.sort();
        sorted_variants.dedup();
        assert_eq!(sorted_variants.len(), all_variants.len());
    }

    // Tests des traits
    #[test]
    fn should_implement_display() {
        for tag in HtmlTag::all_variants() {
            let display_string = format!("{tag}");
            assert_eq!(display_string, tag.to_url_name());
        }
    }

    #[test]
    fn should_implement_ordering() {
        let mut tags = HtmlTag::all_variants();
        tags.reverse(); // Désordre volontaire

        tags.sort();

        // Vérifier que le tri fonctionne
        assert!(tags.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn should_implement_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        for tag in HtmlTag::all_variants() {
            set.insert(tag);
        }

        assert_eq!(set.len(), 9); // Tous les tags doivent être uniques dans le HashSet
    }
}
