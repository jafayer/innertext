mod model;
mod render;

pub use model::InnerTextError;

pub fn inner_text_from_html(input: &str) -> Result<String, InnerTextError> {
    render::inner_text_from_html(input)
}

#[cfg(test)]
mod tests {
    use super::inner_text_from_html;

    #[test]
    fn collapses_whitespace_in_normal_mode() {
        let html = "<div style='white-space: normal;'>  Multiple   spaces    here  </div>";
        let got = inner_text_from_html(html).expect("must extract text");
        assert_eq!(got, "Multiple spaces here");
    }

    #[test]
    fn preserves_whitespace_in_pre_mode() {
        let html = "<div style='white-space: pre;'>  Preserve   spaces  \n  newlines</div>";
        let got = inner_text_from_html(html).expect("must extract text");
        assert_eq!(got, "  Preserve   spaces  \n  newlines");
    }
}
