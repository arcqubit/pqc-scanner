# Polyglot Application - Multi-Language Vulnerable Sample

**WARNING**: This is an intentionally vulnerable application for testing purposes. DO NOT use any of this code in production.

## Overview

A multi-language monorepo demonstrating cryptographic vulnerabilities across different programming languages and tech stacks. This sample showcases the PQC Scanner's ability to detect inconsistent security practices across a polyglot codebase.

## Technology Stack

- **Frontend**: JavaScript/TypeScript (React)
- **Backend**: Python (Flask/FastAPI)
- **Microservices**: Go
- **Crypto Library**: Rust
- **Legacy Service**: Java

## Structure

```
polyglot-app/
├── frontend/          # JavaScript - RSA-1024, MD5
├── backend/           # Python - ECDSA, SHA-1
├── microservices/     # Go - DES, weak random
├── lib-crypto/        # Rust - Mixed (some secure, some weak)
└── legacy-service/    # Java - Multiple crypto issues
```

## Intentional Vulnerabilities by Component

### Frontend (JavaScript/TypeScript)
- RSA-1024 for client-side encryption
- MD5 for cache keys and checksums
- Weak random number generation
- **Expected**: 8 vulnerabilities

### Backend (Python)
- ECDSA with P-256 (quantum-vulnerable)
- SHA-1 for API token generation
- Hardcoded secret keys
- RC4 for session encryption
- **Expected**: 10 vulnerabilities

### Microservices (Go)
- DES encryption for inter-service communication
- MD5 for service discovery hashing
- Weak random for request IDs
- No forward secrecy in TLS
- **Expected**: 7 vulnerabilities

### Crypto Library (Rust)
- Mixed implementation (demonstrating partial migration)
- Some secure post-quantum algorithms
- Some legacy weak algorithms for compatibility
- **Expected**: 5 vulnerabilities

### Legacy Service (Java)
- 3DES for data encryption
- DSA-1024 signatures
- SHA-1 for logging and audit
- Weak TLS configuration
- **Expected**: 8 vulnerabilities

## Total Expected Results

- **Total Vulnerabilities**: 35+
- **Compliance Score**: 31/100
- **Risk Score**: 76/100
- **Demonstrates**: Cross-language consistency issues

## Key Findings

This sample repository demonstrates several real-world challenges:

1. **Inconsistent Security Levels**: Different teams using different crypto standards
2. **Legacy Compatibility**: Old services dragging down overall security posture
3. **Language-Specific Patterns**: Each language has common crypto anti-patterns
4. **Mixed Migration State**: Partial PQC adoption creating security gaps

## Scanning with PQC Scanner

```bash
# Scan entire polyglot repository
cargo run --example generate_compliance_report -- \
  --path samples/polyglot-app/ \
  --output reports/polyglot-app-report.json

# Scan by language
cargo run --example generate_compliance_report -- \
  --path samples/polyglot-app/frontend/ \
  --output reports/frontend-report.json

cargo run --example generate_compliance_report -- \
  --path samples/polyglot-app/backend/ \
  --output reports/backend-report.json
```

## Remediation Strategy

For a polyglot application, remediation should follow a unified crypto policy:

1. **Establish Baseline**: Minimum security standards across all languages
2. **Prioritize by Risk**: Fix critical vulnerabilities in all components first
3. **Unified Migration**: Coordinate PQC adoption across all services
4. **Shared Libraries**: Create common crypto libraries when possible
5. **Continuous Monitoring**: Regular scanning of all components

## Expected Remediation Impact

After applying unified crypto policy:
- Frontend: 100/100 (AES-256-GCM, SHA-256, crypto.randomBytes)
- Backend: 100/100 (Kyber-768, Dilithium2, proper key management)
- Microservices: 100/100 (AES-256-GCM, TLS 1.3 with forward secrecy)
- Crypto Library: 100/100 (Full PQC implementation)
- Legacy Service: 98/100 (Some constraints due to framework limitations)

**Final Overall Score: 100/100**

## Building and Testing

Each component has its own build system:

```bash
# Frontend
cd frontend && npm install && npm test

# Backend
cd backend && pip install -r requirements.txt && pytest

# Microservices
cd microservices && go build && go test

# Crypto Library
cd lib-crypto && cargo build && cargo test

# Legacy Service
cd legacy-service && mvn clean install && mvn test
```

## Notes for Developers

This sample demonstrates why **consistent security policies** and **automated scanning** are critical in polyglot environments. Without tooling like the PQC Scanner, vulnerabilities can easily slip through in the gaps between languages and teams.

## License

MIT - For testing purposes only
