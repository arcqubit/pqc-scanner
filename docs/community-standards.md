# Community Standards

This document outlines the community standards, governance, and best practices for the PQC Scanner project.

## Table of Contents

- [Overview](#overview)
- [Community Health Files](#community-health-files)
- [Governance Model](#governance-model)
- [Contribution Workflow](#contribution-workflow)
- [Code Review Process](#code-review-process)
- [Release Process](#release-process)
- [Communication Channels](#communication-channels)
- [Recognition and Attribution](#recognition-and-attribution)
- [Metrics and Reporting](#metrics-and-reporting)

---

## Overview

PQC Scanner is committed to fostering an open, welcoming, and inclusive community. We follow industry best practices and OpenSSF guidelines to ensure a healthy, sustainable open-source project.

### Core Principles

1. **Transparency**: Open development, public discussions, documented decisions
2. **Inclusivity**: Welcome contributors of all backgrounds and skill levels
3. **Quality**: High standards for code, documentation, and security
4. **Respect**: Professional, courteous communication at all times
5. **Collaboration**: Community-driven decision making

### OpenSSF Best Practices

PQC Scanner follows [OpenSSF Best Practices](https://bestpractices.coreinfrastructure.org/):

- ✅ **Passing Badge**: Active achievement of OpenSSF criteria
- ✅ **Security**: Automated scanning, signed releases, vulnerability reporting
- ✅ **Quality**: CI/CD, code coverage, automated testing
- ✅ **Documentation**: Comprehensive docs, API documentation, examples
- ✅ **Community**: Code of Conduct, contributing guidelines, responsive maintainers

---

## Community Health Files

### Required Files (GitHub Standard)

All standard community health files are present in the repository root:

| File | Purpose | Location |
|------|---------|----------|
| **README.md** | Project overview, quick start | `/README.md` |
| **CODE_OF_CONDUCT.md** | Community behavior standards | `/CODE_OF_CONDUCT.md` |
| **CONTRIBUTING.md** | Contribution guidelines | `/CONTRIBUTING.md` |
| **SECURITY.md** | Security policy and reporting | `/SECURITY.md` |
| **SUPPORT.md** | Getting help and resources | `/SUPPORT.md` |
| **LICENSE** | Project license | `/LICENSE` |
| **CHANGELOG.md** | Version history and changes | `/CHANGELOG.md` |

### GitHub Templates

Issue and PR templates ensure consistent, high-quality submissions:

| Template | Type | Location |
|----------|------|----------|
| **Bug Report** | Issue | `.github/ISSUE_TEMPLATE/bug_report.yml` |
| **Feature Request** | Issue | `.github/ISSUE_TEMPLATE/feature_request.yml` |
| **Documentation** | Issue | `.github/ISSUE_TEMPLATE/documentation.yml` |
| **Performance** | Issue | `.github/ISSUE_TEMPLATE/performance.yml` |
| **Pull Request** | PR | `.github/PULL_REQUEST_TEMPLATE.md` |
| **Config** | Settings | `.github/ISSUE_TEMPLATE/config.yml` |

### Code Ownership

**CODEOWNERS** (`.github/CODEOWNERS`):
- Defines code ownership and review responsibilities
- Automatically requests reviews from appropriate maintainers
- Organized by subsystem for granular control
- Critical paths have explicit ownership

---

## Governance Model

### Project Leadership

**Maintainers**:
- @arcqubit (Lead Maintainer)
- Additional maintainers may be appointed based on contributions

**Responsibilities**:
- Code review and merge decisions
- Release management
- Security response
- Community moderation
- Roadmap planning

### Decision Making

1. **Consensus-Driven**: Most decisions made through community consensus
2. **Transparent**: Discussions in public (GitHub Discussions, issues)
3. **Documented**: Architectural decisions documented in ADRs (`docs/adr/`)
4. **Escalation**: Maintainers have final say on disputes

### Becoming a Maintainer

Contributors may be invited to become maintainers based on:
- Sustained high-quality contributions
- Deep understanding of the codebase
- Community engagement and support
- Alignment with project values
- Demonstrated commitment

**Process**:
1. Existing maintainer nominates contributor
2. Nomination announced in GitHub Discussions
3. 1-week community feedback period
4. Maintainer consensus (majority vote)
5. Formal invitation and onboarding

---

## Contribution Workflow

### Standard Workflow

```
1. Fork repository
2. Create feature branch
3. Make changes
4. Write tests
5. Update docs
6. Run local checks
7. Submit pull request
8. Address review feedback
9. Merge (by maintainer)
```

### Branch Naming Convention

- `feature/description` - New features
- `bugfix/issue-number-description` - Bug fixes
- `hotfix/critical-issue` - Production hotfixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring
- `perf/description` - Performance improvements

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`

**Examples**:
```
feat(scanner): add Kyber algorithm detection
fix(wasm): resolve memory leak in analysis loop
docs(readme): update installation instructions
```

### Pre-Commit Checklist

- [ ] Code formatted (`cargo fmt`)
- [ ] Lints pass (`cargo clippy -- -D warnings`)
- [ ] Tests pass (`cargo test`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for user-facing changes)
- [ ] Commit messages follow conventions

---

## Code Review Process

### Review Requirements

**Pull Requests Require**:
- ✅ 1+ approvals from maintainers
- ✅ All CI checks passing
- ✅ No unresolved conversations
- ✅ Up-to-date with base branch
- ✅ Code coverage maintained or improved

### Review Criteria

Reviewers evaluate:

1. **Correctness**: Does it work as intended?
2. **Quality**: Is the code well-written and maintainable?
3. **Testing**: Adequate test coverage?
4. **Documentation**: Are changes documented?
5. **Security**: Any security implications?
6. **Performance**: Performance impact acceptable?
7. **Breaking Changes**: Justified and documented?

### Review Etiquette

**For Authors**:
- Be responsive to feedback
- Be open to suggestions
- Ask questions if unclear
- Mark conversations resolved when addressed
- Keep PRs focused and reasonably sized

**For Reviewers**:
- Be constructive and specific
- Explain the "why" behind suggestions
- Distinguish between blocking and non-blocking feedback
- Acknowledge good work
- Be timely (respond within 3-5 days)

### Merge Strategy

- **Squash and Merge**: Default for most PRs (clean history)
- **Rebase and Merge**: For clean commits already following conventions
- **Regular Merge**: For merge commits (rare, specific cases)

---

## Release Process

### Versioning: CalVer

PQC Scanner uses **Calendar Versioning** (CalVer):

```
YYYY.MM.PATCH[-TAG]
```

**Examples**:
- `2025.11.0` - November 2025 release
- `2025.11.1` - November 2025, patch 1
- `2025.11.0-beta.1` - November 2025 beta 1
- `2025.12.0-rc.1` - December 2025 release candidate 1

See [docs/CALVER.md](CALVER.md) for details.

### Release Workflow

1. **Preparation**:
   - Update version in `Cargo.toml`, `package.json`
   - Update `CHANGELOG.md` with release notes
   - Run full test suite
   - Update documentation

2. **Tagging**:
   ```bash
   git tag -a v2025.11.0 -m "Release 2025.11.0"
   git push origin v2025.11.0
   ```

3. **Automation**:
   - GitHub Actions builds release artifacts
   - Sigstore signs all artifacts
   - SLSA provenance generated
   - GitHub Release created
   - SBOM generated and attached

4. **Verification**:
   - Download and verify signatures
   - Test release artifacts
   - Verify SLSA provenance
   - Check SBOM accuracy

5. **Announcement**:
   - GitHub Release notes
   - GitHub Discussions announcement
   - Social media (if applicable)

### Release Schedule

- **Major Releases**: Monthly (first Monday of the month)
- **Patch Releases**: As needed for bugs/security
- **Beta/RC**: 1-2 weeks before major releases
- **Hotfixes**: Immediate for critical security issues

---

## Communication Channels

### GitHub Discussions

**Primary community forum**: https://github.com/arcqubit/pqc-scanner/discussions

**Categories**:
- **Announcements**: Official project updates
- **Q&A**: Questions and answers
- **Ideas**: Feature discussions
- **Show and Tell**: Community projects
- **General**: Everything else

**Guidelines**:
- Search before posting
- Use appropriate categories
- Be specific and provide context
- Follow Code of Conduct

### GitHub Issues

**For**: Bug reports, feature requests, documentation issues

**NOT for**: General questions (use Discussions)

**Guidelines**:
- Use issue templates
- One issue per report
- Provide reproduction steps
- Include environment details

### Email

- **General**: support@arcqubit.io
- **Security**: security@arcqubit.io
- **Conduct**: conduct@arcqubit.io
- **Enterprise**: enterprise@arcqubit.io

### Social Media

- **Twitter/X**: @arcqubit (when available)
- **Blog**: https://arcqubit.io/blog (when available)

---

## Recognition and Attribution

### Contributor Recognition

Contributors are recognized in:

1. **CHANGELOG.md**: Listed in release notes
2. **GitHub**: Contribution graph, insights
3. **All Contributors**: Automated recognition bot (future)
4. **Release Notes**: Special mentions for significant contributions

### Types of Contributions

We recognize all types of contributions:
- Code contributions
- Documentation improvements
- Bug reports with reproduction
- Feature suggestions
- Community support
- Testing and QA
- Design and UX
- Translations
- Event organization

### Hall of Fame

Exceptional contributors may be featured in:
- Project README
- Security Hall of Fame (for security researchers)
- Special acknowledgment in release notes

---

## Metrics and Reporting

### Community Health Metrics

We track:
- Issue response time
- PR review time
- Contributor diversity
- Code review quality
- Release cadence
- Security response time

**Target SLAs**:
| Metric | Target |
|--------|--------|
| Issue First Response | < 3 days |
| PR First Review | < 5 days |
| Security Response | < 24-48 hours |
| Release Schedule | Monthly |

### Transparency Reports

Published quarterly:
- Contributor statistics
- Issue/PR velocity
- Security vulnerabilities addressed
- Community growth
- Project health

**Location**: `docs/reports/YYYY-QQ.md` (future)

### OpenSSF Scorecard

Monitored continuously:
- Current score target: ~10.0/10
- Automated daily scans
- Public dashboard: https://securityscorecards.dev/viewer/?uri=github.com/arcqubit/pqc-scanner

**Key Metrics**:
- ✅ Signed Releases
- ✅ Token Permissions
- ✅ Vulnerability Scanning
- ✅ Code Review
- ✅ Dependency Updates
- ✅ Security Policy
- ✅ SBOM

---

## Continuous Improvement

### Feedback Loops

We continuously improve based on:
1. **Community Feedback**: GitHub Discussions, issues
2. **Metrics**: Response times, contribution velocity
3. **Security Scans**: Automated security findings
4. **Best Practices**: OpenSSF, OSSF guidelines
5. **User Surveys**: Periodic community surveys (future)

### Process Evolution

Community standards are living documents:
- Reviewed quarterly
- Updated based on feedback
- Changes announced in Discussions
- Major changes require community input

### Suggestions Welcome

To suggest improvements to community standards:
1. Open a GitHub Discussion in "Ideas" category
2. Describe the improvement
3. Provide rationale and examples
4. Allow community feedback (1-2 weeks)
5. Maintainers decide on adoption

---

## Additional Resources

### OpenSSF Resources
- [Best Practices Badge](https://bestpractices.coreinfrastructure.org/)
- [Scorecard](https://securityscorecards.dev/)
- [Supply Chain Security](https://openssf.org/)

### Community Building
- [Open Source Guides](https://opensource.guide/)
- [TODO Group](https://todogroup.org/)
- [CNCF Community](https://www.cncf.io/people/ambassadors/)

### Governance Models
- [Open Governance](https://opengovernance.dev/)
- [Contributor Covenant](https://www.contributor-covenant.org/)
- [Open Source Way](https://www.theopensourceway.org/)

---

## Contact

For questions about community standards:
- **GitHub Discussions**: [Community category](https://github.com/arcqubit/pqc-scanner/discussions)
- **Email**: community@arcqubit.io (future)
- **Maintainers**: @arcqubit

---

**Last Updated**: 2025-11-17
**Version**: 1.0.0
**Maintained by**: PQC Scanner Maintainers
