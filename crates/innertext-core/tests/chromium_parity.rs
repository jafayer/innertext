use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use innertext_core::Document;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CorpusCase {
    id: String,
    html: String,
    selector: String,
    category: String,
}

#[derive(Debug, Deserialize)]
struct BrowserText {
    #[serde(rename = "innerText")]
    inner_text: String,
    #[serde(rename = "outerText")]
    outer_text: String,
    #[serde(rename = "textContent")]
    text_content: String,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    InnerText,
    OuterText,
    TextContent,
}

impl Mode {
    fn label(self) -> &'static str {
        match self {
            Mode::InnerText => "innerText",
            Mode::OuterText => "outerText",
            Mode::TextContent => "textContent",
        }
    }
}

#[derive(Debug)]
struct Gap {
    id: String,
    category: String,
    rust_value: String,
    browser_value: String,
}

fn workspace_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root should exist")
        .to_path_buf()
}

fn corpus_path() -> PathBuf {
    workspace_root().join("tools/browser-oracle/corpus.json")
}

fn load_corpus() -> Vec<CorpusCase> {
    let corpus = fs::read_to_string(corpus_path()).expect("must read parity corpus file");
    serde_json::from_str(&corpus).expect("parity corpus must be valid JSON")
}

fn chromium_text(html: &str, selector: &str) -> Result<BrowserText, String> {
    let mut child = Command::new("node")
        .arg("tools/browser-oracle/oracle.mjs")
        .current_dir(workspace_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to spawn node oracle: {err}"))?;

    let input = serde_json::json!({
        "html": html,
        "selector": selector,
    });

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open stdin for oracle process".to_string())?;
        stdin
            .write_all(input.to_string().as_bytes())
            .map_err(|err| format!("failed to write oracle input: {err}"))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to read oracle output: {err}"))?;

    if !output.status.success() {
        return Err(format!(
            "oracle failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|err| format!("oracle output was not valid JSON: {err}"))
}

fn run_mode(mode: Mode) {
    let mut gaps = Vec::new();

    for case in load_corpus() {
        let doc = Document::parse(&case.html).expect("rust parse should succeed");
        let browser = chromium_text(&case.html, &case.selector)
            .unwrap_or_else(|err| panic!("{} [{}]: {err}", case.id, case.category));

        let (rust_value, browser_value) = match mode {
            Mode::InnerText => (doc.inner_text(), browser.inner_text),
            Mode::OuterText => (doc.outer_text(), browser.outer_text),
            Mode::TextContent => (doc.text_content(), browser.text_content),
        };

        if rust_value != browser_value {
            gaps.push(Gap {
                id: case.id,
                category: case.category,
                rust_value,
                browser_value,
            });
        }
    }

    if !gaps.is_empty() {
        let mut by_category: BTreeMap<String, usize> = BTreeMap::new();
        for gap in &gaps {
            *by_category.entry(gap.category.clone()).or_insert(0) += 1;
        }

        let mut report = String::new();
        let _ = writeln!(
            report,
            "{} parity gaps: {}/{} mismatches",
            mode.label(),
            gaps.len(),
            gaps.len() + (load_corpus().len() - gaps.len())
        );

        let _ = writeln!(report, "category counts:");
        for (category, count) in by_category {
            let _ = writeln!(report, "  - {}: {}", category, count);
        }

        let _ = writeln!(report, "first mismatches:");
        for gap in gaps.iter().take(10) {
            let _ = writeln!(
                report,
                "  - {} [{}] rust={:?} chromium={:?}",
                gap.id, gap.category, gap.rust_value, gap.browser_value
            );
        }

        panic!("{report}");
    }
}

#[test]
#[ignore = "expensive parity suite; requires Node.js + Playwright (npm install in tools/browser-oracle)"]
fn chromium_parity_inner_text_corpus() {
    run_mode(Mode::InnerText);
}

#[test]
#[ignore = "expensive parity suite; requires Node.js + Playwright (npm install in tools/browser-oracle)"]
fn chromium_parity_outer_text_corpus() {
    run_mode(Mode::OuterText);
}

#[test]
#[ignore = "expensive parity suite; requires Node.js + Playwright (npm install in tools/browser-oracle)"]
fn chromium_parity_text_content_corpus() {
    run_mode(Mode::TextContent);
}
