use std::io::Write;
use std::process::{Command, Stdio};

use innertext_core::inner_text_from_html;

#[derive(Debug)]
struct Case {
    name: &'static str,
    html: &'static str,
    selector: &'static str,
}

fn cases() -> Vec<Case> {
    vec![
        Case {
            name: "display_none_dropped",
            html: "<div id='root'>Hello <span style='display:none'>Secret</span> World</div>",
            selector: "#root",
        },
        Case {
            name: "br_newline",
            html: "<div id='root'>Line One<br>Line Two</div>",
            selector: "#root",
        },
        Case {
            name: "block_boundaries",
            html: "<div id='root'><div>Block A</div><div>Block B</div></div>",
            selector: "#root",
        },
    ]
}

fn chromium_inner_text(html: &str, selector: &str) -> Result<String, String> {
    let mut child = Command::new("node")
        .arg("tools/browser-oracle/oracle.mjs")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to spawn node oracle: {err}"))?;

    let input = format!("{{\"html\":{:?},\"selector\":{:?}}}", html, selector);
    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open stdin for oracle process".to_string())?;
        stdin
            .write_all(input.as_bytes())
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

    let stdout = String::from_utf8_lossy(&output.stdout);
    let marker = "\"innerText\":\"";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("oracle output missing innerText field: {stdout}"))?
        + marker.len();
    let mut escaped = String::new();
    let mut chars = stdout[start..].chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                match next {
                    'n' => escaped.push('\n'),
                    't' => escaped.push('\t'),
                    '"' => escaped.push('"'),
                    '\\' => escaped.push('\\'),
                    other => {
                        escaped.push('\\');
                        escaped.push(other);
                    }
                }
            }
            continue;
        }
        if ch == '"' {
            break;
        }
        escaped.push(ch);
    }

    Ok(escaped)
}

#[test]
#[ignore = "requires Node.js and Playwright (run npm install in tools/browser-oracle)"]
fn matches_chromium_for_seed_cases() {
    for case in cases() {
        let rust = inner_text_from_html(case.html).expect("rust extraction should succeed");
        let browser = chromium_inner_text(case.html, case.selector)
            .unwrap_or_else(|err| panic!("{}: {err}", case.name));
        assert_eq!(rust, browser, "parity mismatch in case {}", case.name);
    }
}
