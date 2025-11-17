# GitLab CI/CD for pqc-scanner

This directory contains GitLab CI/CD configuration for automated testing, building, and deployment of the pqc-scanner project.

## Overview

The GitLab CI/CD pipeline provides comprehensive automation including:

- Code validation (formatting, linting)
- Multi-stage testing (unit, integration, WASM)
- Multi-target builds (binary, WASM bundler/nodejs/web)
- Security scanning (audit, unsafe code detection, SBOM)
- Container builds (multi-arch)
- Artifact packaging and publishing
- NPM package publishing
- GitLab release creation

## Pipeline Stages

1. **Validate**: Code formatting, linting, dependency checks
2. **Test**: Unit tests, integration tests, WASM tests, code coverage
3. **Build**: Binary compilation, WASM builds for all targets, benchmarks
4. **Security**: Dependency audit, unsafe code detection, SBOM generation, container scanning
5. **Package**: Artifact creation, container builds
6. **Publish**: GitLab releases, NPM publishing, container registry

## Prerequisites

### For Self-Hosted GitLab

1. **GitLab Runner** with Docker executor
2. **Rust toolchain** (handled by Docker image)
3. **Docker-in-Docker** support for container builds

### Required CI/CD Variables

Configure these in GitLab Settings → CI/CD → Variables:

| Variable | Description | Protected | Masked |
|----------|-------------|-----------|--------|
| `NPM_TOKEN` | NPM authentication token | ✅ Yes | ✅ Yes |
| `CI_REGISTRY_USER` | GitLab container registry user | Auto | Auto |
| `CI_REGISTRY_PASSWORD` | GitLab container registry password | Auto | Auto |

#### Getting an NPM Token

1. Login to npmjs.com
2. Go to Access Tokens in account settings
3. Generate new token with "Automation" type
4. Copy token and add to GitLab CI/CD variables

## Setup Instructions

### 1. Initial Setup

```bash
# Clone repository
git clone https://gitlab.com/your-org/pqc-scanner.git
cd pqc-scanner

# Verify CI/CD configuration
cat .gitlab/.gitlab-ci.yml
```

### 2. Configure GitLab Project

1. Go to Settings → CI/CD
2. Expand "Variables" section
3. Add required variables (see table above)
4. Enable "Auto DevOps" (optional)

### 3. Configure GitLab Runner

For self-hosted runners:

```bash
# Install GitLab Runner
curl -L "https://packages.gitlab.com/install/repositories/runner/gitlab-runner/script.deb.sh" | sudo bash
sudo apt-get install gitlab-runner

# Register runner
sudo gitlab-runner register \
  --url https://gitlab.com/ \
  --registration-token YOUR_TOKEN \
  --executor docker \
  --docker-image rust:1.83-bookworm \
  --docker-privileged
```

### 4. Enable Container Registry

1. Go to Settings → General → Visibility
2. Enable "Container Registry"
3. Save changes

### 5. First Pipeline Run

```bash
# Push to trigger pipeline
git add .
git commit -m "Setup GitLab CI/CD"
git push origin main
```

## Pipeline Triggers

The pipeline runs on:

- **Push to main/develop**: Full pipeline
- **Merge Requests**: Test and validate stages
- **Tags** (v*.*.*): Full pipeline + publish stages
- **Manual**: Via GitLab UI (Pipelines → Run Pipeline)
- **Schedule**: Nightly security scans (configure in CI/CD → Schedules)

## Creating a Release

### 1. Prepare Release

```bash
# Update version in Cargo.toml
sed -i 's/version = ".*"/version = "2025.11.1"/' Cargo.toml

# Update CHANGELOG.md
echo "## [2025.11.1] - $(date +%Y-%m-%d)" >> CHANGELOG.md
echo "### Added" >> CHANGELOG.md
echo "- Feature description" >> CHANGELOG.md

# Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 2025.11.1"
git push origin main
```

### 2. Create Tag

```bash
# Create annotated tag (CalVer format)
git tag -a v2025.11.1 -m "Release v2025.11.1"

# Push tag to trigger release pipeline
git push origin v2025.11.1
```

### 3. Monitor Pipeline

1. Go to CI/CD → Pipelines
2. Click on the pipeline for the tag
3. Monitor job progress
4. Review artifacts when complete

### 4. Publish to NPM (Manual)

The NPM publish job requires manual approval:

1. Go to pipeline for the tag
2. Find "publish-npm" job
3. Click "Play" button to start manual job
4. Monitor output for success

### 5. Verify Release

```bash
# Check GitLab release
# Go to Deployments → Releases

# Verify container image
docker pull registry.gitlab.com/your-org/pqc-scanner:2025.11.1

# Verify NPM package
npm view @arcqubit/pqc-scanner
```

## Caching Strategy

The pipeline uses GitLab CI cache to speed up builds:

- **Cache Key**: Based on `Cargo.lock` hash
- **Cached Paths**:
  - `.cargo/registry` - Cargo package registry
  - `.cargo/git` - Git dependencies
  - `target/` - Build artifacts

### Cache Troubleshooting

If builds are slow or using outdated dependencies:

```bash
# Clear cache via GitLab UI
# Go to CI/CD → Pipelines → Clear Runner Caches

# Or force cache refresh in .gitlab-ci.yml
# Change cache key temporarily
```

## Artifacts

Pipeline artifacts are retained for:

- **Build artifacts**: 1 week
- **Test reports**: 30 days
- **Security reports**: 30 days
- **SBOM files**: 90 days
- **Release packages**: 1 month

Download artifacts:

1. Go to CI/CD → Pipelines
2. Click on pipeline
3. Click job name
4. Click "Browse" or "Download" button on right sidebar

## Security Scanning

### Dependency Audit

Runs on every pipeline:

```bash
# View audit results
# Go to Security & Compliance → Vulnerability Report
```

### Container Scanning

Uses Trivy for container vulnerability scanning:

```bash
# View container scan results in job output
# Go to CI/CD → Pipelines → container-scan job
```

### SBOM Generation

Software Bill of Materials generated for releases:

- **SPDX Format**: `sbom-spdx.json`
- **CycloneDX Format**: `sbom-cyclonedx.json`
- **Dependency Tree**: `sbom-dependencies.txt`

Download from release artifacts or job artifacts.

## Scheduled Pipelines

Configure nightly security scans:

1. Go to CI/CD → Schedules
2. Click "New Schedule"
3. Configure:
   - **Description**: Nightly Security Scan
   - **Interval Pattern**: `0 2 * * *` (2 AM daily)
   - **Target Branch**: main
   - **Activated**: ✅
4. Save schedule

## Troubleshooting

### Pipeline Fails on Format Check

```bash
# Run locally to fix
cargo fmt
git add .
git commit -m "fix: format code"
git push
```

### Pipeline Fails on Clippy

```bash
# Run locally to fix
cargo clippy --all-targets --all-features -- -D warnings
# Fix reported issues
git add .
git commit -m "fix: clippy warnings"
git push
```

### WASM Build Fails

```bash
# Install wasm-pack locally
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Test build
wasm-pack build --target bundler --out-dir pkg

# Check for errors in output
```

### Container Build Fails

Check Docker-in-Docker configuration:

```bash
# Verify runner has --docker-privileged flag
sudo gitlab-runner verify

# Check runner configuration
sudo cat /etc/gitlab-runner/config.toml
```

### NPM Publish Fails

Verify NPM token:

```bash
# Test token locally
echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > ~/.npmrc
npm whoami

# Update token in GitLab if expired
```

## Performance Optimization

### Reduce Build Time

1. **Use cache effectively**: Ensure cache key is stable
2. **Parallel jobs**: Pipeline runs multiple jobs in parallel
3. **Shared runners**: Use GitLab.com runners for better performance
4. **Docker layer caching**: Enable in runner configuration

### Optimize Cache Size

```yaml
# In .gitlab-ci.yml, adjust cache paths
cache:
  paths:
    - .cargo/registry/index
    - .cargo/registry/cache
    - .cargo/git/db
    - target/release/deps
    - target/release/build
```

## Migration from GitHub Actions

Key differences:

| Feature | GitHub Actions | GitLab CI/CD |
|---------|---------------|--------------|
| Config file | `.github/workflows/*.yml` | `.gitlab/.gitlab-ci.yml` |
| Secrets | GitHub Secrets | CI/CD Variables |
| Container registry | ghcr.io | registry.gitlab.com |
| Releases | GitHub Releases | GitLab Releases |
| Artifacts | Upload/download actions | Built-in artifacts |

### Migration Steps

1. Review existing GitHub Actions workflows
2. Map jobs to GitLab stages
3. Convert secrets to CI/CD variables
4. Update container registry references
5. Test pipeline on branch before merging
6. Update documentation and README badges

## Advanced Configuration

### Custom Docker Image

Build custom image with pre-installed tools:

```dockerfile
# .gitlab/Dockerfile
FROM rust:1.83-bookworm

RUN cargo install wasm-pack cargo-audit cargo-geiger
RUN rustup target add wasm32-unknown-unknown
```

Update `.gitlab-ci.yml`:

```yaml
default:
  image: registry.gitlab.com/your-org/pqc-scanner/ci-image:latest
```

### Multi-Project Pipelines

Trigger downstream projects:

```yaml
trigger-downstream:
  stage: publish
  trigger:
    project: your-org/downstream-project
    branch: main
  only:
    - tags
```

### Environment-Specific Deployments

```yaml
deploy-staging:
  stage: publish
  script:
    - echo "Deploying to staging"
  environment:
    name: staging
    url: https://staging.example.com
  only:
    - main

deploy-production:
  stage: publish
  script:
    - echo "Deploying to production"
  environment:
    name: production
    url: https://example.com
  only:
    - tags
  when: manual
```

## Support

For issues with GitLab CI/CD:

1. Check [GitLab CI/CD Documentation](https://docs.gitlab.com/ee/ci/)
2. Review pipeline logs in GitLab UI
3. Open issue in project repository
4. Contact GitLab support (if using GitLab EE)

## References

- [GitLab CI/CD Documentation](https://docs.gitlab.com/ee/ci/)
- [GitLab CI/CD Variables](https://docs.gitlab.com/ee/ci/variables/)
- [GitLab Container Registry](https://docs.gitlab.com/ee/user/packages/container_registry/)
- [GitLab Releases](https://docs.gitlab.com/ee/user/project/releases/)
- [CalVer Specification](../docs/CALVER.md)
- [Project README](../README.md)
