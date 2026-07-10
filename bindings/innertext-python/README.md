# innertext (Python)

Pure Rust WHATWG-compliant `innerText`, `outerText`, and `textContent` extraction from HTML.

## Installation

```bash
pip install innertext
```

## Quick Start

```python
import innertext

html = """
<div id="root">
    Hello <span style="display:none">hidden</span> World
    <script>console.log("not shown")</script>
</div>
"""

# Get rendered text (CSS-aware)
print(innertext.inner_text(html))  # "Hello World"

# Get structural text (CSS-blind)
print(innertext.text_content(html))  # "Hello hidden console.log("not shown")"
```

## API

### Functions

#### `inner_text(html: str) -> str`

Extract innerText from HTML string. Implements the [WHATWG innerText algorithm](https://html.spec.whatwg.org/multipage/dom.html#the-innertext-idl-attribute):
- Respects `display` CSS property (skips `display:none`)
- Respects `visibility` CSS property
- Respects `white-space` CSS property (normal, pre, pre-line, pre-wrap)
- Respects `text-transform` CSS property
- Handles replaced elements (textarea, input, img)
- Converts `<br>` tags to newlines

#### `outer_text(html: str) -> str`

Extract outerText from HTML string. Per WHATWG spec, outerText getter is identical to innerText getter.

#### `text_content(html: str) -> str`

Extract textContent from HTML string. Performs CSS-blind structural walk:
- Ignores all CSS properties
- Includes `display:none` content
- Includes `<script>` and `<style>` content

## Accuracy

100% Chromium parity on 36+ test cases covering:
- Display and visibility handling
- Whitespace normalization (normal, pre, pre-line)
- Block element newlines
- Table cell/row separators
- Replaced elements (textarea, input, img, button)
- Metadata elements (script, style)
- Unicode and entity handling

## Performance

- Pure Rust implementation
- Zero external runtime dependencies
- <1ms per document (parsing + extraction)
- Minimal memory overhead (O(n) DOM tree)

## License

MIT
