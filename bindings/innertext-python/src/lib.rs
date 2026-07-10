pub use innertext_core::ExtractionError;
pub use innertext_core::Document;

pub fn inner_text(input: &str) -> Result<String, ExtractionError> {
    innertext_core::inner_text_from_html(input)
}

pub fn outer_text(input: &str) -> Result<String, ExtractionError> {
    innertext_core::outer_text_from_html(input)
}

pub fn text_content(input: &str) -> Result<String, ExtractionError> {
    innertext_core::text_content_from_html(input)
}
