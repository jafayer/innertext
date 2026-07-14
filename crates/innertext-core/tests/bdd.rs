use cucumber::{given, then, when, World};
use innertext_core::{inner_text_from_html, outer_text_from_html, text_content_from_html};

#[derive(Debug, Default, World)]
pub struct HtmlWorld {
    html: String,
    output: String,
}

/// Converts `\n` and `\t` escape sequences (two chars each) to their actual
/// control characters so Gherkin string parameters match Rust string literals.
fn unescape(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\t", "\t")
}

// ── Given ─────────────────────────────────────────────────────────────────

#[given(expr = "a DOM element {string}")]
fn given_dom_element(world: &mut HtmlWorld, html: String) {
    // Unescape so `\n` in the Gherkin becomes an actual newline in the HTML.
    world.html = unescape(&html);
}

// ── When ──────────────────────────────────────────────────────────────────

#[when("the engine computes the innerText serialization")]
fn when_inner_text(world: &mut HtmlWorld) {
    world.output = inner_text_from_html(&world.html).expect("innerText extraction failed");
}

#[when("the engine computes the outerText serialization")]
fn when_outer_text(world: &mut HtmlWorld) {
    world.output = outer_text_from_html(&world.html).expect("outerText extraction failed");
}

#[when("the engine computes the textContent")]
fn when_text_content(world: &mut HtmlWorld) {
    world.output = text_content_from_html(&world.html).expect("textContent extraction failed");
}

// ── Then ──────────────────────────────────────────────────────────────────

#[then(expr = "the output text must be {string}")]
fn then_output_is(world: &mut HtmlWorld, expected: String) {
    assert_eq!(world.output, unescape(&expected));
}

#[then(expr = "the output text must contain correct line spacing boundaries matching {string}")]
fn then_line_boundaries(world: &mut HtmlWorld, expected: String) {
    assert_eq!(world.output, unescape(&expected));
}

// ── Entry point ───────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    HtmlWorld::run(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../spec/whatwg.gherkin"
    ))
    .await;
}
