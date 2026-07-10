/// DOM Living Standard "descendant text content"
/// https://dom.spec.whatwg.org/#concept-descendant-text-content
///
/// Pure structural walk: concatenates the data of every Text node descendant
/// in tree order.  No CSS involvement — display:none, visibility:hidden,
/// script contents, etc. are all included.
use markup5ever_rcdom::{Handle, NodeData, RcDom};

pub(crate) fn extract(dom: &RcDom) -> String {
    let mut out = String::new();
    for child in dom.document.children.borrow().iter() {
        collect(child, &mut out);
    }
    out
}

fn collect(node: &Handle, out: &mut String) {
    match &node.data {
        NodeData::Text { contents } => {
            out.push_str(contents.borrow().as_ref());
        }
        _ => {
            for child in node.children.borrow().iter() {
                collect(child, out);
            }
        }
    }
}
