use crate::model::{Display, Style, TextTransform, WhiteSpace};
pub(crate) fn derive_style(tag: &str, inline_style: Option<&str>, parent: &Style) -> Style {
    let mut style = Style {
        display: default_display(tag),
        visibility_visible: parent.visibility_visible,
        white_space: parent.white_space.clone(),
        text_transform: parent.text_transform.clone(),
    };

    for declaration in inline_style.unwrap_or_default().split(';') {
        let mut parts = declaration.splitn(2, ':');
        let key = parts.next().unwrap_or_default().trim().to_ascii_lowercase();
        let value = parts.next().unwrap_or_default().trim().to_ascii_lowercase();
        match key.as_str() {
            "display" => {
                style.display = match value.as_str() {
                    "none" => Display::None,
                    "block" | "flex" | "grid" | "list-item" => Display::Block,
                    "table" => Display::Table,
                    "table-row" => Display::TableRow,
                    "table-cell" => Display::TableCell,
                    "table-caption" => Display::TableCaption,
                    _ => Display::Inline,
                };
            }
            "visibility" => {
                style.visibility_visible = value != "hidden" && value != "collapse";
            }
            "white-space" => {
                style.white_space = match value.as_str() {
                    "pre" => WhiteSpace::Pre,
                    "pre-line" => WhiteSpace::PreLine,
                    "pre-wrap" => WhiteSpace::PreWrap,
                    _ => WhiteSpace::Normal,
                };
            }
            "text-transform" => {
                style.text_transform = match value.as_str() {
                    "uppercase" => TextTransform::Uppercase,
                    "lowercase" => TextTransform::Lowercase,
                    _ => TextTransform::None,
                };
            }
            _ => {}
        }
    }

    style
}

pub(crate) fn default_display(tag: &str) -> Display {
    match tag {
        "table" => Display::Table,
        "tr" => Display::TableRow,
        "td" | "th" => Display::TableCell,
        "caption" => Display::TableCaption,
        tag if is_default_block(tag) => Display::Block,
        _ => Display::Inline,
    }
}

fn is_default_block(tag: &str) -> bool {
    const BLOCK_TAGS: &[&str] = &[
        "address", "article", "aside", "blockquote", "details", "dialog", "dd", "div", "dl",
        "dt", "fieldset", "figcaption", "figure", "footer", "form", "h1", "h2", "h3", "h4",
        "h5", "h6", "header", "hr", "li", "main", "nav", "ol", "p", "pre", "section", "ul",
    ];
    BLOCK_TAGS.contains(&tag)
}

pub(crate) fn is_metadata_element(tag: &str) -> bool {
    matches!(
        tag,
        "script" | "style" | "meta" | "link" | "title" | "head" | "noscript"
    )
}

pub(crate) fn is_block_like(display: &Display) -> bool {
    matches!(display, Display::Block | Display::TableCaption)
}

pub(crate) fn apply_white_space(text: &str, white_space: &WhiteSpace) -> String {
    match white_space {
        WhiteSpace::Pre | WhiteSpace::PreWrap => text.to_string(),
        WhiteSpace::PreLine => apply_pre_line(text),
        WhiteSpace::Normal => apply_normal_spaces(text),
    }
}

fn apply_normal_spaces(text: &str) -> String {
    let mut out = String::new();
    let mut in_space = false;
    for ch in text.chars() {
        // U+00A0 (non-breaking space) is preserved as-is, not collapsed
        if ch as u32 == 0xA0 {
            out.push(ch);
            in_space = false;
        } else if ch.is_whitespace() {
            if !in_space {
                out.push(' ');
                in_space = true;
            }
        } else {
            out.push(ch);
            in_space = false;
        }
    }
    out
}

fn apply_pre_line(text: &str) -> String {
    // pre-line: collapse spaces within lines, preserve line breaks, trim lines
    let lines: Vec<&str> = text.split('\n').collect();
    let mut result_lines = Vec::new();
    
    for line in lines {
        let collapsed = {
            let mut out = String::new();
            let mut in_space = false;
            for ch in line.chars() {
                // U+00A0 (non-breaking space) is preserved as-is, not collapsed
                if ch as u32 == 0xA0 {
                    out.push(ch);
                    in_space = false;
                } else if ch.is_whitespace() {
                    if !in_space {
                        out.push(' ');
                        in_space = true;
                    }
                } else {
                    out.push(ch);
                    in_space = false;
                }
            }
            out
        };
        result_lines.push(collapsed.trim().to_string());
    }
    
    result_lines.join("\n")
}

pub(crate) fn apply_text_transform(text: &str, transform: &TextTransform) -> String {
    match transform {
        TextTransform::None => text.to_string(),
        TextTransform::Uppercase => text.to_uppercase(),
        TextTransform::Lowercase => text.to_lowercase(),
    }
}

pub(crate) fn inline_style_attr(attrs: &[markup5ever::Attribute]) -> Option<String> {
    attrs
        .iter()
        .find(|attr| attr.name.local.as_ref().eq_ignore_ascii_case("style"))
        .map(|attr| attr.value.to_string())
}
