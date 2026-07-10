// This module has been refactored into:
//   src/css.rs          — style parsing and CSS helpers
//   src/inner_text.rs   — WHATWG rendered-text collection (innerText/outerText getter)
//   src/text_content.rs — DOM descendant-text-content (textContent)
//
// This file is no longer compiled (mod render is absent from lib.rs).

#[allow(dead_code)]
fn _placeholder() {
    let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(input);
    let root_style = Style::root();

    let mut results = Vec::new();
    for child in dom.document.children.borrow().iter() {
        results.extend(rendered_text_collection(child, &root_style));
    }

    Ok(finalize_items(results))
}

fn rendered_text_collection(node: &Handle, inherited_style: &Style) -> Vec<Item> {
    match &node.data {
        NodeData::Document => collect_children(node, inherited_style),
        NodeData::Text { contents } => {
            if !inherited_style.visibility_visible {
                return Vec::new();
            }

            let transformed = apply_text_transform(contents.borrow().as_ref(), &inherited_style.text_transform);
            let normalized = apply_white_space(&transformed, &inherited_style.white_space);
            if normalized.is_empty() {
                Vec::new()
            } else {
                vec![Item::Text(normalized)]
            }
        }
        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.as_ref();
            if is_metadata_element(tag) {
                return Vec::new();
            }

            if tag.eq_ignore_ascii_case("br") {
                return vec![Item::Text("\n".to_string())];
            }

            let inline_style = attrs
                .borrow()
                .iter()
                .find(|attr| attr.name.local.as_ref().eq_ignore_ascii_case("style"))
                .map(|attr| attr.value.to_string());

            let style = derive_style(tag, inline_style.as_deref(), inherited_style);
            if matches!(style.display, Display::None) {
                return Vec::new();
            }

            let mut items = collect_children(node, &style);

            if matches!(style.display, Display::TableCell) {
                if has_next_table_cell(node) {
                    items.push(Item::Text("\t".to_string()));
                }
            }

            if matches!(style.display, Display::TableRow) {
                if has_next_table_row(node) {
                    items.push(Item::Text("\n".to_string()));
                }
            }

            if tag.eq_ignore_ascii_case("p") {
                items.insert(0, Item::RequiredBreak(2));
                items.push(Item::RequiredBreak(2));
            } else if is_block_like(&style.display) {
                items.insert(0, Item::RequiredBreak(1));
                items.push(Item::RequiredBreak(1));
            }

            items
        }
        _ => collect_children(node, inherited_style),
    }
}

fn collect_children(node: &Handle, style: &Style) -> Vec<Item> {
    let mut items = Vec::new();
    for child in node.children.borrow().iter() {
        items.extend(rendered_text_collection(child, style));
    }
    items
}

fn derive_style(tag: &str, inline_style: Option<&str>, parent: &Style) -> Style {
    let mut style = Style {
        display: default_display(tag),
        visibility_visible: parent.visibility_visible,
        white_space: parent.white_space.clone(),
        text_transform: parent.text_transform.clone(),
    };

    let inline_style = inline_style.unwrap_or_default();

    for declaration in inline_style.split(';') {
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
                }
            }
            "visibility" => {
                style.visibility_visible = value != "hidden" && value != "collapse";
            }
            "white-space" => {
                style.white_space = if value.starts_with("pre") {
                    WhiteSpace::Pre
                } else {
                    WhiteSpace::Normal
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

fn finalize_items(mut items: Vec<Item>) -> String {
    items.retain(|item| !matches!(item, Item::Text(text) if text.is_empty()));

    while matches!(items.first(), Some(Item::RequiredBreak(_))) {
        items.remove(0);
    }

    while matches!(items.last(), Some(Item::RequiredBreak(_))) {
        items.pop();
    }

    let mut folded = Vec::new();
    let mut i = 0;
    while i < items.len() {
        match &items[i] {
            Item::RequiredBreak(count) => {
                let mut max_breaks = *count;
                let mut j = i + 1;
                while j < items.len() {
                    match &items[j] {
                        Item::RequiredBreak(next) => {
                            max_breaks = max_breaks.max(*next);
                            j += 1;
                        }
                        _ => break,
                    }
                }
                folded.push(Item::Text("\n".repeat(max_breaks as usize)));
                i = j;
            }
            Item::Text(text) => {
                folded.push(Item::Text(text.clone()));
                i += 1;
            }
        }
    }

    let mut out = String::new();
    for item in folded {
        if let Item::Text(text) = item {
            append_text_with_space_collapse(&mut out, &text);
        }
    }
    out
}

fn append_text_with_space_collapse(out: &mut String, text: &str) {
    if out.is_empty() {
        out.push_str(text);
        return;
    }

    if out.ends_with(' ') && text.starts_with(' ') {
        out.push_str(text.trim_start_matches(' '));
        return;
    }

    out.push_str(text);
}

fn apply_white_space(text: &str, white_space: &WhiteSpace) -> String {
    match white_space {
        WhiteSpace::Pre => text.to_string(),
        WhiteSpace::Normal => {
            let mut out = String::new();
            let mut in_space = false;
            for ch in text.chars() {
                if ch.is_whitespace() {
                    if !in_space {
                        out.push(' ');
                        in_space = true;
                    }
                } else {
                    out.push(ch);
                    in_space = false;
                }
            }
            out.trim().to_string()
        }
    }
}

fn apply_text_transform(text: &str, transform: &TextTransform) -> String {
    match transform {
        TextTransform::None => text.to_string(),
        TextTransform::Uppercase => text.to_uppercase(),
        TextTransform::Lowercase => text.to_lowercase(),
    }
}

fn default_display(tag: &str) -> Display {
    if matches!(tag, "table") {
        Display::Table
    } else if matches!(tag, "tr") {
        Display::TableRow
    } else if matches!(tag, "td" | "th") {
        Display::TableCell
    } else if matches!(tag, "caption") {
        Display::TableCaption
    } else if is_default_block(tag) {
        Display::Block
    } else {
        Display::Inline
    }
}

fn is_default_block(tag: &str) -> bool {
    let block_tags: HashSet<&str> = HashSet::from([
        "address", "article", "aside", "blockquote", "details", "dialog", "dd", "div", "dl",
        "dt", "fieldset", "figcaption", "figure", "footer", "form", "h1", "h2", "h3", "h4",
        "h5", "h6", "header", "hr", "li", "main", "nav", "ol", "p", "pre", "section", "ul",
    ]);
    block_tags.contains(tag)
}

fn is_metadata_element(tag: &str) -> bool {
    matches!(tag, "script" | "style" | "meta" | "link" | "title" | "head" | "noscript")
}

fn is_block_like(display: &Display) -> bool {
    matches!(display, Display::Block | Display::TableCaption)
}

fn has_next_table_cell(node: &Handle) -> bool {
    if let Some(parent) = get_parent(node) {
        let siblings = parent.children.borrow();
        let mut found_current = false;
        for sibling in siblings.iter() {
            if Rc::ptr_eq(sibling, node) {
                found_current = true;
                continue;
            }
            if !found_current {
                continue;
            }
            if let NodeData::Element { name, attrs, .. } = &sibling.data {
                let inline_style = attrs
                    .borrow()
                    .iter()
                    .find(|attr| attr.name.local.as_ref().eq_ignore_ascii_case("style"))
                    .map(|attr| attr.value.to_string());
                let style = derive_style(name.local.as_ref(), inline_style.as_deref(), &Style::root());
                if matches!(style.display, Display::TableCell) {
                    return true;
                }
            }
        }
    }
    false
}

fn has_next_table_row(node: &Handle) -> bool {
    if let Some(parent) = get_parent(node) {
        let siblings = parent.children.borrow();
        let mut found_current = false;
        for sibling in siblings.iter() {
            if Rc::ptr_eq(sibling, node) {
                found_current = true;
                continue;
            }
            if !found_current {
                continue;
            }
            if let NodeData::Element { name, attrs, .. } = &sibling.data {
                let inline_style = attrs
                    .borrow()
                    .iter()
                    .find(|attr| attr.name.local.as_ref().eq_ignore_ascii_case("style"))
                    .map(|attr| attr.value.to_string());
                let style = derive_style(name.local.as_ref(), inline_style.as_deref(), &Style::root());
                if matches!(style.display, Display::TableRow) {
                    return true;
                }
            }
        }
    }
    false
}

fn get_parent(node: &Handle) -> Option<Handle> {
    if let Some(weak) = node.parent.take() {
        let parent = weak.upgrade();
        node.parent.set(Some(weak));
        parent
    } else {
        None
    }
}
