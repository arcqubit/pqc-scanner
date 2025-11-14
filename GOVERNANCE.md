# Project Governance

This document describes the governance structure and processes for the PQC Scanner project.

## Table of Contents

- [Project Vision](#project-vision)
- [Roles and Responsibilities](#roles-and-responsibilities)
- [Decision Making](#decision-making)
- [Change Process](#change-process)
- [Maintainer Responsibilities](#maintainer-responsibilities)
- [Release Process](#release-process)
- [Conflict Resolution](#conflict-resolution)
- [Amending Governance](#amending-governance)

## Project Vision

PQC Scanner aims to help organizations prepare for the post-quantum cryptography era by:

1. Detecting quantum-vulnerable cryptographic algorithms in codebases
2. Providing actionable remediation guidance
3. Supporting multiple programming languages and platforms
4. Maintaining high security and code quality standards
5. Building an active, inclusive open-source community

## Roles and Responsibilities

### Users

Anyone who uses the PQC Scanner tool. Users are encouraged to:

- Report bugs and security vulnerabilities
- Suggest features and improvements
- Participate in discussions
- Help other users in the community

### Contributors

Anyone who contributes code, documentation, or other resources. Contributors:

- Follow the [Contributing Guidelines](CONTRIBUTING.md)
- Adhere to the [Code of Conduct](CODE_OF_CONDUCT.md)
- Have their contributions reviewed by maintainers
- Retain copyright to their contributions under the MIT License

### Committers

Trusted contributors with commit access to the repository. Committers:

- **Requirements**:
  - Sustained high-quality contributions over 3+ months
  - Deep understanding of project architecture
  - Consistent adherence to project standards
  - Positive community engagement

- **Responsibilities**:
  - Review and merge pull requests
  - Triage issues and discussions
  - Mentor new contributors
  - Maintain code quality standards
  - Participate in project decisions

- **Current Committers**:
  - @arcqubit (Project Lead)

### Maintainers

Core team members responsible for project direction. Maintainers:

- **Requirements**:
  - Demonstrated leadership and technical expertise
  - Sustained committer status for 6+ months
  - Significant project contributions
  - Community trust and respect

- **Responsibilities**:
  - Set project direction and roadmap
  - Make final decisions on disputes
  - Grant/revoke committer status
  - Manage releases
  - Ensure project sustainability
  - Represent project externally

- **Current Maintainers**:
  - @arcqubit (Founder & Lead Maintainer)

### Security Response Team

Dedicated team handling security vulnerabilities. Members:

- Review and triage security reports
- Coordinate vulnerability fixes
- Manage coordinated disclosure
- Publish security advisories

**Contact**: security@arcqubit.io

## Decision Making

### Consensus-Based Decision Making

The project operates on **lazy consensus**:

1. **Proposal**: Propose changes via issue or PR
2. **Discussion**: Community discusses (minimum 72 hours for major changes)
3. **Consensus**: If no objections, proposal is accepted
4. **Action**: Proposer or maintainer implements the change

### Voting

For contentious issues, maintainers may call for a vote:

- **Simple Majority**: Most decisions (>50% approval)
- **Supermajority**: Governance changes, maintainer additions (≥66% approval)
- **Voting Period**: Minimum 7 days
- **Quorum**: At least 50% of maintainers must vote

### Veto Rights

Maintainers may veto decisions that:

- Violate project principles
- Compromise security or stability
- Break backward compatibility without justification

Vetoes must be explained and can be overridden by supermajority vote.

## Change Process

### Small Changes

**Examples**: Bug fixes, documentation updates, minor refactoring

**Process**:
1. Submit a pull request
2. Pass automated checks (CI, tests, linting)
3. Get approval from 1 committer
4. Merge

**Timeline**: Hours to days

### Medium Changes

**Examples**: New features, significant refactoring, API changes

**Process**:
1. Open an issue to discuss the change
2. Get buy-in from maintainers
3. Submit a pull request with implementation
4. Get approval from 1 maintainer
5. Merge after 48-hour review period

**Timeline**: Days to weeks

### Large Changes

**Examples**: Architectural changes, major features, breaking changes

**Process**:
1. Create an RFC (Request for Comments) issue
2. Include:
   - Problem statement
   - Proposed solution
   - Alternatives considered
   - Implementation plan
   - Migration strategy (if breaking)
3. Community discussion (minimum 7 days)
4. Maintainer approval via consensus or vote
5. Implementation via PRs
6. Documentation and migration guides

**Timeline**: Weeks to months

## Maintainer Responsibilities

### Code Review

Maintainers review PRs for:

- **Correctness**: Code works as intended
- **Tests**: Adequate test coverage (≥80%)
- **Security**: No vulnerabilities introduced
- **Performance**: No significant regressions
- **Documentation**: Public APIs documented
- **Style**: Follows project conventions

### Issue Triage

Maintainers triage issues by:

- Confirming bug reports
- Prioritizing issues (P0-Critical, P1-High, P2-Medium, P3-Low)
- Adding appropriate labels
- Assigning to committers or milestones
- Closing duplicates or invalid issues

### Release Management

Maintainers coordinate releases:

1. Plan release scope and timeline
2. Ensure tests pass and security scans clean
3. Update CHANGELOG.md
4. Create release tag
5. Publish to GitHub Releases
6. Update documentation
7. Announce to community

### Community Management

Maintainers foster community by:

- Welcoming new contributors
- Mentoring and supporting contributors
- Enforcing Code of Conduct
- Responding to questions and discussions
- Promoting project externally

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/) (SemVer):

- **MAJOR**: Breaking changes (e.g., 2.0.0)
- **MINOR**: New features, backward compatible (e.g., 1.3.0)
- **PATCH**: Bug fixes, backward compatible (e.g., 1.2.1)

### Release Cycle

- **Major**: Annually or as needed
- **Minor**: Every 2-3 months
- **Patch**: As needed for bug fixes and security issues

### Release Checklist

- [ ] All tests pass
- [ ] Security scans clean (cargo audit, CodeQL)
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Release notes drafted
- [ ] Tag created (e.g., v1.2.0)
- [ ] GitHub Release published
- [ ] Artifacts uploaded (binaries, WASM, SBOM)
- [ ] Documentation updated
- [ ] Community announcement

### Security Releases

Critical security fixes follow expedited process:

1. Fix developed privately by Security Response Team
2. Coordinated disclosure with reporters
3. Patch release prepared
4. Security advisory published
5. Announcement to community

## Conflict Resolution

### Resolution Process

1. **Direct Communication**: Parties attempt to resolve directly
2. **Mediation**: Uninvolved maintainer mediates
3. **Maintainer Decision**: Maintainers vote if mediation fails
4. **Code of Conduct Enforcement**: CoC committee handles violations

### Grounds for Removal

Committers or maintainers may be removed for:

- Repeated Code of Conduct violations
- Abuse of access privileges
- Sustained inactivity (6+ months without communication)
- Violation of project security policies

Removal requires supermajority (≥66%) maintainer vote.

## Maintainer Nomination

### Nomination Process

1. **Self-nomination or nomination by existing maintainer**
2. **Nomination announcement** with candidate background
3. **Discussion period** (minimum 14 days)
4. **Vote by existing maintainers** (≥66% approval required)
5. **Announcement** of decision

### Nomination Criteria

- Demonstrated technical expertise
- Sustained high-quality contributions (6+ months)
- Strong understanding of project architecture
- Positive community interactions
- Alignment with project values
- Time commitment (≥5 hours/week)

## Amending Governance

This governance document may be amended by:

1. Proposal submitted as a pull request
2. Discussion period (minimum 14 days)
3. Supermajority approval (≥66%) from maintainers
4. Announcement to community

## Credits and Attribution

This governance model is inspired by:

- [Apache Software Foundation Governance](https://www.apache.org/foundation/governance/)
- [CNCF Project Governance](https://github.com/cncf/project-template/blob/main/GOVERNANCE.md)
- [Kubernetes Governance](https://github.com/kubernetes/community/blob/master/governance.md)

## Contact

- **General Questions**: support@arcqubit.io
- **Governance Questions**: governance@arcqubit.io
- **Security**: security@arcqubit.io
- **Code of Conduct**: conduct@arcqubit.io

---

**Document Version**: 1.0.0
**Last Updated**: 2025-11-11
**Next Review**: 2026-05-11
