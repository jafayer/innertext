/// WHATWG "get the text steps" / "rendered text collection steps"
/// https://html.spec.whatwg.org/multipage/dom.html#the-innertext-idl-attribute
use std::rc::Rc;

use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::css;
use crate::model::{Display, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
enum RenderToken {
    Text(String),
    RequiredBreak(u8),
    CollapseBoundary,
}

pub(crate) fn extract(dom: &RcDom) -> String {
    let root_style = Style::root();
    let mut tokens = Vec::new();
    for child in dom.document.children.borrow().iter() {
        tokens.extend(rendered_text_collection(child, &root_style));
    }

    let normalized = normalize_tokens(tokens);
    let normalized = drop_inter_block_separator_spaces(normalized);
    let folded = fold_required_break_runs(normalized);
    serialize_tokens(folded)
}

fn rendered_text_collection(node: &Handle, inherited_style: &Style) -> Vec<RenderToken> {
    match &node.data {
        NodeData::Document => collect_children(node, inherited_style),
        NodeData::Text { contents } => {
            if !inherited_style.visibility_visible {
                return Vec::new();
            }

            let raw = contents.borrow();
            let transformed =
                css::apply_text_transform(raw.as_ref(), &inherited_style.text_transform);
            let normalized = css::apply_white_space(&transformed, &inherited_style.white_space);

            if normalized.is_empty() {
                Vec::new()
            } else {
                vec![RenderToken::Text(normalized)]
            }
        }
        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.as_ref();

            if css::is_metadata_element(tag) {
                return vec![RenderToken::CollapseBoundary];
            }

            if tag.eq_ignore_ascii_case("br") {
                return vec![RenderToken::Text("\n".to_string())];
            }

            // Replaced elements: textarea, input, and img are skipped for innerText
            // These form controls and replaced content don't contribute rendered text
            if tag.eq_ignore_ascii_case("textarea")
                || tag.eq_ignore_ascii_case("input")
                || tag.eq_ignore_ascii_case("img")
            {
                // These are replaced elements that don't expose their content in innerText
                return Vec::new();
            }

            let inline_style = css::inline_style_attr(&attrs.borrow());
            let style = css::derive_style(tag, inline_style.as_deref(), inherited_style);

            if matches!(style.display, Display::None) {
                return vec![RenderToken::CollapseBoundary];
            }

            let mut tokens = collect_children(node, &style);

            if !style.visibility_visible {
                return tokens;
            }

            if matches!(style.display, Display::TableCell)
                && has_next_sibling_of_display(node, Display::TableCell)
            {
                tokens.push(RenderToken::Text("\t".to_string()));
            }

            if matches!(style.display, Display::TableRow)
                && has_next_sibling_of_display(node, Display::TableRow)
            {
                tokens.push(RenderToken::Text("\n".to_string()));
            }

            if tag.eq_ignore_ascii_case("p") {
                tokens.insert(0, RenderToken::RequiredBreak(2));
                tokens.push(RenderToken::RequiredBreak(2));
            } else if css::is_block_like(&style.display) {
                tokens.insert(0, RenderToken::RequiredBreak(1));
                tokens.push(RenderToken::RequiredBreak(1));
            }

            tokens
        }
        _ => collect_children(node, inherited_style),
    }
}

fn collect_children(node: &Handle, style: &Style) -> Vec<RenderToken> {
    node.children
        .borrow()
        .iter()
        .flat_map(|child| rendered_text_collection(child, style))
        .collect()
}

fn normalize_tokens(mut tokens: Vec<RenderToken>) -> Vec<RenderToken> {
    tokens.retain(|token| !matches!(token, RenderToken::Text(t) if t.is_empty()));

    while matches!(tokens.first(), Some(RenderToken::CollapseBoundary)) {
        tokens.remove(0);
    }
    while matches!(tokens.last(), Some(RenderToken::CollapseBoundary)) {
        tokens.pop();
    }

    while matches!(tokens.first(), Some(RenderToken::RequiredBreak(_))) {
        tokens.remove(0);
    }
    while matches!(tokens.last(), Some(RenderToken::RequiredBreak(_))) {
        tokens.pop();
    }

    tokens
}

fn drop_inter_block_separator_spaces(tokens: Vec<RenderToken>) -> Vec<RenderToken> {
    let mut out = Vec::with_capacity(tokens.len());

    for i in 0..tokens.len() {
        if let RenderToken::Text(text) = &tokens[i] {
            let prev_is_break =
                i > 0 && matches!(tokens.get(i - 1), Some(RenderToken::RequiredBreak(_)));
            let next_is_break = matches!(tokens.get(i + 1), Some(RenderToken::RequiredBreak(_)));
            if text.chars().all(|ch| ch == ' ') && prev_is_break && next_is_break {
                continue;
            }
        }
        out.push(tokens[i].clone());
    }

    out
}

fn fold_required_break_runs(tokens: Vec<RenderToken>) -> Vec<RenderToken> {
    let mut out = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            RenderToken::RequiredBreak(count) => {
                let mut max = *count;
                let mut j = i + 1;
                while let Some(RenderToken::RequiredBreak(next)) = tokens.get(j) {
                    max = max.max(*next);
                    j += 1;
                }
                out.push(RenderToken::Text("\n".repeat(max as usize)));
                i = j;
            }
            token => {
                out.push(token.clone());
                i += 1;
            }
        }
    }

    out
}

/// Serializes normalized tokens while applying boundary-aware whitespace joins.
fn serialize_tokens(tokens: Vec<RenderToken>) -> String {
    let mut out = String::new();
    let mut collapse_next_leading_space = false;

    for token in tokens {
        match token {
            RenderToken::Text(text) => {
                if collapse_next_leading_space && text.starts_with(' ') {
                    out.push_str(text.trim_start_matches(' '));
                } else {
                    out.push_str(&text);
                }
                collapse_next_leading_space = false;
            }
            RenderToken::CollapseBoundary => {
                if out.ends_with(' ') {
                    collapse_next_leading_space = true;
                }
            }
            RenderToken::RequiredBreak(_) => {
                // Required breaks should be folded before this phase.
            }
        }
    }

    normalize_single_edge_spaces(out)
}

fn normalize_single_edge_spaces(mut out: String) -> String {
    if out.starts_with(' ') && !out.starts_with("  ") {
        out.remove(0);
    }
    if out.ends_with(' ') && !out.ends_with("  ") {
        out.pop();
    }
    out
}

fn has_next_sibling_of_display(node: &Handle, target: Display) -> bool {
    if let Some(parent) = get_parent(node) {
        let siblings = parent.children.borrow();
        let mut found = false;

        for sibling in siblings.iter() {
            if Rc::ptr_eq(sibling, node) {
                found = true;
                continue;
            }
            if !found {
                continue;
            }

            if let NodeData::Element { name, attrs, .. } = &sibling.data {
                let inline_style = css::inline_style_attr(&attrs.borrow());
                let style =
                    css::derive_style(name.local.as_ref(), inline_style.as_deref(), &Style::root());
                if style.display == target {
                    return true;
                }
            }
        }
    }

    false
}

fn get_parent(node: &Handle) -> Option<Handle> {
    let weak = node.parent.take()?;
    let parent = weak.upgrade();
    node.parent.set(Some(weak));
    parent
}
