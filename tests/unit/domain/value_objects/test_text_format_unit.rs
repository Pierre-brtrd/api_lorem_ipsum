use api_lorem_ipsum::domain::value_objects::text_format::TextFormat;

#[cfg(test)]
mod generation_unit_tests {

    use super::*;

    // Tests de création et méthodes de base
    #[test]
    fn should_create_plan_text_unit() {
        let unit = TextFormat::PlainText;
        assert_eq!(unit.to_display_name(), "Texte brut");
        assert_eq!(unit.to_api_name(), "plain_text");
    }

    #[test]
    fn should_create_markdown_unit() {
        let unit = TextFormat::Markdown;
        assert_eq!(unit.to_display_name(), "Markdown");
        assert_eq!(unit.to_api_name(), "markdown");
    }

    #[test]
    fn should_create_html_unit() {
        let unit = TextFormat::HTML;
        assert_eq!(unit.to_display_name(), "HTML");
        assert_eq!(unit.to_api_name(), "html");
    }

    #[test]
    fn should_implement_equality() {
        let unit1 = TextFormat::PlainText;
        let unit2 = TextFormat::PlainText;
        let unit3 = TextFormat::Markdown;

        assert_eq!(unit1, unit2);
        assert_ne!(unit1, unit3);
    }

    #[test]
    fn should_implement_copy() {
        let unit = TextFormat::PlainText;
        let copied = unit;
        assert_eq!(unit, copied);
    }

    #[test]
    fn should_implement_debug() {
        let unit = TextFormat::PlainText;
        let debug_str = format!("{unit:?}");
        assert_eq!(debug_str, "PlainText");
    }

    // Tests de parsing (FromStr)
    #[test]
    fn should_parse_valid_lowercase_strings() {
        assert_eq!(
            "plain_text".parse::<TextFormat>().unwrap(),
            TextFormat::PlainText
        );
        assert_eq!(
            "markdown".parse::<TextFormat>().unwrap(),
            TextFormat::Markdown
        );
        assert_eq!("html".parse::<TextFormat>().unwrap(), TextFormat::HTML);
    }

    #[test]
    fn should_fail_on_invalid_strings() {
        let invalid_strings = ["txt", "html5", "markdowns", "text"];
        for &s in &invalid_strings {
            let result = s.parse::<TextFormat>();
            assert!(result.is_err(), "Expected error for invalid string: {s}");
        }
    }

    #[test]
    fn should_fail_on_empty_string() {
        let result = "".parse::<TextFormat>();
        assert!(result.is_err(), "Expected error for empty string");
    }

    #[test]
    fn should_serialize_to_json() {
        let plan_text = TextFormat::PlainText;
        let markdown = TextFormat::Markdown;
        let html = TextFormat::HTML;

        assert_eq!(serde_json::to_string(&plan_text).unwrap(), r#""plaintext""#);
        assert_eq!(serde_json::to_string(&markdown).unwrap(), r#""markdown""#);
        assert_eq!(serde_json::to_string(&html).unwrap(), r#""html""#);
    }

    #[test]
    fn should_deserialize_from_json() {
        let plan_text: TextFormat = serde_json::from_str(r#""plaintext""#).unwrap();
        let markdown: TextFormat = serde_json::from_str(r#""markdown""#).unwrap();
        let html: TextFormat = serde_json::from_str(r#""html""#).unwrap();

        assert_eq!(plan_text, TextFormat::PlainText);
        assert_eq!(markdown, TextFormat::Markdown);
        assert_eq!(html, TextFormat::HTML);
    }

    #[test]
    fn should_use_french_display_names() {
        assert_eq!(TextFormat::PlainText.to_display_name(), "Texte brut");
        assert_eq!(TextFormat::Markdown.to_display_name(), "Markdown");
        assert_eq!(TextFormat::HTML.to_display_name(), "HTML");
    }

    #[test]
    fn should_use_french_api_names() {
        assert_eq!(TextFormat::PlainText.to_api_name(), "plain_text");
        assert_eq!(TextFormat::Markdown.to_api_name(), "markdown");
        assert_eq!(TextFormat::HTML.to_api_name(), "html");
    }
}
