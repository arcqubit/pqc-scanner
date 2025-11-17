# Travis CI for pqc-scanner

This directory contains Travis CI configuration for automated testing, building, and deployment of the pqc-scanner project.

## Overview

Travis CI provides continuous integration with:

- Multi-OS testing (Linux, macOS)
- Code validation (formatting, linting)
- Comprehensive testing (unit, integration, WASM)
- Multi-target WASM builds
- Security auditing
- Automated GitHub releases
- NPM publishing

## Prerequisites

### Travis CI Account

1. Sign up at [travis-ci.com](https://travis-ci.com/) (for private repos) or [travis-ci.org](https://travis-ci.org/) (for open source)
2. Connect your GitHub account
3. Enable Travis CI for your repository

### Required Environment Variables

Configure in Travis CI Settings (Repository → Settings → Environment Variables):

| Variable | Description | Display Value | Example |
|----------|-------------|---------------|---------|
| `GITHUB_TOKEN` | GitHub Personal Access Token | ❌ No | `ghp_...` |
| `NPM_TOKEN` | NPM authentication token | ❌ No | `npm_...` |
| `NPM_EMAIL` | NPM account email | ✅ Yes | `you@example.com` |

## Setup Instructions

### 1. Enable Repository on Travis CI

1. Go to [travis-ci.com](https://travis-ci.com/)
2. Sign in with GitHub
3. Click on your profile icon → Settings
4. Find `pqc-scanner` repository
5. Toggle the switch to enable Travis CI

### 2. Configure Environment Variables

#### GitHub Token

For GitHub Releases:

1. Go to GitHub Settings → Developer settings → Personal access tokens
2. Generate new token (classic)
3. Scopes needed:
   - ✅ `repo` (full control)
   - ✅ `write:packages`
4. Copy token
5. In Travis CI:
   - Repository Settings → Environment Variables
   - Name: `GITHUB_TOKEN`
   - Value: (paste token)
   - Display value in build log: ❌ OFF
   - Click **Add**

#### NPM Token

For NPM publishing:

1. Login to [npmjs.com](https://www.npmjs.com/)
2. Account → Access Tokens
3. Generate New Token → Automation
4. Copy token
5. In Travis CI:
   - Name: `NPM_TOKEN`
   - Value: (paste token)
   - Display value: ❌ OFF
   - Click **Add**
6. Add email:
   - Name: `NPM_EMAIL`
   - Value: your@email.com
   - Display value: ✅ ON
   - Click **Add**

### 3. Activate Builds

#### Option A: Push to GitHub

```bash
# Any push triggers Travis CI
git add .
git commit -m "Enable Travis CI"
git push origin main
```

#### Option B: Manual Trigger

1. Go to Travis CI dashboard
2. Click on `pqc-scanner` repository
3. Click **More options** → **Trigger build**
4. Select branch
5. Click **Trigger custom build**

### 4. Verify Build

1. Check Travis CI dashboard
2. Click on build to see progress
3. Review build logs
4. Check for any failures

## Build Matrix

Travis CI runs multiple jobs in parallel:

### Validation Jobs
- **Format Check**: Verify code formatting with `cargo fmt`
- **Clippy**: Lint code with `cargo clippy`

### Test Jobs
- **Unit Tests**: Run all unit tests
- **Integration Tests**: Run integration test suite
- **macOS Tests**: Run tests on macOS (optional)

### Build Jobs
- **WASM Bundler**: Build WASM for bundler target
- **WASM Node.js**: Build WASM for Node.js target
- **WASM Web**: Build WASM for web target
- **Binary Build**: Build native binary

### Security Jobs
- **Security Audit**: Scan for known vulnerabilities

### Deploy Jobs (Tags Only)
- **SBOM Generation**: Generate Software Bill of Materials
- **GitHub Release**: Create GitHub release with artifacts
- **NPM Publish**: Publish to NPM registry

## Creating a Release

### 1. Prepare Release

```bash
# Update version in Cargo.toml
sed -i 's/version = ".*"/version = "2025.11.1"/' Cargo.toml

# Update CHANGELOG.md
cat >> CHANGELOG.md << 'EOF'
## [2025.11.1] - 2025-11-17
### Added
- New feature description
### Fixed
- Bug fix description
EOF

# Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 2025.11.1"
git push origin main
```

### 2. Create and Push Tag

```bash
# Create annotated tag (CalVer format)
git tag -a v2025.11.1 -m "Release v2025.11.1"

# Push tag to trigger release build
git push origin v2025.11.1
```

### 3. Monitor Release Build

1. Go to Travis CI dashboard
2. Find build for tag `v2025.11.1`
3. Monitor progress through stages
4. Wait for deploy stage to complete

### 4. Verify Release

Check that release was created:

```bash
# GitHub release
gh release view v2025.11.1

# NPM package (if enabled)
npm view @arcqubit/pqc-scanner

# Download artifacts
curl -LO https://github.com/arcqubit/pqc-scanner/releases/download/v2025.11.1/pqc-scanner-2025.11.1-linux-x86_64.tar.gz
```

## Build Stages

Travis CI uses these stages:

1. **validate**: Code formatting and linting (always runs)
2. **test**: Unit and integration tests (always runs)
3. **build**: Binary and WASM builds (always runs)
4. **security**: Security scanning (always runs)
5. **deploy**: Release creation (tags only)

## Caching

Travis CI caches:
- Cargo registry (`~/.cargo/registry`)
- Cargo git (`~/.cargo/git`)
- Build target directory (`target/`)

This speeds up builds by reusing downloaded dependencies and compiled artifacts.

### Clear Cache

If you encounter cache-related issues:

1. Go to Repository Settings on Travis CI
2. Click **Caches** in left menu
3. Click **Delete all repository caches**
4. Trigger new build

## Artifacts

Release builds create these artifacts:

- `pqc-scanner-{version}-linux-x86_64.tar.gz` - Native binary
- `pqc-scanner-wasm-bundler-{version}.tar.gz` - WASM bundler
- `pqc-scanner-wasm-nodejs-{version}.tar.gz` - WASM Node.js
- `pqc-scanner-wasm-web-{version}.tar.gz` - WASM web
- `pqc-scanner-wasm-all-{version}.tar.gz` - All WASM targets
- `checksums.txt` - SHA256 checksums

Download from GitHub Releases page.

## Notifications

### Email Notifications

Default configuration:
- **On success**: Only when status changes (fail → pass)
- **On failure**: Every failure

### Customize Notifications

Add to `.travis.yml`:

```yaml
notifications:
  email:
    recipients:
      - team@example.com
    on_success: always
    on_failure: always

  slack:
    rooms:
      - secure: "encrypted-webhook-url"
    on_success: change
    on_failure: always
```

### Disable Notifications

```yaml
notifications:
  email: false
```

## Troubleshooting

### Build Timeout

**Problem**: Build exceeds 50-minute limit

**Solution**: Optimize build or split into multiple jobs

```yaml
# Add timeout extension
script:
  - travis_wait 60 cargo build --release
```

### Cache Issues

**Problem**: Builds using stale dependencies

**Solution**: Clear cache (see Caching section above)

### WASM Build Fails

**Problem**: wasm-pack installation fails

**Solution**: Use specific version or pre-install

```yaml
before_script:
  - curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- --version 0.12.1
```

### NPM Publish Fails

**Problem**: Authentication error

**Solution**:
1. Verify NPM_TOKEN is set correctly
2. Check token hasn't expired
3. Regenerate token if needed
4. Ensure token has "Automation" type

### GitHub Release Fails

**Problem**: Release creation fails with 403

**Solution**:
1. Verify GITHUB_TOKEN has correct scopes
2. Check token hasn't expired
3. Ensure repository allows release creation

### macOS Build Fails

**Problem**: macOS builds timeout or fail

**Solution**: Mark as optional in build matrix

```yaml
jobs:
  include:
    - name: "macOS - Tests"
      os: osx
      allow_failure: true  # Allow this job to fail
```

## Performance Optimization

### Reduce Build Time

1. **Cache aggressively**: Ensure cache is working
2. **Parallel jobs**: Build matrix runs jobs in parallel
3. **Skip unnecessary steps**: Use conditions

```yaml
script:
  - |
    if [ "$TRAVIS_EVENT_TYPE" = "pull_request" ]; then
      cargo test
    else
      cargo test --all-features
    fi
```

4. **Incremental compilation**: Enable for dev builds

```yaml
env:
  - CARGO_INCREMENTAL=1
```

### Optimize Matrix

Remove or conditionally run expensive jobs:

```yaml
jobs:
  include:
    - name: "Benchmarks"
      if: branch = main AND type = push
      script:
        - cargo bench --no-run
```

## Conditional Builds

### Skip CI

Add to commit message to skip CI:

```bash
git commit -m "docs: update README [skip ci]"
```

### Branch-Specific Builds

```yaml
branches:
  only:
    - main
    - develop
    - /^v\d+\.\d+\.\d+$/  # Tags only
```

### Pull Request Builds

```yaml
if: type = pull_request
script:
  - cargo test
  - cargo clippy
```

## Migration from GitHub Actions

### Key Differences

| Feature | GitHub Actions | Travis CI |
|---------|---------------|-----------|
| Config file | `.github/workflows/*.yml` | `.travis.yml` |
| Secrets | Repository secrets | Environment variables |
| Matrix | `strategy.matrix` | `jobs.include` |
| Artifacts | `upload-artifact` action | `before_deploy` + deploy |
| Caching | `actions/cache` | Built-in `cache` |

### Migration Steps

1. Review GitHub Actions workflows
2. Map jobs to Travis build matrix
3. Convert secrets to environment variables
4. Test on branch before enabling on main
5. Update CI badges in README

## Security Best Practices

1. **Never commit tokens**: Use environment variables
2. **Hide secrets**: Disable "Display value in build log"
3. **Minimal permissions**: Use least-privilege tokens
4. **Rotate tokens**: Regularly update tokens
5. **Review logs**: Check for leaked secrets

## Advanced Configuration

### Custom Docker Image

Use custom Docker image for builds:

```yaml
services:
  - docker

before_install:
  - docker pull your-registry/rust-wasm:latest

script:
  - docker run --rm -v $(pwd):/workspace your-registry/rust-wasm:latest cargo test
```

### Multi-Stage Deployment

Deploy to multiple targets:

```yaml
deploy:
  - provider: releases
    # GitHub releases config
    on:
      tags: true

  - provider: npm
    # NPM config
    on:
      tags: true

  - provider: script
    script: bash scripts/deploy.sh
    on:
      branch: main
```

### Build Status Badge

Add to README.md:

```markdown
[![Build Status](https://travis-ci.com/arcqubit/pqc-scanner.svg?branch=main)](https://travis-ci.com/arcqubit/pqc-scanner)
```

## Limitations

- **Build time**: 50 minutes per job (can request extension)
- **Build log**: 4 MB max (use `travis_fold` for large outputs)
- **Concurrent jobs**: Based on plan (open source: unlimited)
- **macOS builds**: Limited availability, slower

## Support

For Travis CI issues:

1. [Travis CI Documentation](https://docs.travis-ci.com/)
2. [Travis CI Community Forum](https://travis-ci.community/)
3. [Travis CI Status](https://www.traviscistatus.com/)
4. Email support (paid plans)

## References

- [Travis CI Documentation](https://docs.travis-ci.com/)
- [Building Rust Projects](https://docs.travis-ci.com/user/languages/rust/)
- [Deployment Documentation](https://docs.travis-ci.com/user/deployment/)
- [GitHub Releases Provider](https://docs.travis-ci.com/user/deployment/releases/)
- [NPM Provider](https://docs.travis-ci.com/user/deployment/npm/)
- [CalVer Specification](../docs/CALVER.md)
- [Project README](../README.md)
