use innertext_core::{inner_text_from_html, outer_text_from_html, text_content_from_html};
use pretty_assertions::assert_eq;

// ── innerText ──────────────────────────────────────────────────────────────
#[test]
fn drops_display_none_content() {
    let html = "<div>Hello <span style='display: none;'>Secret</span> World</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "Hello World");
}

#[test]
fn hidden_block_preserves_structure_boundaries() {
    let html = "<div>Left <p style='visibility: hidden;'>Hidden Block</p> Right</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "Left  Right");
}

#[test]
fn ignores_script_even_if_display_block() {
    let html = "<div>Visible <script style='display: block;'>alert(1)</script> End</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "Visible End");
}

#[test]
fn collapses_consecutive_whitespace_for_normal() {
    let html = "<div style='white-space: normal;'>  Multiple   spaces    here  </div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "Multiple spaces here");
}

#[test]
fn preserves_spaces_for_pre() {
    let html = "<div style='white-space: pre;'>  Preserve   spaces  \n  newlines</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "  Preserve   spaces  \n  newlines");
}

#[test]
fn br_injects_newline() {
    let html = "<div>Line One<br>Line Two</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "Line One\nLine Two");
}

#[test]
fn sibling_blocks_have_two_newlines_between_them() {
    let html = "<div>Block A</div><div>Block B</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "Block A\nBlock B");
}

// ── outerText getter ───────────────────────────────────────────────────────

#[test]
fn outer_text_getter_equals_inner_text() {
    // WHATWG 3.2.7: outerText getter steps are identical to innerText getter.
    let cases = [
        "<div>Hello World</div>",
        "<div>Line One<br>Line Two</div>",
        "<p>Para</p><p>Two</p>",
    ];
    for html in cases {
        assert_eq!(
            inner_text_from_html(html).unwrap(),
            outer_text_from_html(html).unwrap(),
            "mismatch for: {html}"
        );
    }
}

// ── textContent ────────────────────────────────────────────────────────────

#[test]
fn text_content_is_css_blind() {
    // display:none content IS included — textContent has no CSS involvement.
    let html = "<div>Hello <span style='display:none'>Hidden</span> World</div>";
    assert_eq!(text_content_from_html(html).unwrap(), "Hello Hidden World");
}

#[test]
fn text_content_includes_script_text() {
    let html = "<div>A<script>alert(1)</script>B</div>";
    assert_eq!(text_content_from_html(html).unwrap(), "Aalert(1)B");
}

#[test]
fn text_content_preserves_raw_whitespace() {
    // No whitespace collapsing — raw text node data is concatenated.
    let html = "<div>  Multiple   spaces  </div>";
    assert_eq!(
        text_content_from_html(html).unwrap(),
        "  Multiple   spaces  "
    );
}

#[test]
fn text_content_includes_visibility_hidden() {
    let html = "<div>Visible<span style='visibility:hidden'>Ghost</span>End</div>";
    assert_eq!(text_content_from_html(html).unwrap(), "VisibleGhostEnd");
}
