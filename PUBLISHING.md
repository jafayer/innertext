# Publishing to PyPI and npm

This project uses GitHub Actions to automate publishing of language bindings to PyPI (Python) and npm (Node.js) registries.

## Release Process

### Prerequisites

1. **PyPI Setup** (one-time, using Trusted Publishers):
   - Create a PyPI account at https://pypi.org if you don't have one
   - Go to Account Settings → Publishing
   - Add a new **pending publisher** with:
     - **PyPI Project Name**: `innertext`
     - **Owner**: Your GitHub username
     - **Repository name**: `innertext`
     - **Workflow name**: `publish-pypi.yml`
     - **Environment name**: `pypi`
   - No API tokens needed! Uses OpenID Connect (OIDC) for secure, credential-free publishing
   - Once you publish the first release, this becomes a trusted publisher

2. **npm Setup** (one-time):
   - Create an npm account at https://www.npmjs.com if you don't have one
   - Go to Access Tokens → Generate new token
   - Create a token with "Automation" or "Write" permissions
   - Add the token to your GitHub repository secrets as `NPM_TOKEN`
   - (npm doesn't support OIDC yet, so tokens are currently required)

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
- **Authentication Method**: PyPI Trusted Publishers (OpenID Connect / OIDC)
  - No stored tokens needed
  - GitHub Actions generates a time-limited OIDC token
  - PyPI verifies the token and allows publish if trusted publisher is registered
  - More secure and maintainable than API tokens

### `publish-npm.yml`

Builds native Node.js modules for 5 platforms and publishes to npm:

- **Trigger**: Push tag matching `v*` (e.g., `v0.1.0`)
- **Platforms**: Same 5 platforms as Python
- **Tool**: `@napi-rs/cli` (NAPI-rs build system)
- **Output**: Native modules (.node files) bundled with JavaScript loader
- **Auth**: Uses `NPM_TOKEN` environment variable
- **Registry**: npm public registry

## Setting Up GitHub Environment for PyPI Publishing

Create a GitHub Actions environment to enable OIDC trusted publishers:

1. Go to your repository on GitHub
2. Settings → Environments → New environment
3. Name: `pypi`
4. Leave "Deployment branches" as default (allow all)
5. Click "Create environment"
6. No secrets needed! (OIDC handles auth automatically)

## Registering Trusted Publisher on PyPI

This connects PyPI to your GitHub Actions workflow via OIDC:

1. Go to https://pypi.org/manage/account/publishing/
2. Click "Add a pending publisher"
3. Fill in:
   - **PyPI Project Name**: `innertext`
   - **Owner**: Your GitHub username
   - **Repository name**: `innertext` 
   - **Workflow name**: `publish-pypi.yml`
   - **Environment name**: `pypi`
4. Click "Add publisher"
5. First time you run `publish-pypi.yml`, the pending publisher becomes active

## Setting Up npm Token Secret

npm doesn't support OIDC yet, so you need to store a token:

1. Go to your repository on GitHub
2. Settings → Secrets and variables → Actions
3. Create new repository secret:
   - **NPM_TOKEN**: Your npm authentication token (from https://www.npmjs.com/settings/tokens)

## Cross-Platform Build Strategy

### Python (maturin)

Each platform builds on the appropriate GitHub Actions runner:
- Ubuntu runners build Linux (glibc and musl via `manylinux` maturin arg)
- macOS runners build x86_64 wheels/binaries on `macos-13` (Intel) and ARM64 wheels/binaries on `macos-14` (Apple Silicon)
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

### PyPI publish fails with "No trusted publishers found"
- Trusted publisher not registered on PyPI
- Go to https://pypi.org/manage/account/publishing/
- Check that a pending or active publisher is configured for `innertext`
- Verify the environment name in workflow matches `pypi` in PyPI settings

### PyPI publish fails with permission/auth error
- GitHub environment `pypi` not created
- Go to Settings → Environments → New environment → `pypi`
- Ensure the workflow job is using `environment: pypi`

### PyPI publish fails with version already exists
- You're trying to publish a version that already exists
- Increment version in Cargo.toml
- Yank the old release on PyPI if needed (see Emergency Procedures)

### npm publish fails
- Check that `NPM_TOKEN` secret is set in GitHub
- Verify the token has "Automation" or "Write" permissions
- Verify the .node files were created for all platforms
- Ensure `package.json` version matches the git tag

### Wheel/binary not building for a specific platform
- Check the platform-specific job logs in GitHub Actions
- Common issues:
  - Rust toolchain target not installed (usually auto-handled)
  - Python version compatibility (pyproject.toml classifiers)
  - Node version compatibility (package.json engines field)

## Understanding OIDC Trusted Publishers

**OpenID Connect (OIDC)** is a secure authentication method that PyPI recommends for automating releases. Here's why it's better than API tokens:

### Benefits
- **No stored secrets**: GitHub Actions generates temporary tokens that expire after the build
- **Revocation**: Automatically invalidated after each publish (no manual token rotation needed)
- **Audit trail**: PyPI logs which workflow performed the publish
- **Least privilege**: Each publish uses minimum necessary permissions
- **Repo isolation**: Publishing permissions aren't available to all jobs/branches

### How it works
1. GitHub Actions generates an OIDC token signed by GitHub
2. Workflow sends token to PyPI
3. PyPI verifies the token signature and checks trusted publishers
4. If verified, PyPI allows the publish
5. Token is immediately invalidated

No API tokens ever stored in GitHub secrets!

## Future Enhancements

- [ ] npm OIDC support (when available)
- [ ] Automate version syncing across all config files
- [ ] Add GitHub Release notes generation from CHANGELOG
- [ ] Publish Java bindings to Maven Central
- [ ] Add pre-release workflow for testing before publishing
- [ ] Sign released artifacts with GPG
- [ ] Add SBOMs (Software Bill of Materials)
