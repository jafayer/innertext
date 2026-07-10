# First-Time Release Setup Checklist

Complete these steps once to enable automated publishing:

## ✅ PyPI (Python) - OIDC Trusted Publishers

### 1. Create GitHub Actions Environment
```
Repository Settings → Environments → New environment
Name: pypi
Leave "Deployment branches and secrets" as default
```

### 2. Register Trusted Publisher on PyPI
```
https://pypi.org/manage/account/publishing/
  → Add pending publisher
  → PyPI Project Name: innertext
  → Owner: <your-github-username>
  → Repository name: innertext
  → Workflow name: publish-pypi.yml
  → Environment name: pypi
```

**That's it for PyPI!** No secrets to manage. OIDC handles authentication automatically.

## ✅ npm (Node.js) - API Token

### 1. Create npm Access Token
```
https://www.npmjs.com/settings/tokens
  → Generate new token
  → Type: Automation or Write permissions
  → Copy the token
```

### 2. Add GitHub Secret
```
Repository Settings → Secrets and variables → Actions
  → New repository secret
  → Name: NPM_TOKEN
  → Value: <paste-token-from-above>
```

## ✅ Make First Release

```bash
# 1. Bump version
python scripts/sync-version.py 0.2.0

# 2. Review and commit
git diff
git add .
git commit -m "chore: bump version to 0.2.0"

# 3. Create tag and push
git tag v0.2.0
git push origin v0.2.0

# 4. Monitor workflows
# Go to GitHub → Actions
# Wait for both publish workflows to complete
```

## ✅ Verify Publishing

- **PyPI**: https://pypi.org/project/innertext/
- **npm**: https://www.npmjs.com/package/innertext

---

**Questions?** See [PUBLISHING.md](PUBLISHING.md) for full documentation.
