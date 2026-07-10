use innertext_core::Document;
use napi::{bindgen_prelude::Result, Error, Status};
use napi_derive::napi;

#[napi(js_name = "innerText")]
pub fn inner_text(html: String) -> Result<String> {
    Document::parse(&html)
        .map(|doc| doc.inner_text())
        .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse HTML: {}", e)))
}

#[napi(js_name = "outerText")]
pub fn outer_text(html: String) -> Result<String> {
    Document::parse(&html)
        .map(|doc| doc.outer_text())
        .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse HTML: {}", e)))
}

#[napi(js_name = "textContent")]
pub fn text_content(html: String) -> Result<String> {
    Document::parse(&html)
        .map(|doc| doc.text_content())
        .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse HTML: {}", e)))
}

#[napi(js_name = "HtmlDocument")]
pub struct HtmlDocument {
    doc: Document,
}

#[napi]
impl HtmlDocument {
    #[napi(constructor)]
    pub fn new(html: String) -> Result<Self> {
        let doc = Document::parse(&html)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse HTML: {}", e)))?;
        Ok(Self { doc })
    }

    #[napi(js_name = "innerText")]
    pub fn inner_text(&self) -> String {
        self.doc.inner_text()
    }

    #[napi(js_name = "outerText")]
    pub fn outer_text(&self) -> String {
        self.doc.outer_text()
    }

    #[napi(js_name = "textContent")]
    pub fn text_content(&self) -> String {
        self.doc.text_content()
    }
}
