use innertext_core::inner_text_from_html;
use pretty_assertions::assert_eq;

#[test]
fn drops_display_none_content() {
    let html = "<div>Hello <span style='display: none;'>Secret</span> World</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "HelloWorld");
}

#[test]
fn hidden_block_preserves_structure_boundaries() {
    let html = "<div>Left <p style='visibility: hidden;'>Hidden Block</p> Right</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "Left\n\nRight");
}

#[test]
fn ignores_script_even_if_display_block() {
    let html = "<div>Visible <script style='display: block;'>alert(1)</script> End</div>";
    let got = inner_text_from_html(html).expect("must extract text");
    assert_eq!(got, "VisibleEnd");
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
