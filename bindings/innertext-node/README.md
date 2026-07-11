# innertext (Node.js/TypeScript)

Pure Rust WHATWG-compliant `innerText`, `outerText`, and `textContent` extraction from HTML.

## Installation

```bash
npm install innertext
# or
yarn add innertext
# or
pnpm add innertext
```

## Quick Start

### JavaScript

```javascript
const { innerText, outerText, textContent, HtmlDocument } = require('innertext');

const html = `
<div id="root">
    Hello <span style="display:none">hidden</span> World
    <script>console.log("not shown")</script>
</div>
`;

// Function-based API
console.log(innerText(html));      // "Hello World"
console.log(textContent(html));    // "Hello hidden console.log("not shown")"

// Class-based API (for multiple extractions)
const doc = new HtmlDocument(html);
console.log(doc.innerText());      // "Hello World"
console.log(doc.textContent());    // "Hello hidden console.log("not shown")"
```

### TypeScript

```typescript
import { innerText, HtmlDocument, textContent } from 'innertext';

const html: string = '<div>Hello World</div>';
const text: string = innerText(html);

const doc = new HtmlDocument(html);
console.log(doc.innerText());
```

## API

### Functions

#### `innerText(html: string): string`

Extract innerText from HTML string. Implements the [WHATWG innerText algorithm](https://html.spec.whatwg.org/multipage/dom.html#the-innertext-idl-attribute):
- Respects `display` CSS property (skips `display:none`)
- Respects `visibility` CSS property
- Respects `white-space` CSS property (normal, pre, pre-line, pre-wrap)
- Respects `text-transform` CSS property
- Handles replaced elements (textarea, input, img)
- Converts `<br>` tags to newlines

#### `outerText(html: string): string`

Extract outerText from HTML string. Per WHATWG spec, outerText getter is identical to innerText getter.

#### `textContent(html: string): string`

Extract textContent from HTML string. Performs CSS-blind structural walk:
- Ignores all CSS properties
- Includes `display:none` content
- Includes `<script>` and `<style>` content

### Classes

#### `HtmlDocument`

Parsed HTML document for efficient multiple text extractions.

```typescript
const doc = new HtmlDocument(html);
const rendered = doc.innerText();    // First extraction
const structural = doc.textContent(); // Second extraction (reuses parse)
```

## Performance

- Pure Rust implementation (native bindings via NAPI)
- Zero JavaScript overhead in extraction
- <1ms per document (parsing + extraction)
- Minimal memory overhead (O(n) DOM tree)

## Accuracy

100% Chromium parity on 36+ test cases covering:
- Display and visibility handling
- Whitespace normalization (normal, pre, pre-line)
- Block element newlines
- Table cell/row separators
- Replaced elements (textarea, input, img, button)
- Metadata elements (script, style)
- Unicode and entity handling

## Supported Platforms

- ✅ Linux x64 (glibc, musl)
- ✅ macOS x64
- ✅ macOS ARM64 (Apple Silicon)
- ✅ Windows x64 (MSVC)

## License

MIT
