use innertext_core::Document;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Extract innerText from HTML (rendered text collection per WHATWG spec)
#[pyfunction]
fn inner_text(html: &str) -> PyResult<String> {
    Document::parse(html)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse HTML: {}", e)))
        .map(|doc| doc.inner_text())
}

/// Extract outerText from HTML (identical to innerText getter per WHATWG spec)
#[pyfunction]
fn outer_text(html: &str) -> PyResult<String> {
    Document::parse(html)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse HTML: {}", e)))
        .map(|doc| doc.outer_text())
}

/// Extract textContent from HTML (CSS-blind structural text)
#[pyfunction]
fn text_content(html: &str) -> PyResult<String> {
    Document::parse(html)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse HTML: {}", e)))
        .map(|doc| doc.text_content())
}

#[pymodule]
fn innertext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(inner_text, m)?)?;
    m.add_function(wrap_pyfunction!(outer_text, m)?)?;
    m.add_function(wrap_pyfunction!(text_content, m)?)?;
    Ok(())
}
