pub mod generation_unit;
pub mod html_tags;
pub mod text_format;
pub mod text_length;
pub mod text_length_category;

// Re-exports pour faciliter l'usage
pub use generation_unit::GenerationUnit;
pub use html_tags::{HtmlTag, HtmlTags};
pub use text_format::TextFormat;
pub use text_length::TextLength;
pub use text_length_category::{HtmlComplexity, TextLengthCategory};
