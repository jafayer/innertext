# Publishing to PyPI and npm

This project uses GitHub Actions to automate publishing of language bindings to PyPI (Python) and npm (Node.js) registries.

## Release Process

### Prerequisites

1. **PyPI Setup** (one-time):
   - Create a PyPI account at https://pypi.org if you don't have one
   - Go to Account Settings → API tokens
   - Create a new token with scope "Entire account (all projects)"
   - Add the token to your GitHub repository secrets as `PYPI_TOKEN`

2. **npm Setup** (one-time):
   - Create an npm account at https://www.npmjs.com if you don't have one
   - Go to Access Tokens → Generate new token
   - Create a token with "Automation" or "Write" permissions
   - Add the token to your GitHub repository secrets as `NPM_TOKEN`

### Making a Release

1. **Update version in `Cargo.toml`** (workspace root):
   ```toml
   [workspace]
   package.version = "0.2.0"
   ```

2. **Commit the version bump**:
   ```bash
   git add Cargo.toml
   git commit -m "chore: bump version to 0.2.0"
   ```

3. **Create and push a git tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. **Watch the workflow**:
   - Go to GitHub → Actions tab
   - Both `publish-pypi.yml` and `publish-npm.yml` workflows will trigger
   - Monitor their progress
   - When complete, verify packages on PyPI and npm registries

## GitHub Actions Workflows

### `publish-pypi.yml`

Builds Python wheels for 5 platforms and publishes to PyPI:

- **Trigger**: Push tag matching `v*` (e.g., `v0.1.0`)
- **Platforms**:
  - Linux x86_64 (glibc)
  - Linux x86_64 (musl - Alpine)
  - macOS x86_64
  - macOS ARM64 (Apple Silicon)
  - Windows x86_64
- **Tool**: `maturin` (PyO3 build backend)
- **Output**: Wheels uploaded to PyPI
- **Auth**: Uses `PYPI_TOKEN` environment variable
- **Authentication Method**: PyPA trusted publishing (OIDC) - no need to store tokens

### `publish-npm.yml`

Builds native Node.js modules for 5 platforms and publishes to npm:

- **Trigger**: Push tag matching `v*` (e.g., `v0.1.0`)
- **Platforms**: Same 5 platforms as Python
- **Tool**: `@napi-rs/cli` (NAPI-rs build system)
- **Output**: Native modules (.node files) bundled with JavaScript loader
- **Auth**: Uses `NPM_TOKEN` environment variable
- **Registry**: npm public registry

## Setting Up Secrets in GitHub

1. Go to your repository on GitHub
2. Settings → Secrets and variables → Actions
3. Create new repository secrets:
   - **PYPI_TOKEN**: Your PyPI API token
   - **NPM_TOKEN**: Your npm authentication token

## Cross-Platform Build Strategy

### Python (maturin)

Each platform builds on the appropriate GitHub Actions runner:
- Ubuntu runners build Linux (glibc and musl via `manylinux` maturin arg)
- macOS runners build macOS (x86_64 and ARM64 on `macos-14`)
- Windows runners build Windows x86_64

The `maturin build --release` command handles:
- Python version compatibility (3.7-3.13)
- Platform-specific optimizations (LTO, single codegen unit)
- Creating properly-tagged wheels

### Node.js (NAPI-rs)

Similar matrix strategy, building `.node` native modules for each platform. The `@napi-rs/cli` tool handles:
- Node version compatibility (14+)
- Native module exports with proper entry points
- Binary naming conventions for platform detection

## Version Synchronization

All bindings use the **workspace version** from the root `Cargo.toml`:

```toml
[workspace]
package.version = "0.2.0"
```

Each binding's `Cargo.toml` references it:

```toml
# bindings/innertext-python/Cargo.toml
[package]
version.workspace = true
```

Python's `pyproject.toml` and Node's `package.json` use hardcoded versions that must be updated manually alongside the Cargo.toml. Future: Could automate this with a version sync script.

## Troubleshooting

### PyPI publish fails
- Check that `PYPI_TOKEN` is set in repository secrets
- Verify the wheel files were created and uploaded as artifacts
- Check PyPI for any validation errors in the build logs

### npm publish fails
- Check that `NPM_TOKEN` is set and has "publish" permissions
- Verify the .node files were created for all platforms
- Ensure `package.json` version matches the git tag

### Wheel/binary not building for a specific platform
- Check the platform-specific job logs in GitHub Actions
- Common issues:
  - Rust toolchain target not installed (usually auto-handled)
  - Python version compatibility (pyproject.toml classifiers)
  - Node version compatibility (package.json engines field)

## Future Enhancements

- [ ] Automate version syncing across all config files
- [ ] Add GitHub Release notes generation from CHANGELOG
- [ ] Publish Java bindings to Maven Central
- [ ] Add pre-release workflow for testing before publishing
- [ ] Sign released artifacts with GPG
- [ ] Add SBOMs (Software Bill of Materials)
