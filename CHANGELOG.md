# Changelog

## [2025.12.1-beta.1](https://github.com/arcqubit/pqc-scanner/compare/pqc-scanner-v2025.12.0-beta.1...pqc-scanner-v2025.12.1-beta.1) (2025-11-18)


### ⚠ BREAKING CHANGES

* Version numbering changed from SemVer to CalVer

### Features

* Add Canadian CCCS/CSE Cryptographic Compliance Support (ITSG-33, ITSP.40.111, ITSP.40.062) ([#9](https://github.com/arcqubit/pqc-scanner/issues/9)) ([281ebe1](https://github.com/arcqubit/pqc-scanner/commit/281ebe12abe87d1c8ce42fbdfd0d92330eab5e60))
* add multi-platform CI/CD automation support ([4b2510e](https://github.com/arcqubit/pqc-scanner/commit/4b2510e164368dd6fd9b9ffdb5ba6962537e9232))
* **ci:** implement automated release management with Release Please ([bc13e58](https://github.com/arcqubit/pqc-scanner/commit/bc13e58c421625dbb793292f085ce9f500bb5eea))
* **community:** Add comprehensive community standards and templates ([#24](https://github.com/arcqubit/pqc-scanner/issues/24)) ([2750dd6](https://github.com/arcqubit/pqc-scanner/commit/2750dd6a39b6cb9d7fe8654b324bd7a69324165f))
* Implement Calendar Versioning (CalVer) scheme ([73f1742](https://github.com/arcqubit/pqc-scanner/commit/73f1742e12f106184c805d783e51438d7e3c8945))
* **security:** Add TruffleHog and Trivy comprehensive security scanning ([#11](https://github.com/arcqubit/pqc-scanner/issues/11)) ([6270197](https://github.com/arcqubit/pqc-scanner/commit/6270197f82d19d629499e9a82a13f0324be7f0e3))
* **security:** Comprehensive OpenSSF Scorecard Improvements ([#10](https://github.com/arcqubit/pqc-scanner/issues/10)) ([f2338aa](https://github.com/arcqubit/pqc-scanner/commit/f2338aadf5d831210db5700a053d4546b6fbe4b1))


### Bug Fixes

* **ci:** Improve cargo-audit workflow efficiency and reliability ([ad449f3](https://github.com/arcqubit/pqc-scanner/commit/ad449f3571d5d3c005f2c19c4bb23e52b8a2403f))
* **ci:** Pin cargo-audit to v0.22.0 ([0f8c79c](https://github.com/arcqubit/pqc-scanner/commit/0f8c79c4c2574f157a3a87acb42158015361468f))
* **ci:** remove deprecated package-name parameter from Release Please ([6b5d514](https://github.com/arcqubit/pqc-scanner/commit/6b5d5143a18ac5175380b60d8ca452a4da0af7b3))
* **ci:** Remove redundant SARIF upload step in CodeQL workflow ([ec22685](https://github.com/arcqubit/pqc-scanner/commit/ec226851ee0badcbf8f4073803bf9443487139c1))
* **ci:** Update rustsec/audit-check to correct commit SHA ([6d642ed](https://github.com/arcqubit/pqc-scanner/commit/6d642ed903eb4ee11416d128d0270a215e344cd7))
* **ci:** Update upload-artifact action to correct commit SHA ([7f651ac](https://github.com/arcqubit/pqc-scanner/commit/7f651ac1140ad151bedebd132c733583e7f8f51e))
* **docs:** Make CalVer badge dynamic and create draft release ([#30](https://github.com/arcqubit/pqc-scanner/issues/30)) ([54b7e9a](https://github.com/arcqubit/pqc-scanner/commit/54b7e9a15a19bc6c0f9c711f8a3a6dc409fd3b1d))
* Update GitHub badges to use correct repository name ([e5b3f4d](https://github.com/arcqubit/pqc-scanner/commit/e5b3f4dcac8dca72ea5828c9387ffd3d3617a21f))
* **workflows:** Update actions/upload-artifact from v3 to v5 ([#32](https://github.com/arcqubit/pqc-scanner/issues/32)) ([44d32e6](https://github.com/arcqubit/pqc-scanner/commit/44d32e6a2b894c7affee85666fbc91a5d19fcef0))
* **workflows:** Update trivy-action to v0.33.1 and fix CodeQL configuration ([#25](https://github.com/arcqubit/pqc-scanner/issues/25)) ([a5bbb6e](https://github.com/arcqubit/pqc-scanner/commit/a5bbb6ead78cdb439c2865ab9b23b3240d7e26f7))


### Documentation

* Samples repository migration plan ([#26](https://github.com/arcqubit/pqc-scanner/issues/26)) ([5f25b63](https://github.com/arcqubit/pqc-scanner/commit/5f25b6335182aaf985631cfaa8e6f624acf873ac))


### Miscellaneous

* **deps:** Complete Phase 3-5 dependency updates ([c5999f3](https://github.com/arcqubit/pqc-scanner/commit/c5999f3909b53a4ba9fd302600347d246093c51c))
* **deps:** Remove unused Express and CORS dependencies (Phase 1-2) ([4270670](https://github.com/arcqubit/pqc-scanner/commit/4270670ccf9bb69a519892634462cba84f5d49eb))
* Downgrade scorecard-action to v2.4.3 ([3594815](https://github.com/arcqubit/pqc-scanner/commit/3594815075a483fefa2befc8a52fbd948887463a))
* **main:** release 2025.12.0-beta.1 ([#33](https://github.com/arcqubit/pqc-scanner/issues/33)) ([ba38ebe](https://github.com/arcqubit/pqc-scanner/commit/ba38ebe6e4e37b408711af9fbf224b1eb0bfa23b))
* Release version 2025.11.0-beta.1 ([785132b](https://github.com/arcqubit/pqc-scanner/commit/785132b43ff816a78b9eb7b0adea154954c1b5a7))

## [2025.12.0-beta.1](https://github.com/arcqubit/pqc-scanner/compare/v2025.11.0-beta.1...v2025.12.0-beta.1) (2025-11-18)


### Features

* **ci:** implement automated release management with Release Please ([bc13e58](https://github.com/arcqubit/pqc-scanner/commit/bc13e58c421625dbb793292f085ce9f500bb5eea))
* **community:** Add comprehensive community standards and templates ([#24](https://github.com/arcqubit/pqc-scanner/issues/24)) ([2750dd6](https://github.com/arcqubit/pqc-scanner/commit/2750dd6a39b6cb9d7fe8654b324bd7a69324165f))
* **security:** Add TruffleHog and Trivy comprehensive security scanning ([#11](https://github.com/arcqubit/pqc-scanner/issues/11)) ([6270197](https://github.com/arcqubit/pqc-scanner/commit/6270197f82d19d629499e9a82a13f0324be7f0e3))
* **security:** Comprehensive OpenSSF Scorecard Improvements ([#10](https://github.com/arcqubit/pqc-scanner/issues/10)) ([f2338aa](https://github.com/arcqubit/pqc-scanner/commit/f2338aadf5d831210db5700a053d4546b6fbe4b1))


### Bug Fixes

* **docs:** Make CalVer badge dynamic and create draft release ([#30](https://github.com/arcqubit/pqc-scanner/issues/30)) ([54b7e9a](https://github.com/arcqubit/pqc-scanner/commit/54b7e9a15a19bc6c0f9c711f8a3a6dc409fd3b1d))
* **workflows:** Update actions/upload-artifact from v3 to v5 ([#32](https://github.com/arcqubit/pqc-scanner/issues/32)) ([44d32e6](https://github.com/arcqubit/pqc-scanner/commit/44d32e6a2b894c7affee85666fbc91a5d19fcef0))
* **workflows:** Update trivy-action to v0.33.1 and fix CodeQL configuration ([#25](https://github.com/arcqubit/pqc-scanner/issues/25)) ([a5bbb6e](https://github.com/arcqubit/pqc-scanner/commit/a5bbb6ead78cdb439c2865ab9b23b3240d7e26f7))
