pub fn inner_text(input: &str) -> Result<String, innertext_core::InnerTextError> {
    innertext_core::inner_text_from_html(input)
}
