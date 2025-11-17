# Jenkins CI/CD for pqc-scanner

This directory contains Jenkins pipeline configuration for automated testing, building, and deployment of the pqc-scanner project.

## Overview

The Jenkins pipeline provides comprehensive automation including:

- Code validation (formatting, linting, dependency checks)
- Parallel testing (unit, integration, WASM)
- Multi-target builds (binary, WASM bundler/nodejs/web)
- Security scanning (dependency audit, unsafe code detection)
- Artifact packaging with checksums
- SBOM generation
- NPM publishing (for tagged releases)
- GitHub release creation
- Slack notifications (optional)

## Prerequisites

### Jenkins Server Requirements

- **Jenkins Version**: 2.401+ (LTS recommended)
- **Required Plugins**:
  - Docker Pipeline Plugin
  - Pipeline Plugin
  - Git Plugin
  - Credentials Plugin
  - ANSI Color Plugin
  - Slack Notification Plugin (optional)
  - Blue Ocean (optional, for better UI)

### System Requirements

- Docker installed and running
- Sufficient disk space for Rust toolchain and build artifacts (10+ GB)
- Network access to pull Docker images and Rust crates

## Setup Instructions

### 1. Install Required Jenkins Plugins

Navigate to: **Manage Jenkins** → **Manage Plugins** → **Available**

Install these plugins:
- ✅ Docker Pipeline
- ✅ Pipeline
- ✅ Git
- ✅ Credentials Binding
- ✅ ANSI Color
- ✅ Slack Notification (optional)
- ✅ Blue Ocean (optional)

### 2. Configure Jenkins Credentials

#### NPM Token

1. Go to **Manage Jenkins** → **Manage Credentials**
2. Select domain (usually "Global")
3. Click **Add Credentials**
4. Configure:
   - **Kind**: Secret text
   - **Scope**: Global
   - **Secret**: Your NPM authentication token
   - **ID**: `npm-token`
   - **Description**: NPM Publishing Token
5. Click **OK**

#### GitHub Token (Optional)

For GitHub release creation:

1. Create GitHub Personal Access Token:
   - Go to GitHub Settings → Developer settings → Personal access tokens
   - Generate new token (classic)
   - Scopes: `repo`, `write:packages`
2. Add to Jenkins:
   - **Kind**: Secret text
   - **ID**: `github-token`
   - **Secret**: Your GitHub token

#### Slack Webhook (Optional)

For build notifications:

1. Create Slack incoming webhook
2. Add to Jenkins:
   - **Kind**: Secret text
   - **ID**: `slack-webhook`

### 3. Create Jenkins Pipeline Job

#### Option A: Multibranch Pipeline (Recommended)

1. Go to Jenkins Dashboard → **New Item**
2. Enter name: `pqc-scanner`
3. Select: **Multibranch Pipeline**
4. Click **OK**
5. Configure:

**Branch Sources:**
- Add source: Git
- Project Repository: `https://github.com/arcqubit/pqc-scanner.git`
- Credentials: Add GitHub credentials if private repo

**Build Configuration:**
- Mode: by Jenkinsfile
- Script Path: `jenkins/Jenkinsfile`

**Scan Multibranch Pipeline Triggers:**
- ✅ Periodically if not otherwise run
- Interval: 1 day

**Orphaned Item Strategy:**
- Days to keep old items: 30
- Max # of old items to keep: 20

6. Click **Save**

#### Option B: Pipeline Job (Single Branch)

1. Go to Jenkins Dashboard → **New Item**
2. Enter name: `pqc-scanner-main`
3. Select: **Pipeline**
4. Click **OK**
5. Configure:

**General:**
- ✅ GitHub project: `https://github.com/arcqubit/pqc-scanner/`

**Build Triggers:**
- ✅ GitHub hook trigger for GITScm polling
- OR: Poll SCM: `H/15 * * * *` (every 15 minutes)

**Pipeline:**
- Definition: Pipeline script from SCM
- SCM: Git
- Repository URL: `https://github.com/arcqubit/pqc-scanner.git`
- Branch: `*/main`
- Script Path: `jenkins/Jenkinsfile`

6. Click **Save**

### 4. Configure Docker Access

Jenkins must have Docker access to run the pipeline.

**Option A: Jenkins in Docker (Recommended)**

```bash
# Run Jenkins with Docker socket mounted
docker run -d \
  --name jenkins \
  -p 8080:8080 \
  -p 50000:50000 \
  -v jenkins_home:/var/jenkins_home \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v $(which docker):/usr/bin/docker \
  jenkins/jenkins:lts
```

**Option B: Add Jenkins User to Docker Group**

```bash
# Add jenkins user to docker group
sudo usermod -aG docker jenkins

# Restart Jenkins
sudo systemctl restart jenkins
```

### 5. First Build

Trigger the first build:

1. Go to Jenkins Dashboard
2. Click on `pqc-scanner` project
3. Click **Build Now** (or **Scan Multibranch Pipeline Now**)
4. Monitor console output

## Pipeline Configuration

### Environment Variables

Set these in Jenkins job configuration if needed:

| Variable | Description | Default |
|----------|-------------|---------|
| `PUBLISH_NPM` | Enable NPM publishing | `false` |
| `SLACK_WEBHOOK` | Slack webhook URL for notifications | (none) |
| `GITHUB_TOKEN` | GitHub token for releases | (none) |

Configure in: **Job** → **Configure** → **Pipeline** → **Environment**

### Build Parameters

The pipeline supports these parameters (if configured):

| Parameter | Type | Description |
|-----------|------|-------------|
| `RUST_VERSION` | String | Rust version (default: stable) |
| `SKIP_TESTS` | Boolean | Skip test stage |
| `SKIP_BENCHMARKS` | Boolean | Skip benchmark stage |

## Pipeline Stages

### 1. Environment Setup
- Display build information
- Verify Rust toolchain

### 2. Validate (Parallel)
- Format check (`cargo fmt`)
- Lint with Clippy
- Dependency analysis

### 3. Test (Parallel)
- Unit tests
- Integration tests
- WASM tests

### 4. Security Scan (Parallel)
- Dependency audit (cargo-audit)
- Unsafe code detection (cargo-geiger)

### 5. Build (Parallel)
- Native binary (release mode)
- WASM bundler target
- WASM Node.js target
- WASM web target

### 6. Benchmarks
- Performance benchmarks (main/develop only)

### 7. Package
- Create release archives
- Generate checksums
- (main branch and tags only)

### 8. SBOM Generation
- SPDX format
- CycloneDX format
- Dependency tree
- (main branch and tags only)

### 9. Publish (Tags Only)
- NPM publishing (manual approval)
- GitHub release creation

## Creating a Release

### 1. Prepare Release

```bash
# Update version in Cargo.toml
sed -i 's/version = ".*"/version = "2025.11.1"/' Cargo.toml

# Update CHANGELOG.md
cat >> CHANGELOG.md << EOF
## [2025.11.1] - $(date +%Y-%m-%d)
### Added
- New feature description
EOF

# Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 2025.11.1"
git push origin main
```

### 2. Create Tag

```bash
# Create annotated tag
git tag -a v2025.11.1 -m "Release v2025.11.1"

# Push tag
git push origin v2025.11.1
```

### 3. Publish Workflow

1. Jenkins detects new tag
2. Runs full pipeline
3. Creates release artifacts
4. **Manual Approval**: Go to build and approve "Publish NPM" stage
5. Publishes to NPM and creates GitHub release

### 4. Verify Release

Check Jenkins console output and verify:

```bash
# NPM package
npm view @arcqubit/pqc-scanner

# GitHub release
gh release view v2025.11.1
```

## Artifacts

Pipeline produces these artifacts:

### Build Artifacts
- `target/release/pqc-scanner` - Native binary
- `pkg/` - WASM bundler target
- `pkg-nodejs/` - WASM Node.js target
- `pkg-web/` - WASM web target

### Release Artifacts (Tags Only)
- `pqc-scanner-{version}-linux-x86_64.tar.gz`
- `pqc-scanner-wasm-bundler-{version}.tar.gz`
- `pqc-scanner-wasm-nodejs-{version}.tar.gz`
- `pqc-scanner-wasm-web-{version}.tar.gz`
- `pqc-scanner-wasm-all-{version}.tar.gz`
- `checksums.txt`

### Security Artifacts
- `audit-results.json`
- `sbom-spdx.json`
- `sbom-cyclonedx.json`
- `sbom-dependencies.txt`

Access artifacts:
1. Go to build page
2. Click **Build Artifacts** in left sidebar
3. Download files

## Notifications

### Slack Notifications

Configure Slack webhook for build notifications:

1. Create incoming webhook in Slack
2. Add webhook URL to Jenkins credentials (ID: `slack-webhook`)
3. Set `SLACK_WEBHOOK` environment variable in Jenkinsfile

Notifications sent for:
- ✅ Build success
- ❌ Build failure
- ⚠️ Build unstable

### Email Notifications

Add to Jenkinsfile post section:

```groovy
post {
    failure {
        emailext(
            subject: "Build Failed: ${env.JOB_NAME} - ${env.BUILD_NUMBER}",
            body: "Build failed. Check console output: ${env.BUILD_URL}",
            to: "team@example.com"
        )
    }
}
```

## Troubleshooting

### Pipeline Fails to Start

**Problem**: Docker image pull fails

```bash
# Pull image manually on Jenkins server
docker pull rust:1.83-bookworm

# Check Docker is running
sudo systemctl status docker
```

### Permission Denied Errors

**Problem**: Jenkins can't access Docker

```bash
# Add jenkins user to docker group
sudo usermod -aG docker jenkins

# Restart Jenkins
sudo systemctl restart jenkins
```

### Cargo Cache Issues

**Problem**: Slow builds, cache not working

**Solution**: Ensure Docker volume mounts are correct:

```groovy
agent {
    docker {
        image 'rust:1.83-bookworm'
        args '-v $HOME/.cargo:/root/.cargo'
    }
}
```

### WASM Build Fails

**Problem**: wasm-pack not found

```bash
# Pre-install in Docker image
# Create custom Dockerfile:
FROM rust:1.83-bookworm
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Use custom image in Jenkinsfile
agent {
    docker {
        image 'your-registry/rust-wasm:latest'
    }
}
```

### NPM Publish Fails

**Problem**: Authentication error

```bash
# Verify NPM token
# Test locally:
echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > ~/.npmrc
npm whoami

# Check token expiration
# Regenerate if needed
```

### Out of Disk Space

**Problem**: Jenkins runs out of space

```bash
# Clean old builds
# Go to: Manage Jenkins → System Information → Disk Usage

# Clean Docker
docker system prune -a
docker volume prune

# Configure build retention
# In job: Configure → Discard old builds
```

## Performance Optimization

### Build Time Optimization

1. **Cargo Cache**: Mount `.cargo` directory
2. **Docker Layer Caching**: Use BuildKit
3. **Parallel Execution**: Keep parallel stages
4. **Incremental Builds**: Enable for dev branches

### Resource Management

Configure in Jenkins:

**Manage Jenkins → Configure System → Executors**
- Number of executors: 2-4 (based on CPU cores)
- Labels: Add labels for specific agents

**Manage Jenkins → Configure System → Usage**
- Restrict project execution to agents with label

## Advanced Configuration

### Multi-Agent Setup

For parallel builds across multiple agents:

```groovy
pipeline {
    agent none
    stages {
        stage('Build') {
            parallel {
                stage('Linux Build') {
                    agent { label 'linux' }
                    steps { ... }
                }
                stage('Mac Build') {
                    agent { label 'macos' }
                    steps { ... }
                }
            }
        }
    }
}
```

### Custom Docker Image

Build custom image with pre-installed tools:

```dockerfile
FROM rust:1.83-bookworm

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install audit tools
RUN cargo install cargo-audit cargo-geiger cargo-sbom cargo-cyclonedx

# Install Node.js for NPM publishing
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

# Verify installations
RUN rustc --version && cargo --version && wasm-pack --version && node --version
```

Build and push:

```bash
docker build -t your-registry/pqc-scanner-ci:latest .
docker push your-registry/pqc-scanner-ci:latest
```

Update Jenkinsfile:

```groovy
agent {
    docker {
        image 'your-registry/pqc-scanner-ci:latest'
    }
}
```

### Parameterized Builds

Add parameters to Jenkinsfile:

```groovy
pipeline {
    parameters {
        string(name: 'RUST_VERSION', defaultValue: 'stable', description: 'Rust version')
        booleanParam(name: 'SKIP_TESTS', defaultValue: false, description: 'Skip tests')
        choice(name: 'BUILD_TYPE', choices: ['debug', 'release'], description: 'Build type')
    }
    // ...
}
```

### Blue Ocean UI

For better visualization:

1. Install Blue Ocean plugin
2. Access: `http://jenkins:8080/blue`
3. View pipeline visualization
4. Easier artifact access

## Migration from GitHub Actions

### Key Differences

| Feature | GitHub Actions | Jenkins |
|---------|---------------|---------|
| Config | YAML workflows | Groovy pipeline |
| Agents | GitHub runners | Docker agents |
| Secrets | GitHub Secrets | Jenkins Credentials |
| Artifacts | Upload/download | Archive/fingerprint |
| Stages | Jobs | Stages |

### Migration Checklist

- [ ] Install Jenkins plugins
- [ ] Configure credentials
- [ ] Create pipeline job
- [ ] Test on branch
- [ ] Configure webhooks
- [ ] Set up notifications
- [ ] Update documentation

## Security Best Practices

1. **Credentials**: Store all secrets in Jenkins Credentials
2. **Permissions**: Use role-based access control
3. **Audit**: Enable audit logs
4. **Updates**: Keep Jenkins and plugins updated
5. **Network**: Restrict Jenkins network access
6. **Scanning**: Enable security scanning in pipeline

## Support

For Jenkins-specific issues:

1. Check [Jenkins Documentation](https://www.jenkins.io/doc/)
2. Review console output
3. Check Jenkins logs: `$JENKINS_HOME/logs/`
4. Jenkins community forums
5. Project issue tracker

## References

- [Jenkins Pipeline Documentation](https://www.jenkins.io/doc/book/pipeline/)
- [Docker Pipeline Plugin](https://plugins.jenkins.io/docker-workflow/)
- [Jenkins Credentials](https://www.jenkins.io/doc/book/using/using-credentials/)
- [Blue Ocean](https://www.jenkins.io/doc/book/blueocean/)
- [CalVer Specification](../docs/CALVER.md)
- [Project README](../README.md)
