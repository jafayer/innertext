# Maintenance Guide

Common operations and troubleshooting for maintaining the innertext project.

## Daily Development

### Build locally
```bash
cargo build --release
```

### Run all tests
```bash
cargo test --all
```

### Check all bindings compile
```bash
cargo check -p innertext-core -p innertext-python -p innertext-node -p innertext-java
```

### Format and lint
```bash
cargo fmt
cargo clippy --all --all-targets
```

## Making a Release

### Step 1: Prepare changes
```bash
# Ensure all changes are committed
git status

# Pull latest
git pull origin main
```

### Step 2: Update version
```bash
# Update all version numbers across the workspace
python scripts/sync-version.py 0.2.0

# Review changes
git diff

# Commit
git add .
git commit -m "chore: bump version to 0.2.0"
```

### Step 3: Create release tag
```bash
git tag v0.2.0
git push origin v0.2.0
```

### Step 4: Monitor workflows
- Go to GitHub → Actions
- Watch `publish-pypi.yml` and `publish-npm.yml` workflows
- Both should complete in ~10-15 minutes

### Step 5: Verify publishing
- **PyPI**: https://pypi.org/project/innertext/
- **npm**: https://www.npmjs.com/package/innertext

## Setting Up Secrets (One-time)

### PyPI Trusted Publisher (OIDC)
1. Create account at https://pypi.org
2. Go to Account Settings → Publishing
3. Add pending publisher with:
   - Project Name: `innertext`
   - Owner: Your GitHub username
   - Repository: `innertext`
   - Workflow: `publish-pypi.yml`
   - Environment: `pypi`
4. Create GitHub environment: Settings → Environments → New → `pypi`
5. No secrets needed! (OIDC handles authentication)

### npm Token
1. Create account at https://www.npmjs.com
2. Settings → Access Tokens → Generate new token
3. Choose "Automation" or "Write" scope
4. Go to GitHub repo → Settings → Secrets and variables → Actions
5. Create secret `NPM_TOKEN` with the token value

## Troubleshooting

### Workflow fails to start
- Check that tag matches `v*` pattern (e.g., `v0.1.0`, not just `0.1.0`)
- Check that GitHub Actions is enabled in repository settings

### PyPI publish fails
```bash
# Check if version already exists on PyPI
# Try deleting the failed artifacts and retriggering

# Or manually publish locally to test:
cd bindings/innertext-python
pip install maturin
maturin build --release
pip install target/wheels/*.whl
```

### npm publish fails
```bash
# Check npm token is valid and has publish permissions
# Test locally:
cd bindings/innertext-node
npm install
npm run build:release
npm publish --dry-run
```

### Wheel/binary not built for a platform
Check the GitHub Actions job logs for that platform:
- Look in the workflow run
- Find the job for the specific target
- Read build output for error messages

Common issues:
- Rust toolchain target not available
- Python version incompatibility
- Node version mismatch

### Version sync script fails
Ensure:
- Script is in `scripts/sync-version.py`
- All config files exist (Cargo.toml, pyproject.toml, package.json, build.gradle)
- Files have the expected format
- You have write permissions

## Adding a New Language Binding

1. Create binding in `bindings/new-language/`
2. Update `Cargo.toml` workspace members
3. Create build config (e.g., setup.py, build.gradle)
4. Add language-specific workflow in `.github/workflows/`
5. Update `PUBLISHING.md` with publishing instructions
6. Add to version sync script

## Updating Dependencies

### Core dependencies
```bash
cargo update -p <crate-name>
cargo test --all
```

### Python dependencies (PyO3, maturin)
- Update `bindings/innertext-python/Cargo.toml`
- Test: `cargo check -p innertext-python`
- Test wheel build: `cd bindings/innertext-python && maturin build --release`

### Node.js dependencies (NAPI-rs)
- Update `bindings/innertext-node/Cargo.toml`
- Test: `cargo check -p innertext-node`
- Test native build: `cd bindings/innertext-node && npm run build:release`

### Java dependencies (JNI)
- Update `bindings/innertext-java/Cargo.toml`
- Test: `cargo check -p innertext-java`

## Performance Testing

### Measure build time
```bash
# Clear and rebuild from scratch
cargo clean
time cargo build --release
```

### Benchmark text extraction
```bash
cargo test --release -- --nocapture --test-threads=1
```

## Monitoring

### Watch CI status
```bash
# Enable GitHub CLI notifications
gh repo set-default
gh api user/starred
```

### Subscribe to releases
- GitHub repo → Releases → Watch
- This sends notifications for new releases

## Emergency Procedures

### Yank a release (remove from public registry)
```bash
# PyPI
pip install twine
twine upload --skip-existing --repository pypi dist/innertext-*.whl

# Then on PyPI website, "yank" the release

# npm
npm unpublish innertext@0.2.0 --force
```

### Rollback failed release
1. Delete the git tag: `git tag -d v0.2.0 && git push origin :v0.2.0`
2. Revert version bump: `git revert <commit-hash>`
3. Fix the issue
4. Create new tag and retry

## Documentation

- **[README.md](../README.md)** - Project overview
- **[PUBLISHING.md](../PUBLISHING.md)** - Automated release guide
- **[bindings/*/README.md](../bindings/)** - Language-specific docs
