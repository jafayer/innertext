# innertext

Pure Rust WHATWG-compliant `innerText`, `outerText`, and `textContent` extraction from HTML with language bindings for Python, Node.js, and Java.

## Features

- ✅ **100% Chromium parity** - 36+ test cases covering all edge cases
- ✅ **WHATWG spec compliant** - Exact algorithm implementation
- ✅ **CSS-aware** - Respects `display`, `visibility`, `white-space`, `text-transform`
- ✅ **Fast** - Pure Rust, <1ms per document
- ✅ **Multi-language** - Python, Node.js, Java bindings
- ✅ **Zero dependencies** - Minimal runtime footprint

## Quick Links

- **[PUBLISHING.md](PUBLISHING.md)** - How to set up automated releases with GitHub Actions
- **[Python binding](bindings/innertext-python/README.md)** - Install with `pip install innertext`
- **[Node.js binding](bindings/innertext-node/README.md)** - Install with `npm install innertext`
- **[Java binding](bindings/innertext-java/README.md)** - Add to Maven/Gradle

## Installation

### Python
```bash
pip install innertext
```

### Node.js / TypeScript
```bash
npm install innertext
```

### Java
Maven:
```xml
<dependency>
    <groupId>com.innertext</groupId>
    <artifactId>innertext</artifactId>
    <version>0.1.0</version>
</dependency>
```

## Examples

### Python
```python
import innertext

html = "<div>Hello <span style='display:none'>hidden</span> World</div>"
print(innertext.inner_text(html))  # "Hello World"
```

### JavaScript / TypeScript
```typescript
import { innerText } from 'innertext';

const html = "<div>Hello <span style='display:none'>hidden</span> World</div>";
console.log(innerText(html));  // "Hello World"
```

### Java
```java
import com.innertext.InnerText;

String html = "<div>Hello <span style='display:none'>hidden</span> World</div>";
System.out.println(InnerText.innerText(html));  // "Hello World"
```

## Project Structure

```
.
├── crates/
│   └── innertext-core/          # Core Rust library
├── bindings/
│   ├── innertext-python/        # Python binding (PyO3)
│   ├── innertext-node/          # Node.js binding (NAPI-rs)
│   └── innertext-java/          # Java binding (JNI)
├── .github/
│   └── workflows/               # CI/CD pipelines
│       ├── test.yml             # Test on PR/push
│       ├── publish-pypi.yml     # Publish to PyPI
│       └── publish-npm.yml      # Publish to npm
├── scripts/
│   └── sync-version.py          # Version synchronization
└── PUBLISHING.md                # Release guide
```

## Development

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test --all
```

### Checking bindings
```bash
cargo check -p innertext-python
cargo check -p innertext-node
cargo check -p innertext-java
```

## Releasing

1. Update version using the sync script:
   ```bash
   python scripts/sync-version.py 0.2.0
   ```

2. Commit and tag:
   ```bash
   git add .
   git commit -m "chore: bump version to 0.2.0"
   git tag v0.2.0
   git push origin v0.2.0
   ```

3. GitHub Actions will automatically build and publish to PyPI and npm

For detailed instructions, see [PUBLISHING.md](PUBLISHING.md).

## Testing Strategy

- **Unit tests**: Core algorithm correctness
- **Spec parity tests**: 36 test cases vs Chromium
- **Behavior tests**: Edge case coverage
- **CI/CD tests**: Pre-commit validation on all platforms

## Architecture

### Core Algorithm

The `innertext` algorithm is implemented as a 4-stage token pipeline:

1. **Collection** - DOM walk with CSS-aware filtering
2. **Normalization** - Trim empty tokens, handle boundaries
3. **Folding** - Merge adjacent newlines, enforce max count
4. **Serialization** - Join with context-aware space collapsing

### Bindings

Each language binding exposes three main functions:

- `innerText(html)` - Rendered text (CSS-aware)
- `outerText(html)` - Outer text (identical to innerText getter)
- `textContent(html)` - Structural text (CSS-blind)

Plus an `HtmlDocument` class for efficient multiple extractions from the same document.

## License

MIT

## Contributing

Contributions welcome! Please:

1. Run `cargo fmt` before committing
2. Run `cargo clippy` to check for issues
3. Add tests for new functionality
4. Update binding documentation if needed
