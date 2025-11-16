# Calendar Versioning (CalVer) Scheme

## Overview

PQC Scanner uses **Calendar Versioning (CalVer)** instead of Semantic Versioning (SemVer). CalVer ties version numbers to calendar dates, making it immediately clear when a release was published.

## Format

**`YYYY.MM.MICRO`**

- **YYYY**: Full year (e.g., 2025)
- **MM**: Month (01-12, zero-padded)
- **MICRO**: Incremental release number within that month (0, 1, 2, ...)

### Examples

- `2025.11.0` - First release in November 2025
- `2025.11.1` - Second release in November 2025
- `2025.12.0` - First release in December 2025
- `2026.01.0` - First release in January 2026

## Why CalVer?

### Advantages for PQC Scanner

1. **Immediate Context**: Users can instantly see when a release was published
2. **Security Relevance**: For a security tool tracking quantum-safe cryptography, knowing the release date is crucial for compliance and audit trails
3. **Predictable Cadence**: Clear indication of release frequency and freshness
4. **NIST Alignment**: Compliance reports include timestamps; CalVer makes version-to-date mapping trivial
5. **No Breaking Change Ambiguity**: Unlike SemVer, there's no confusion about major/minor/patch semantics

### When to Increment

- **New Month**: When the calendar month changes, reset MICRO to 0 and update YYYY.MM
- **Within Month**: For each additional release within the same month, increment MICRO

## Tagging Releases

### Git Tags

Tags follow the format: **`vYYYY.MM.MICRO`**

```bash
# First release of November 2025
git tag v2025.11.0

# Second release of November 2025
git tag v2025.11.1

# First release of December 2025
git tag v2025.12.0
```

### Automated Tagging

The release workflow automatically creates tags when triggered via `workflow_dispatch`:

```bash
# Trigger via GitHub CLI
gh workflow run release.yml -f version=2025.11.0

# Or via GitHub UI: Actions → Release Workflow → Run workflow
```

## Releasing

### Manual Release Process

1. **Update Version in Cargo.toml**:
   ```toml
   [package]
   version = "2025.11.0"
   ```

2. **Update CHANGELOG.md**:
   ```markdown
   ## [2025.11.0] - 2025-11-15

   ### Added
   - New quantum-safe algorithm detection
   - Enhanced OSCAL compliance reporting

   ### Fixed
   - Memory leak in parser module
   ```

3. **Commit and Tag**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: Release 2025.11.0"
   git tag v2025.11.0
   git push origin main --tags
   ```

4. **Trigger Release Workflow**:
   ```bash
   gh workflow run release.yml -f version=2025.11.0
   ```

### Automated Release (Recommended)

Use the GitHub Actions workflow:

1. Go to **Actions → Release Workflow**
2. Click **Run workflow**
3. Enter version (e.g., `2025.11.0`)
4. Select **prerelease** if needed
5. Click **Run workflow**

The workflow will:
- Build Rust binaries and WASM packages
- Generate changelog from CHANGELOG.md
- Create GitHub release with artifacts
- Build and push Docker container
- Generate SBOM (Software Bill of Materials)

## Version Comparison

### CalVer vs SemVer

| Aspect | CalVer | SemVer |
|--------|--------|--------|
| Format | `2025.11.0` | `1.2.3` |
| Meaning | November 2025, 1st release | Major.Minor.Patch |
| Breaking Changes | Not indicated | Major version bump |
| Time Context | Built-in | None |
| Security Tool | ✅ Excellent | ❌ Less relevant |
| NIST Compliance | ✅ Clear audit trail | ❌ Requires mapping |

### Migration from SemVer

If migrating from SemVer (e.g., `0.1.0`), the first CalVer release should reflect the current month:

```
0.1.0 (SemVer) → 2025.11.0 (CalVer)
```

## Docker Image Tags

Container images use both CalVer and `latest`:

```bash
# Specific version
ghcr.io/arcqubit/pqc-scanner:2025.11.0

# Latest release
ghcr.io/arcqubit/pqc-scanner:latest
```

## NPM Package Versions

While NPM uses SemVer in `package.json`, the WASM package versions align with CalVer:

```json
{
  "name": "@arcqubit/pqc-scanner",
  "version": "2025.11.0"
}
```

NPM accepts CalVer format as it follows the `MAJOR.MINOR.PATCH` structure.

## Best Practices

### 1. **Monthly Major Releases**

Aim for one major release per month (`.0` release) with hotfixes as needed:

- `2025.11.0` - Major November release
- `2025.11.1` - Critical bug fix
- `2025.11.2` - Security patch
- `2025.12.0` - Major December release

### 2. **CHANGELOG Discipline**

Always update `CHANGELOG.md` before releasing:

```markdown
## [2025.11.0] - 2025-11-15

### Added
- Feature 1
- Feature 2

### Changed
- Improvement 1

### Fixed
- Bug 1
- Bug 2

### Security
- CVE-2025-XXXX fix
```

### 3. **Breaking Changes**

Document breaking changes prominently in CHANGELOG:

```markdown
## [2025.11.0] - 2025-11-15

### ⚠️ BREAKING CHANGES
- API endpoint `/v1/scan` renamed to `/v2/scan`
- Deprecated `analyze()` function removed
```

### 4. **Pre-releases**

Use GitHub pre-release flag for beta/RC versions:

```bash
gh workflow run release.yml -f version=2025.11.0 -f prerelease=true
```

Tag format for pre-releases:
```
v2025.11.0-beta.1
v2025.11.0-rc.1
```

## Tools and Scripts

### Version Bumper Script

```bash
#!/bin/bash
# scripts/bump-version.sh

YEAR=$(date +%Y)
MONTH=$(date +%m)

# Get current version from Cargo.toml
CURRENT=$(grep '^version' Cargo.toml | cut -d'"' -f2)
echo "Current version: $CURRENT"

# Calculate new version
CURRENT_MONTH=$(echo $CURRENT | cut -d'.' -f2)
CURRENT_MICRO=$(echo $CURRENT | cut -d'.' -f3)

if [ "$MONTH" = "$CURRENT_MONTH" ]; then
    # Same month, increment MICRO
    NEW_MICRO=$((CURRENT_MICRO + 1))
    NEW_VERSION="$YEAR.$MONTH.$NEW_MICRO"
else
    # New month, reset MICRO
    NEW_VERSION="$YEAR.$MONTH.0"
fi

echo "New version: $NEW_VERSION"

# Update Cargo.toml
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

echo "✓ Updated Cargo.toml to $NEW_VERSION"
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Ensure version in Cargo.toml is valid CalVer
VERSION=$(grep '^version' Cargo.toml | cut -d'"' -f2)

if ! echo "$VERSION" | grep -qE '^[0-9]{4}\.[0-9]{2}\.[0-9]+$'; then
    echo "Error: Invalid CalVer format in Cargo.toml"
    echo "Expected: YYYY.MM.MICRO (e.g., 2025.11.0)"
    echo "Found: $VERSION"
    exit 1
fi

echo "✓ Version $VERSION is valid CalVer"
```

## References

- [CalVer Specification](https://calver.org/)
- [Calendar Versioning Wikipedia](https://en.wikipedia.org/wiki/Software_versioning#Calendar_versioning)
- [Ubuntu's CalVer Scheme](https://ubuntu.com/about/release-cycle) (YY.MM format)
- [Certifi's CalVer Scheme](https://github.com/certifi/python-certifi) (YYYY.MM.DD format)

## FAQs

**Q: What if we need multiple releases in a day?**
A: Increment the MICRO version. CalVer doesn't mandate day-level granularity for YYYY.MM.MICRO format.

**Q: How do we indicate breaking changes?**
A: Document them prominently in CHANGELOG.md and release notes. Consider major releases at month boundaries.

**Q: Can we use pre-release identifiers?**
A: Yes, append them after the version: `2025.11.0-beta.1`, `2025.11.0-rc.2`

**Q: Is CalVer compatible with Cargo/NPM?**
A: Yes, both package managers accept CalVer format as it follows numeric versioning.

**Q: What about backward compatibility?**
A: Document compatibility in CHANGELOG.md. The MICRO version can indicate minor fixes vs. significant changes.

---

**Adopted**: 2025-11-15
**Format**: YYYY.MM.MICRO
**Current Version**: 2025.11.0
