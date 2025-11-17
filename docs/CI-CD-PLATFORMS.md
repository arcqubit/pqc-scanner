# CI/CD Platform Support

The pqc-scanner project supports multiple CI/CD platforms to provide flexibility for different organizational requirements and deployment scenarios.

## Supported Platforms

### 1. GitHub Actions (Primary Platform)

**Status**: ✅ Production Ready

**Location**: `.github/workflows/`

**Features**:
- Comprehensive CI pipeline (testing, building, security scanning)
- Multi-OS support (Ubuntu, Windows)
- WASM builds for all targets
- Automated releases with CalVer versioning
- NPM publishing with provenance
- Docker container builds (multi-arch)
- SBOM generation
- Security scanning (CodeQL, dependency audit, Scorecard)

**Documentation**: Existing GitHub Actions workflows

**Best For**: GitHub-hosted projects, teams using GitHub ecosystem

---

### 2. GitLab CI/CD

**Status**: ✅ Ready for Use

**Location**: `.gitlab/.gitlab-ci.yml`

**Features**:
- Multi-stage pipeline (validate, test, build, security, package, publish)
- Docker-in-Docker support
- GitLab Container Registry integration
- Artifact caching for faster builds
- Security scanning with reports
- Scheduled security scans
- Manual deployment approval for production

**Documentation**: [.gitlab/README.md](.gitlab/README.md)

**Best For**: GitLab users, self-hosted requirements, enterprise GitLab installations

**Setup Time**: ~30 minutes

---

### 3. Jenkins

**Status**: ✅ Ready for Use

**Location**: `jenkins/Jenkinsfile`

**Features**:
- Declarative pipeline with parallel execution
- Docker agent support
- Multi-branch support
- Artifact archival and fingerprinting
- Slack/email notifications
- Security scanning integration
- Manual approval stages

**Documentation**: [jenkins/README.md](jenkins/README.md)

**Best For**: Enterprise environments, existing Jenkins infrastructure, self-hosted requirements

**Setup Time**: ~1 hour (including Jenkins setup)

---

### 4. Travis CI

**Status**: ✅ Ready for Use

**Location**: `travis/.travis.yml`

**Features**:
- Build matrix (multi-OS support)
- GitHub Releases integration
- NPM publishing
- Automated tag-based releases
- Simple configuration
- Good for open-source projects

**Documentation**: [travis/README.md](travis/README.md)

**Best For**: Open-source projects, simple CI/CD needs, Travis CI users

**Setup Time**: ~20 minutes

---

### 5. ArgoCD (GitOps Deployment)

**Status**: ✅ Ready for Use

**Location**: `argocd/`

**Features**:
- GitOps continuous deployment
- Multi-environment support (dev, staging, prod)
- Automated image updates
- Self-healing applications
- Rollback capability
- Health monitoring
- Kubernetes-native

**Documentation**: [argocd/README.md](argocd/README.md)

**Best For**: Kubernetes deployments, GitOps workflows, cloud-native applications

**Setup Time**: ~45 minutes (including ArgoCD installation)

---

## Quick Start Guide

### For GitHub Users

No setup needed - workflows run automatically on push/PR/tag.

### For GitLab Users

1. Copy `.gitlab/.gitlab-ci.yml` to project root or import repository to GitLab
2. Set required CI/CD variables (NPM_TOKEN)
3. Push to trigger pipeline
4. See [.gitlab/README.md](.gitlab/README.md) for details

### For Jenkins Users

1. Install required Jenkins plugins
2. Create credentials (NPM_TOKEN, GITHUB_TOKEN)
3. Create multibranch or regular pipeline job
4. Point to `jenkins/Jenkinsfile`
5. See [jenkins/README.md](jenkins/README.md) for details

### For Travis CI Users

1. Enable repository on Travis CI
2. Set environment variables (GITHUB_TOKEN, NPM_TOKEN)
3. Copy `travis/.travis.yml` to project root
4. Push to trigger build
5. See [travis/README.md](travis/README.md) for details

### For ArgoCD Users

1. Install ArgoCD in Kubernetes cluster
2. Apply `argocd/application.yaml`
3. ArgoCD will sync from Git automatically
4. See [argocd/README.md](argocd/README.md) for details

---

## Feature Comparison

| Feature | GitHub Actions | GitLab CI | Jenkins | Travis CI | ArgoCD |
|---------|---------------|-----------|---------|-----------|--------|
| **Testing** | ✅ | ✅ | ✅ | ✅ | N/A |
| **Multi-OS** | ✅ | ✅ | ✅ | ✅ | N/A |
| **WASM Build** | ✅ | ✅ | ✅ | ✅ | N/A |
| **Security Scan** | ✅ | ✅ | ✅ | ✅ | N/A |
| **Container Build** | ✅ | ✅ | ✅ | ❌ | N/A |
| **NPM Publish** | ✅ | ✅ | ✅ | ✅ | N/A |
| **Release Creation** | ✅ | ✅ | ✅ | ✅ | N/A |
| **SBOM Generation** | ✅ | ✅ | ✅ | ✅ | N/A |
| **Self-Hosted** | ❌ | ✅ | ✅ | ❌ | ✅ |
| **GitOps** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Kubernetes Deploy** | ⚠️ | ⚠️ | ⚠️ | ❌ | ✅ |
| **Auto Rollback** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Health Monitoring** | ❌ | ❌ | ❌ | ❌ | ✅ |

Legend:
- ✅ Fully supported
- ⚠️ Possible but not configured
- ❌ Not supported/applicable
- N/A Not applicable for this platform

## Platform Capabilities Summary

| Capability | GitHub Actions | GitLab CI | Jenkins | Travis CI | ArgoCD |
|------------|----------------|-----------|---------|-----------|--------|
| **Multi-OS** | ✅ (Ubuntu, Windows, macOS) | ✅ (via Docker) | ✅ (via agents) | ✅ (Linux, macOS) | N/A |
| **Container Build** | ✅ (multi-arch) | ✅ (DinD) | ✅ (Docker) | ❌ | N/A |
| **Self-Hosted** | ❌ (runners only) | ✅ | ✅ | ❌ | ✅ |
| **GitOps** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Auto-Deploy** | ⚠️ (manual) | ⚠️ (manual) | ⚠️ (manual) | ⚠️ (manual) | ✅ |
| **Managed Service** | ✅ | ✅ (cloud) | ❌ | ✅ | ❌ |
| **Free Tier** | ✅ (generous) | ✅ (limited) | N/A (self-hosted) | ✅ (OSS) | N/A (self-hosted) |

Legend:
- ✅ Fully supported
- ⚠️ Possible but requires configuration
- ❌ Not supported
- N/A Not applicable

---

## Platform Selection Guide

### Choose GitHub Actions if:
- ✅ Project hosted on GitHub
- ✅ Want minimal setup
- ✅ Need excellent ecosystem integration
- ✅ Prefer managed solution

### Choose GitLab CI/CD if:
- ✅ Using GitLab for source control
- ✅ Need self-hosted option
- ✅ Want integrated registry
- ✅ Require comprehensive security scanning

### Choose Jenkins if:
- ✅ Have existing Jenkins infrastructure
- ✅ Need maximum customization
- ✅ Require complex workflows
- ✅ Self-hosted requirement

### Choose Travis CI if:
- ✅ Open-source project
- ✅ Want simple configuration
- ✅ Need fast setup
- ✅ Already using Travis CI

### Choose ArgoCD if:
- ✅ Deploying to Kubernetes
- ✅ Want GitOps workflow
- ✅ Need declarative deployment
- ✅ Require rollback capability

---

## Multi-Platform Strategy

You can use multiple platforms simultaneously:

### Recommended Combinations

1. **GitHub Actions + ArgoCD**
   - GitHub Actions for CI (test, build, publish)
   - ArgoCD for CD (Kubernetes deployment)

2. **GitLab CI + ArgoCD**
   - GitLab CI for complete CI/CD
   - ArgoCD for Kubernetes GitOps

3. **Jenkins + ArgoCD**
   - Jenkins for CI pipeline
   - ArgoCD for deployment automation

### Not Recommended

- Running multiple CI platforms for the same branch (wasteful)
- Using different platforms for dev vs prod (inconsistent)

---

## Migration Path

### From GitHub Actions to GitLab CI

1. Review `.gitlab/.gitlab-ci.yml`
2. Map GitHub secrets to GitLab variables
3. Test on branch before main
4. Update badges and documentation

### From Travis CI to GitHub Actions

1. Already have GitHub Actions configured
2. Disable Travis CI repository
3. Update status badges

### Adding ArgoCD to Existing CI

1. Keep existing CI platform
2. Add ArgoCD for deployment
3. Update CI to build and push images
4. Let ArgoCD handle deployment

---

## Maintenance

### Regular Updates

- **Monthly**: Update action/tool versions
- **Quarterly**: Review and optimize build times
- **Annually**: Security audit of all pipelines

### Monitoring

Each platform provides build status:

- **GitHub**: Actions tab, status badges
- **GitLab**: CI/CD → Pipelines
- **Jenkins**: Build history, Blue Ocean
- **Travis**: Build dashboard
- **ArgoCD**: Application health dashboard

---

## Troubleshooting

For platform-specific issues, see:

- [GitLab Troubleshooting](.gitlab/README.md#troubleshooting)
- [Jenkins Troubleshooting](jenkins/README.md#troubleshooting)
- [Travis Troubleshooting](travis/README.md#troubleshooting)
- [ArgoCD Troubleshooting](argocd/README.md#troubleshooting)

---

## Support and Contributions

- Issues: [GitHub Issues](https://github.com/arcqubit/pqc-scanner/issues)
- Discussions: [GitHub Discussions](https://github.com/arcqubit/pqc-scanner/discussions)
- Documentation: This file and platform-specific READMEs

---

## References

- [Multi-Platform CI/CD Plan](plans/multi-platform-cicd.md)
- [CalVer Versioning](CALVER.md)
- [Project README](../README.md)
- [GitHub Actions Documentation](https://docs.github.com/actions)
- [GitLab CI Documentation](https://docs.gitlab.com/ee/ci/)
- [Jenkins Documentation](https://www.jenkins.io/doc/)
- [Travis CI Documentation](https://docs.travis-ci.com/)
- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)
