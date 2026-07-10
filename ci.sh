#!/usr/bin/env bash
#
# ci.sh - Run the same checks as GitHub Actions CI locally.
#
# Usage:
#   ./ci.sh           # Run all checks
#   ./ci.sh --fast    # Skip slow steps (tests already cached)
#
# Exit code 0 = all checks passed, safe to push.

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

pass() { echo -e "${GREEN}✓ $1${NC}"; }
fail() { echo -e "${RED}✗ $1${NC}"; exit 1; }
step() { echo -e "${YELLOW}── $1${NC}"; }

cd "$(dirname "$0")"

# ---------------------------------------------------------------------------
# 1. Format check (matches CI lint job)
# ---------------------------------------------------------------------------
step "Format check"
if ! cargo fmt -- --check; then
    echo ""
    echo "Run 'cargo fmt' to fix formatting, then re-run this script."
    fail "Formatting issues found"
fi
pass "Formatting"

# ---------------------------------------------------------------------------
# 2. Core tests
# ---------------------------------------------------------------------------
step "Core tests"
cargo test -p innertext-core --all 2>&1 | tail -5
pass "Core tests"

# ---------------------------------------------------------------------------
# 3. Clippy — root workspace (standard crates)
# ---------------------------------------------------------------------------
step "Clippy (workspace, excluding bindings)"
cargo clippy \
    -p innertext-core \
    -p innertext-node \
    -p innertext-java \
    --all-targets -- -D warnings
pass "Clippy (core + node + java)"

# ---------------------------------------------------------------------------
# 4. Clippy — Python binding with pyo3/extension-module activated
#    This is the flag maturin passes during wheel builds and is required
#    to surface PyO3-specific lints that won't appear without it.
# ---------------------------------------------------------------------------
step "Clippy (innertext-python with pyo3/extension-module)"
cargo clippy \
    -p innertext-python \
    --features pyo3/extension-module \
    --all-targets -- -D warnings
pass "Clippy (python binding)"

echo ""
echo -e "${GREEN}All checks passed. Safe to push.${NC}"
