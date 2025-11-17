# Multi-Platform CI/CD Automation Plan

**Project**: pqc-scanner
**Version**: 2025.11.0
**Date**: 2025-11-17
**Branch**: feature/multi-platform-cicd

## Executive Summary

This plan outlines the implementation of comprehensive CI/CD automation across multiple platforms (GitHub Actions, GitLab CI/CD, Jenkins, Travis CI, and ArgoCD) for the pqc-scanner project. The goal is to provide flexible deployment options for organizations using different CI/CD platforms while maintaining consistency in build quality, security, and release processes.

## Current State Assessment

### Existing GitHub Actions Workflows

The project currently has robust GitHub Actions automation:

1. **ci.yml** - Comprehensive CI pipeline with:
   - Multi-OS testing (Ubuntu, Windows)
   - WASM builds (bundler, nodejs, web targets)
   - Performance benchmarks
   - Code coverage (Codecov integration)
   - Static analysis (cargo-geiger for unsafe code)
   - Dependency analysis (cargo-udeps)

2. **release.yml** - Release automation with:
   - CalVer versioning support (YYYY.MM.MICRO)
   - Multi-target WASM builds
   - Binary and WASM artifact creation
   - Docker container builds (multi-arch: amd64, arm64)
   - SBOM generation
   - GitHub Container Registry publishing

3. **npm-publish.yml** - NPM publishing with:
   - WASM optimization (wasm-opt)
   - Multi-target WASM packaging
   - NPM provenance support
   - Release asset management

4. **Security Workflows**:
   - cargo-audit.yml - Dependency vulnerability scanning
   - codeql.yml - CodeQL security analysis
   - scorecard.yml - OpenSSF Scorecard
   - dependency-review.yml - PR dependency scanning
   - sbom.yml - Software Bill of Materials generation

### Project Characteristics

- **Language**: Rust with WASM compilation
- **Versioning**: CalVer (YYYY.MM.MICRO)
- **Build Artifacts**:
  - Native binary (Linux x86_64)
  - WASM packages (bundler, nodejs, web)
  - Docker containers (multi-arch)
  - NPM packages
- **Package Registries**:
  - NPM (@arcqubit/pdq-scanner)
  - GitHub Container Registry (ghcr.io)
  - GitHub Releases
- **Testing**: Cargo tests, benchmarks, clippy, rustfmt
- **Security**: OpenSSF best practices, SBOM, provenance

## Multi-Platform CI/CD Strategy

### Platform Selection Rationale

1. **GitHub Actions** (Primary)
   - Already implemented and mature
   - Native GitHub integration
   - Excellent ecosystem support
   - OIDC for NPM provenance

2. **GitLab CI/CD**
   - Second most popular platform
   - Self-hosted capability
   - Enterprise adoption
   - Docker-native workflows

3. **Jenkins**
   - Enterprise legacy systems
   - Self-hosted requirement
   - High customization needs
   - Pipeline-as-code support

4. **Travis CI**
   - Open-source project support
   - Simple configuration
   - Historical usage
   - Educational value

5. **ArgoCD**
   - GitOps deployment model
   - Kubernetes-native
   - Continuous deployment
   - Declarative configuration

### Design Principles

1. **Consistency**: All platforms should produce identical artifacts
2. **Security**: Maintain security scanning and provenance across platforms
3. **Modularity**: Reusable scripts and configurations
4. **Documentation**: Clear setup instructions per platform
5. **Maintenance**: Minimize duplication, use shared scripts where possible

## Implementation Plan

### Phase 1: Shared Infrastructure

Create reusable build scripts that all platforms can use:

**Directory Structure**:
```
pqc-scanner/
├── .gitlab/
│   └── .gitlab-ci.yml
├── jenkins/
│   ├── Jenkinsfile
│   └── README.md
├── travis/
│   ├── .travis.yml
│   └── README.md
├── argocd/
│   ├── application.yaml
│   ├── kustomization.yaml
│   └── README.md
├── scripts/
│   ├── ci/
│   │   ├── setup-rust.sh
│   │   ├── setup-wasm.sh
│   │   ├── run-tests.sh
│   │   ├── build-wasm.sh
│   │   ├── build-binary.sh
│   │   ├── security-scan.sh
│   │   └── publish-artifacts.sh
│   └── (existing scripts)
└── docs/
    └── plans/
        └── multi-platform-cicd.md (this file)
```

### Phase 2: GitLab CI/CD Implementation

**Features**:
- Multi-stage pipeline (test, build, security, deploy)
- Docker-in-Docker for container builds
- GitLab Container Registry integration
- GitLab Package Registry for NPM
- Artifact caching and storage
- Security scanning (SAST, dependency scanning)
- Performance testing

**Stages**:
1. **Validate**: Format, lint, dependency check
2. **Test**: Unit tests, integration tests, coverage
3. **Build**: Binary, WASM (all targets), containers
4. **Security**: Audit, SBOM, vulnerability scan
5. **Publish**: NPM, container registry, releases

### Phase 3: Jenkins Implementation

**Features**:
- Declarative pipeline
- Multi-branch support
- Docker agent support
- Artifact archival
- Security scanning integration
- Parallel execution
- Email notifications

**Pipeline Structure**:
1. **Checkout**: SCM checkout with submodules
2. **Environment**: Setup Rust, WASM toolchain
3. **Quality**: Tests, linting, formatting
4. **Build**: Parallel builds (binary, WASM targets)
5. **Security**: Cargo audit, geiger scan
6. **Package**: Create release artifacts
7. **Publish**: Conditional deployment based on branch/tag

### Phase 4: Travis CI Implementation

**Features**:
- Matrix builds (OS: Linux, macOS)
- Rust version matrix (stable, beta)
- Build stages
- Conditional deployment
- GitHub Releases integration
- NPM publishing

**Build Matrix**:
```yaml
os: [linux, osx]
rust: [stable]
env:
  - TARGET=native
  - TARGET=wasm
```

### Phase 5: ArgoCD Implementation

**Features**:
- GitOps workflow for Kubernetes deployments
- Automated sync from Git repository
- Health status monitoring
- Rollback capability
- Multi-environment support (dev, staging, prod)

**Components**:
1. **Application**: ArgoCD Application definition
2. **Kustomization**: Base and overlays for environments
3. **Deployment**: Kubernetes Deployment manifests
4. **Service**: Service and Ingress definitions
5. **ConfigMap**: Configuration management

**Deployment Strategy**:
- Container-based deployment
- Rolling updates
- Health checks
- Resource limits
- Auto-scaling support

## Security Considerations

### Secrets Management

Each platform requires different secrets:

1. **GitHub Actions**: GitHub secrets
2. **GitLab CI**: GitLab CI/CD variables (masked)
3. **Jenkins**: Credentials plugin
4. **Travis CI**: Encrypted environment variables
5. **ArgoCD**: Kubernetes secrets

**Required Secrets**:
- NPM_TOKEN (NPM publishing)
- DOCKER_USERNAME, DOCKER_PASSWORD (container registries)
- GPG signing keys (optional)
- GitHub token (for releases)

### Security Scanning

All platforms should include:
- Dependency vulnerability scanning (cargo-audit)
- Unsafe code detection (cargo-geiger)
- SBOM generation (cargo-sbom or syft)
- Container scanning (trivy, grype)
- License compliance

### Supply Chain Security

- Signed commits (GPG)
- Artifact signing (cosign for containers)
- Provenance attestation (SLSA)
- Reproducible builds
- Dependency pinning

## Performance Optimization

### Caching Strategy

Each platform supports caching differently:

1. **GitHub Actions**: actions/cache
2. **GitLab CI**: cache directive with keys
3. **Jenkins**: Workspace caching, Docker layer caching
4. **Travis CI**: cache directive
5. **ArgoCD**: N/A (deployment only)

**Cache Targets**:
- Cargo registry (~/.cargo/registry)
- Cargo git dependencies (~/.cargo/git)
- Build artifacts (target/)
- WASM build cache
- Docker layers

### Build Parallelization

- Parallel test execution
- Parallel WASM target builds
- Matrix builds for multiple platforms
- Concurrent security scans

## Testing Strategy

### Test Stages

All platforms should execute:

1. **Unit Tests**: `cargo test`
2. **Integration Tests**: Full test suite
3. **Format Check**: `cargo fmt --check`
4. **Lint**: `cargo clippy -- -D warnings`
5. **Benchmarks**: `cargo bench --no-run` (validation)
6. **WASM Tests**: wasm-pack test for each target

### Platform-Specific Testing

- **GitHub Actions**: Multi-OS (Ubuntu, Windows)
- **GitLab CI**: Docker-based (consistent environment)
- **Jenkins**: Agent-specific (configured environments)
- **Travis CI**: Multi-OS matrix
- **ArgoCD**: Deployment verification

## Release Process

### Versioning

- **Format**: CalVer (YYYY.MM.MICRO)
- **Tagging**: `v2025.11.0` format
- **Triggers**: Tag push or manual workflow dispatch

### Artifact Creation

1. **Binary Artifacts**:
   - Linux x86_64 (static if possible)
   - macOS (if supported)
   - Windows (if supported)

2. **WASM Packages**:
   - Bundler target
   - Node.js target
   - Web target
   - Combined archive

3. **Container Images**:
   - Multi-arch (amd64, arm64)
   - Tagged with version and latest
   - Pushed to GHCR and Docker Hub

4. **NPM Packages**:
   - Published with provenance
   - Each WASM target as separate package option

### Release Checklist

- [ ] Update version in Cargo.toml
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Build all artifacts
- [ ] Generate SBOM
- [ ] Create checksums
- [ ] Sign artifacts (if configured)
- [ ] Create Git tag
- [ ] Push to registry/registries
- [ ] Create GitHub/GitLab release
- [ ] Update documentation

## Monitoring and Observability

### Build Monitoring

- Build status badges for each platform
- Notification on failures (email, Slack, etc.)
- Build duration tracking
- Artifact size monitoring

### Deployment Monitoring (ArgoCD)

- Application health status
- Sync status
- Resource utilization
- Error tracking

### Metrics to Track

- Build success rate
- Average build duration
- Test pass rate
- Code coverage percentage
- Security vulnerabilities found
- Deployment frequency
- Time to recovery

## Documentation Requirements

### Platform-Specific Documentation

Each platform directory should include:

1. **README.md**:
   - Setup instructions
   - Prerequisites
   - Configuration steps
   - Secrets management
   - Troubleshooting

2. **Configuration Files**:
   - Well-commented
   - Reference to shared scripts
   - Platform-specific optimizations

### User Documentation

Update main README.md to include:
- Supported CI/CD platforms
- Setup instructions for each
- Badge links for all platforms
- Migration guide from GitHub Actions

## Migration Path

### For Existing Users

1. **GitHub Actions** (no change)
   - Continue using existing workflows
   - Benefit from improvements

2. **New Platform Adoption**:
   - Choose target platform
   - Follow setup guide
   - Configure secrets
   - Test in branch before main
   - Monitor initial runs

### For New Projects

- Start with GitHub Actions (most mature)
- Add other platforms as needed
- Use Docker-based platforms for consistency

## Maintenance Plan

### Regular Updates

- **Monthly**: Update action versions, Docker images
- **Quarterly**: Review and optimize build times
- **Annually**: Security audit of all pipelines

### Deprecation Strategy

- Monitor platform popularity
- Communicate deprecation 6 months in advance
- Provide migration guide
- Archive deprecated configurations

## Success Criteria

### Technical Metrics

- [ ] All platforms produce identical artifacts
- [ ] Build time within 20% of GitHub Actions baseline
- [ ] 100% test pass rate on all platforms
- [ ] Zero high-severity security vulnerabilities
- [ ] SBOM generated for all releases

### Adoption Metrics

- [ ] Documentation complete for all platforms
- [ ] At least 3 successful releases via each platform
- [ ] Community feedback positive
- [ ] Migration path validated

## Risks and Mitigation

### Risk 1: Platform Divergence

**Risk**: Different platforms produce different artifacts
**Mitigation**: Shared scripts, checksums, automated comparison

### Risk 2: Maintenance Burden

**Risk**: Too many platforms to maintain
**Mitigation**: Shared scripts, automated testing, deprecation strategy

### Risk 3: Security Vulnerabilities

**Risk**: Secrets exposure, supply chain attacks
**Mitigation**: Secret scanning, least privilege, security reviews

### Risk 4: Build Complexity

**Risk**: Configurations become too complex
**Mitigation**: Documentation, examples, simplification passes

## Timeline

### Week 1: Foundation
- [x] Create plan document
- [ ] Review GitHub Actions for publish-readiness
- [ ] Create shared script infrastructure

### Week 2: GitLab
- [ ] Implement GitLab CI/CD
- [ ] Test on branch
- [ ] Documentation

### Week 3: Jenkins & Travis
- [ ] Implement Jenkins pipeline
- [ ] Implement Travis CI
- [ ] Testing and validation

### Week 4: ArgoCD & Polish
- [ ] Implement ArgoCD deployment
- [ ] Integration testing across all platforms
- [ ] Final documentation updates
- [ ] Release announcement

## Conclusion

This multi-platform CI/CD strategy provides flexibility for users while maintaining consistent build quality and security. By using shared scripts and following GitOps principles, we minimize maintenance burden while maximizing platform support.

The phased implementation allows for iterative improvement and validation at each stage. Success will be measured by artifact consistency, build reliability, and community adoption.

## References

- [CalVer Specification](../CALVER.md)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [GitLab CI/CD Documentation](https://docs.gitlab.com/ee/ci/)
- [Jenkins Pipeline Documentation](https://www.jenkins.io/doc/book/pipeline/)
- [Travis CI Documentation](https://docs.travis-ci.com/)
- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)
- [OpenSSF Best Practices](https://www.bestpractices.dev/)
- [SLSA Framework](https://slsa.dev/)
