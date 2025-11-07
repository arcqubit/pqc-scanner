# Release Checklist

Use this checklist when preparing a new release of the ArcQubit PQC Scanner.

## Pre-Release

### Code Quality

- [ ] All tests passing (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] Benchmarks run successfully (`cargo bench`)
- [ ] Documentation builds (`cargo doc --no-deps`)

### Version Management

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `package.json`
- [ ] Update version in `pkg-web/package.json`
- [ ] Update version in `pkg-nodejs/package.json`
- [ ] Update CHANGELOG.md with release notes
- [ ] Commit version changes

### Build Verification

- [ ] Clean build succeeds (`cargo clean && cargo build --release`)
- [ ] WASM builds for all targets:
  - [ ] Bundler: `npm run build:bundler`
  - [ ] Node.js: `npm run build:nodejs`
  - [ ] Web: `npm run build:web`
- [ ] Optimized builds succeed (`npm run build:release`)
- [ ] Package size under limits:
  - [ ] Bundler gzipped < 350KB
  - [ ] Node.js gzipped < 350KB
  - [ ] Web gzipped < 350KB

### Testing

- [ ] Unit tests pass (`cargo test --lib`)
- [ ] Integration tests pass (`cargo test --test '*'`)
- [ ] Benchmark tests pass (`cargo test --benches`)
- [ ] Example runs successfully:
  ```bash
  cargo run --example generate_compliance_report
  ```
- [ ] Node.js test passes:
  ```bash
  node test/node-test.js
  ```

### Documentation

- [ ] README.md updated with new features
- [ ] CHANGELOG.md has entry for this version
- [ ] API documentation updated (if changed)
- [ ] Examples updated (if API changed)
- [ ] Migration guide included (if breaking changes)

## Release Process

### 1. Create Release Tag

```bash
# Ensure on main branch
git checkout main
git pull origin main

# Create annotated tag
git tag -a v1.2.0 -m "Release 1.2.0: [Brief description]"

# Push tag
git push origin v1.2.0
```

### 2. GitHub Release (Automatic via Actions)

Wait for GitHub Actions to complete:
- [ ] Build workflow completes successfully
- [ ] Tests pass
- [ ] Artifacts generated
- [ ] GitHub release created
- [ ] Docker image built and pushed
- [ ] SBOM generated

### 3. NPM Publish (Automatic via Actions)

Verify NPM publication:
- [ ] Package published to NPM
- [ ] Provenance attestation included
- [ ] All WASM targets included in package
- [ ] TypeScript definitions present

Manual verification:
```bash
npm info @arcqubit/pqc-scanner
npm view @arcqubit/pqc-scanner versions
```

### 4. Container Verification

Check Docker image:
```bash
# Pull latest
docker pull ghcr.io/arcqubit/pqc-scanner:latest
docker pull ghcr.io/arcqubit/pqc-scanner:1.2.0

# Verify image
docker run --rm ghcr.io/arcqubit/pqc-scanner:1.2.0 --help

# Check image size
docker images ghcr.io/arcqubit/pqc-scanner
```

## Post-Release

### Verification

- [ ] NPM package installs correctly:
  ```bash
  npm install -g @arcqubit/pqc-scanner@latest
  ```
- [ ] GitHub Action works in test repository
- [ ] Docker container runs successfully
- [ ] Documentation links work
- [ ] Release notes readable on GitHub

### Communication

- [ ] Update homepage/marketing site
- [ ] Post announcement (if major release):
  - [ ] Blog post
  - [ ] Twitter/X
  - [ ] LinkedIn
  - [ ] Reddit (r/rust, r/programming)
- [ ] Notify users of breaking changes (if any)
- [ ] Update roadmap

### Monitoring

- [ ] Monitor NPM download stats
- [ ] Check for user issues on GitHub
- [ ] Review crash reports (if any)
- [ ] Monitor container pull metrics

## Emergency Rollback

If critical issues are discovered:

### NPM Rollback

```bash
# Deprecate broken version
npm deprecate @arcqubit/pqc-scanner@1.2.0 "Critical bug, use 1.1.0"

# Or unpublish (within 72 hours)
npm unpublish @arcqubit/pqc-scanner@1.2.0
```

### GitHub Release

```bash
# Delete tag
git tag -d v1.2.0
git push origin :refs/tags/v1.2.0

# Delete GitHub release via UI or API
gh release delete v1.2.0
```

### Docker Image

```bash
# Delete container image tags
gh api -X DELETE /orgs/arcqubit/packages/container/pqc-scanner/versions/{VERSION_ID}
```

## Version Numbering

Follow Semantic Versioning (semver):

- **Major (X.0.0)**: Breaking API changes
- **Minor (1.X.0)**: New features, backward compatible
- **Patch (1.2.X)**: Bug fixes, backward compatible

Examples:
- New detection pattern: `1.3.0` (minor)
- Fix false positive: `1.2.1` (patch)
- Change API signature: `2.0.0` (major)

## Release Schedule

- **Patch releases**: As needed for critical bugs
- **Minor releases**: Monthly or bi-monthly
- **Major releases**: Quarterly or as needed

## Release Notes Template

```markdown
## [1.2.0] - 2025-11-06

### Added
- New SPHINCS+ signature detection
- Support for Kotlin language
- Auto-remediation for ECDH curves

### Changed
- Improved regex performance (15% faster)
- Updated NIST compliance mapping

### Fixed
- False positive in Go crypto imports
- Memory leak in large file scanning
- TypeScript definition errors

### Security
- Updated dependencies for CVE-2024-XXXX

### Deprecated
- Old `analyze()` function (use `audit_code()`)

### Removed
- Support for Node.js 14 (EOL)

### Breaking Changes
- None (minor release)
```

## Checklist Completion

Once all items are checked:

- [ ] Release is live and verified
- [ ] All automation succeeded
- [ ] Documentation updated
- [ ] Users notified (if applicable)
- [ ] Monitoring in place
- [ ] This checklist filed with release artifacts

---

**Prepared by**: ArcQubit Team
**Last Updated**: 2025-11-06
**Version**: 1.0
