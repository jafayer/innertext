mod css;
mod inner_text;
mod model;
mod text_content;

pub use model::ExtractionError;

use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::RcDom;

/// A parsed HTML document.  Parse once; extract in any mode.
pub struct Document {
    dom: RcDom,
}

impl Document {
    pub fn parse(html: &str) -> Result<Self, ExtractionError> {
        let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(html);
        Ok(Self { dom })
    }

    /// WHATWG innerText getter: CSS-aware rendered text.
    pub fn inner_text(&self) -> String {
        inner_text::extract(&self.dom)
    }

    /// WHATWG outerText getter: identical to innerText getter per spec 3.2.7.
    pub fn outer_text(&self) -> String {
        self.inner_text()
    }

    /// DOM textContent: structural concatenation of all Text node descendants,
    /// no CSS involvement.
    pub fn text_content(&self) -> String {
        text_content::extract(&self.dom)
    }
}

// Convenience free functions for callers that only need a single extraction.

pub fn inner_text_from_html(html: &str) -> Result<String, ExtractionError> {
    Ok(Document::parse(html)?.inner_text())
}

pub fn outer_text_from_html(html: &str) -> Result<String, ExtractionError> {
    Ok(Document::parse(html)?.outer_text())
}

pub fn text_content_from_html(html: &str) -> Result<String, ExtractionError> {
    Ok(Document::parse(html)?.text_content())
}

#[cfg(test)]
mod tests {
    use super::{inner_text_from_html, outer_text_from_html, text_content_from_html};

    #[test]
    fn inner_text_collapses_whitespace() {
        let html = "<div style='white-space: normal;'>  Multiple   spaces    here  </div>";
        assert_eq!(inner_text_from_html(html).unwrap(), "Multiple spaces here");
    }

    #[test]
    fn inner_text_preserves_whitespace_in_pre() {
        let html = "<div style='white-space: pre;'>  Preserve   spaces  \n  newlines</div>";
        assert_eq!(
            inner_text_from_html(html).unwrap(),
            "  Preserve   spaces  \n  newlines"
        );
    }

    #[test]
    fn outer_text_getter_matches_inner_text() {
        let html = "<div>Hello <span>World</span></div>";
        assert_eq!(
            inner_text_from_html(html).unwrap(),
            outer_text_from_html(html).unwrap()
        );
    }

    #[test]
    fn text_content_includes_display_none() {
        // textContent is CSS-blind: display:none content IS included.
        let html = "<div>Hello <span style='display:none'>Hidden</span> World</div>";
        assert_eq!(text_content_from_html(html).unwrap(), "Hello Hidden World");
    }

    #[test]
    fn text_content_includes_script_content() {
        // textContent does not strip metadata elements.
        let html = "<div>A<script>alert(1)</script>B</div>";
        assert_eq!(text_content_from_html(html).unwrap(), "Aalert(1)B");
    }
}
