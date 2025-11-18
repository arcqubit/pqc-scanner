# Release Process

This project uses [Release Please](https://github.com/googleapis/release-please) for automated versioning and releases.

## Overview

Release Please automates the entire release process:
1. Analyzes commit messages since the last release
2. Determines the next version number
3. Generates/updates CHANGELOG.md
4. Creates a release PR with version bumps
5. Creates a GitHub release when the PR is merged

## How It Works

### 1. Commit Format (Conventional Commits)

All commits to `main` must follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat:` - New feature (triggers version bump)
- `fix:` - Bug fix (triggers version bump)
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `test:` - Adding/updating tests
- `build:` - Build system changes
- `ci:` - CI/CD changes
- `chore:` - Maintenance tasks
- `revert:` - Reverting changes

**Examples:**
```bash
feat(scanner): add support for Kyber-1024 detection
fix(mcp): resolve memory leak in server initialization
docs: update installation instructions
chore(deps): update thiserror to 2.0
```

### 2. Pull Request Validation

When you create a PR, the `conventional-commits.yml` workflow validates the PR title format. Your PR title must follow the same conventional format as commits.

**Valid PR Titles:**
```
feat: add quantum-resistant cipher detection
fix: resolve false positives in RSA scanning
docs: improve API documentation
```

**Invalid PR Titles:**
```
Add new feature           ❌ (missing type)
FIX: bug in scanner       ❌ (type must be lowercase)
feat - add support        ❌ (wrong separator, use colon)
```

### 3. Automated Release PR Creation

When commits are merged to `main`, Release Please automatically:

1. **Analyzes Commits**: Scans all commits since the last release
2. **Determines Version**: Calculates the next version based on commit types
3. **Generates Changelog**: Creates entries from commit messages
4. **Updates Files**: Bumps version in:
   - `Cargo.toml`
   - `mcp/package.json`
   - `.release-please-manifest.json`
5. **Creates PR**: Opens a release PR with all changes

The PR will be titled like: `chore(main): release 2025.11.1`

### 4. Review and Merge

1. Review the automatically created release PR
2. Check the CHANGELOG.md for accuracy
3. Verify version bumps in `Cargo.toml` and `package.json`
4. Merge the PR when ready to release

### 5. Automatic Release Creation

When the release PR is merged:

1. **Tag Created**: Git tag `v{version}` is automatically created
2. **Release Workflow Triggered**: `.github/workflows/release.yml` runs
3. **Artifacts Built**:
   - Rust binaries (Linux x86_64)
   - WASM bundles (bundler, nodejs, web)
   - Docker images (multi-arch)
   - SBOM files (SPDX, CycloneDX)
4. **Signatures Generated**: Sigstore signatures for all artifacts
5. **SLSA Provenance**: Supply chain attestation generated
6. **GitHub Release**: Published with all artifacts and changelog

## Manual Operations

### Creating a Pre-release

To create a pre-release (e.g., `2025.11.1-beta.1`):

1. Edit `.release-please-manifest.json`:
   ```json
   {
     ".": "2025.11.1-beta.1"
   }
   ```

2. Commit and push to main:
   ```bash
   git add .release-please-manifest.json
   git commit -m "chore: prepare beta release 2025.11.1-beta.1"
   git push origin main
   ```

3. Release Please will create a PR with the pre-release version

### Force a Specific Version

To override the calculated version:

1. Create an empty commit with the desired version in the PR title:
   ```bash
   git commit --allow-empty -m "chore: release 2025.12.0"
   ```

2. Or manually edit the release PR created by Release Please

### Skipping Release

To merge commits without triggering a release, add `chore(main): release` to your PR title or commit message:

```bash
git commit -m "chore(main): release skip - documentation updates"
```

## Version Scheme

This project uses **CalVer** (Calendar Versioning):
- Format: `YYYY.MM.PATCH`
- Example: `2025.11.0`
- Year: 4-digit year (2025)
- Month: Month without leading zero (11 = November)
- Patch: Incremental counter reset monthly (0, 1, 2...)

Release Please automatically calculates the patch number based on the current month.

## Troubleshooting

### Release PR Not Created

**Cause**: No conventional commits since last release, or all commits are non-release types (docs, test, style).

**Solution**: Ensure at least one `feat:` or `fix:` commit exists.

### PR Title Validation Fails

**Cause**: PR title doesn't follow conventional format.

**Solution**: Update PR title to match the format: `type: description`

### Merge Conflict in Release PR

**Cause**: Files were updated on main while release PR was open.

**Solution**:
```bash
# Locally update release-please branch
git checkout release-please--branches--main
git rebase main
git push --force-with-lease
```

Or close the PR and let Release Please create a new one.

### Version Not Bumped Correctly

**Cause**: `.release-please-manifest.json` is out of sync.

**Solution**: Manually update the manifest to the last released version, commit, and push.

## Configuration Files

- **`.github/workflows/release-please.yml`**: Main Release Please workflow
- **`release-please-config.json`**: Release Please configuration
- **`.release-please-manifest.json`**: Current version tracking
- **`.github/workflows/conventional-commits.yml`**: PR title validation
- **`.github/workflows/release.yml`**: Release artifact generation

## Additional Resources

- [Release Please Documentation](https://github.com/googleapis/release-please)
- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [CalVer Specification](https://calver.org/)
- [Sigstore Documentation](https://www.sigstore.dev/)
- [SLSA Framework](https://slsa.dev/)
