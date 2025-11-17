# Canadian CCCS/CSE Cryptographic Compliance

## Overview

The PQC Scanner now includes comprehensive support for Canadian cryptographic compliance frameworks, enabling organizations to assess compliance with:

- **ITSG-33 SC-13** - Cryptographic Protection control (Canadian equivalent to NIST 800-53 SC-13)
- **ITSP.40.111** - Cryptographic Algorithms for UNCLASSIFIED, PROTECTED A, and PROTECTED B Information
- **ITSP.40.062** - Guidance on Securely Configuring Network Protocols
- **CMVP** - Cryptographic Module Validation Program (joint NIST/CCCS program)

## Key Features

### ✅ Implemented Features (Phase 1)

1. **Security Classification Support**
   - Unclassified
   - Protected A
   - Protected B
   - Protected C

2. **CCCS Algorithm Validation**
   - Approved algorithms (AES, SHA-2, SHA-3, HMAC)
   - Conditionally approved (RSA, ECDSA, ECDH, DH) with sunset dates
   - Deprecated algorithms (3DES, DSA)
   - Prohibited algorithms (MD5, SHA-1, DES, RC4)
   - Post-quantum algorithms under review (CRYSTALS-Kyber, CRYSTALS-Dilithium, SPHINCS+)

3. **Classification-Specific Requirements**
   - Minimum key sizes per classification level
   - CMVP validation requirements
   - Approved hash functions per classification

4. **Unified Compliance Reporting**
   - Generates both NIST 800-53 SC-13 and ITSG-33 SC-13 reports
   - Cross-mapping between frameworks
   - Unified recommendations

5. **CMVP Certificate Database**
   - Offline certificate lookup
   - Library-to-CMVP mapping
   - Support for major cryptographic libraries (OpenSSL, BouncyCastle, AWS-LC, etc.)

6. **Comprehensive Evidence Collection**
   - Algorithm inventory with CCCS approval status
   - ITSP reference citations
   - Classification-specific compliance assessment

## Usage

### Command Line (Rust)

```rust
use pqc_scanner::{
    analyze,
    generate_itsg33_report,
    generate_unified_report,
    SecurityClassification,
    export_itsg33_json,
    export_unified_json
};

// Analyze code
let source = r#"
    const hash = crypto.createHash('md5');
    const rsa = crypto.generateKeyPair('rsa', { modulusLength: 2048 });
"#;

let audit_result = analyze(source, "javascript").unwrap();

// Generate ITSG-33 report for Protected A classification
let itsg33_report = generate_itsg33_report(
    &audit_result,
    SecurityClassification::ProtectedA,
    Some("example.js")
);

// Generate unified NIST + Canadian report
let unified_report = generate_unified_report(
    &audit_result,
    SecurityClassification::ProtectedB,
    Some("example.js")
);

// Export to JSON
let json = export_itsg33_json(&itsg33_report).unwrap();
println!("{}", json);
```

### WebAssembly (JavaScript/TypeScript)

```javascript
import init, {
    generate_itsg33_compliance_report,
    generate_unified_compliance_report
} from './pqc_scanner.js';

await init();

const source = `
    const hash = crypto.createHash('md5');
    const rsa = crypto.generateKeyPair('rsa', { modulusLength: 2048 });
`;

// Generate ITSG-33 report
const itsg33Report = generate_itsg33_compliance_report(
    source,
    'javascript',
    'protected-a',
    'example.js'
);

console.log('ITSG-33 Compliance Score:', itsg33Report.summary.compliance_score);
console.log('ITSP.40.111 Compliant:', itsg33Report.summary.itsp_40_111_compliant);

// Generate unified report
const unifiedReport = generate_unified_compliance_report(
    source,
    'javascript',
    'protected-b',
    'example.js'
);

console.log('Control Mapping:', unifiedReport.control_mapping);
```

## Security Classifications

### Unclassified
- **Minimum AES Key Size:** 128 bits
- **Minimum RSA Key Size:** 2048 bits
- **Minimum ECC Key Size:** 256 bits
- **Approved Hash:** SHA-256, SHA-384, SHA-512, SHA3-256
- **CMVP Required:** No

### Protected A
- **Minimum AES Key Size:** 128 bits
- **Minimum RSA Key Size:** 2048 bits
- **Minimum ECC Key Size:** 256 bits
- **Approved Hash:** SHA-256, SHA-384, SHA-512
- **CMVP Required:** Yes

### Protected B
- **Minimum AES Key Size:** 256 bits
- **Minimum RSA Key Size:** 3072 bits
- **Minimum ECC Key Size:** 384 bits
- **Approved Hash:** SHA-384, SHA-512
- **CMVP Required:** Yes

### Protected C
- **Minimum AES Key Size:** 256 bits
- **Minimum RSA Key Size:** 4096 bits
- **Minimum ECC Key Size:** 521 bits
- **Approved Hash:** SHA-512
- **CMVP Required:** Yes

## Algorithm Approval Status

### ✅ Approved
- **AES** (128, 192, 256-bit) in GCM, CCM, CBC, CTR modes
- **SHA-2** (SHA-224, SHA-256, SHA-384, SHA-512)
- **SHA-3** (SHA3-224, SHA3-256, SHA3-384, SHA3-512)
- **HMAC** (with SHA-256, SHA-384, SHA-512)

### ⚠️ Conditionally Approved (Quantum-Vulnerable)
- **RSA** (2048, 3072, 4096-bit) - Legacy systems only, migrate by 2030
- **ECDSA** (P-256, P-384, P-521) - Plan PQC migration by 2030
- **ECDH** (P-256, P-384, P-521) - Ephemeral keys required, migrate by 2030
- **Diffie-Hellman** (2048, 3072, 4096-bit) - DHE preferred, migrate by 2030

### 🔻 Deprecated
- **3DES** - No longer approved for new systems, sunset 2023
- **DSA** - Migrate to ECDSA or PQC alternatives, sunset 2024

### 🚫 Prohibited
- **MD5** - Cryptographically broken, immediate migration required
- **SHA-1** - Collision vulnerabilities, immediate migration required
- **DES** - Insufficient key length, prohibited
- **RC4** - Keystream biases, prohibited

### 🔬 Under Review (Post-Quantum)
- **CRYSTALS-Kyber** - Key encapsulation mechanism
- **CRYSTALS-Dilithium** - Digital signature scheme
- **SPHINCS+** - Stateless hash-based signatures

## Report Structure

### ITSG-33 Report

```typescript
interface ITSG33Report {
    metadata: ReportMetadata;
    control_assessment: ITSG33ControlAssessment;
    summary: CanadianAssessmentSummary;
    findings: CanadianFinding[];
    protocol_compliance: ProtocolCompliance[];
    cmvp_validations: CMVPValidation[];
    recommendations: string[];
}

interface CanadianAssessmentSummary {
    // Standard fields
    files_scanned: number;
    lines_scanned: number;
    total_vulnerabilities: number;
    quantum_vulnerable_algorithms: string[];
    deprecated_algorithms: string[];
    weak_key_sizes: string[];
    compliance_score: number;
    risk_score: number;

    // Canadian-specific fields
    cccs_approved_algorithms: string[];
    cccs_deprecated_algorithms: string[];
    cccs_prohibited_algorithms: string[];
    cmvp_validated_count: number;
    cmvp_required_count: number;
    itsp_40_111_compliant: boolean;
    itsp_40_062_compliant: boolean;
    security_classification: SecurityClassification;
    classification_compliant: boolean;
}

interface CanadianFinding {
    finding_id: string;
    control_id: string;
    implementation_status: ImplementationStatus;
    assessment_status: AssessmentStatus;
    description: string;
    related_vulnerabilities: string[];
    evidence: Evidence[];
    remediation: string;
    risk_level: Severity;

    // Canadian-specific fields
    cccs_approval_status: CCCSApprovalStatus;
    itsp_references: string[];
    cmvp_validation?: CMVPValidation;
    applicable_classifications: SecurityClassification[];
}
```

### Unified Report

```typescript
interface UnifiedComplianceReport {
    metadata: ReportMetadata;

    // NIST components
    nist_sc13_assessment: ControlAssessment;
    nist_summary: AssessmentSummary;
    nist_findings: ControlFinding[];

    // Canadian components
    itsg33_sc13_assessment: ITSG33ControlAssessment;
    canadian_summary: CanadianAssessmentSummary;
    canadian_findings: CanadianFinding[];

    // Cross-mapping
    control_mapping: ControlCrossReference[];

    // Unified recommendations
    recommendations: string[];
}
```

## Compliance Scoring

The Canadian compliance score (0-100) is calculated based on:

- **Prohibited algorithms:** -40 points each (MD5, SHA-1, DES, RC4)
- **Deprecated algorithms:** -20 points each (3DES, DSA)
- **Weak key sizes:** -15 points each (below classification minimum)
- **Quantum-vulnerable algorithms:** -10 points per unique type

**Example:**
```
Initial Score: 100
- MD5 detected: -40
- 3DES detected: -20
- RSA 1024-bit (weak for Protected B): -15
- RSA detected (quantum-vulnerable): -10
= Final Score: 15 (Non-compliant)
```

## CMVP Certificate Database

The scanner includes an offline database of CMVP certificates for common cryptographic libraries:

| Certificate # | Vendor | Module | Algorithms |
|---------------|--------|--------|------------|
| 4282 | OpenSSL | FIPS Object Module | AES, RSA, ECDSA, SHA-256/384/512 |
| 3966 | Microsoft | Cryptographic Primitives | AES, RSA, ECDSA, ECDH, SHA |
| 4118 | Bouncy Castle | BC-FJA | AES, RSA, ECDSA, SHA-2/3 |
| 4536 | AWS | AWS-LC | AES, RSA, ECDSA, ECDH, SHA |
| 4407 | wolfSSL | wolfCrypt FIPS | AES, RSA, ECDSA, SHA |

### Library Mappings

- `openssl` → Certificate #4282
- `bouncycastle` → Certificate #4118
- `aws-lc` → Certificate #4536
- `wolfssl` → Certificate #4407
- `java.security` → Certificate #3615

## ITSP References

Each finding includes specific ITSP references:

- **AES:** ITSP.40.111 Annex A
- **RSA:** ITSP.40.111 Section 4.1
- **ECDSA:** ITSP.40.111 Section 4.2
- **ECDH:** ITSP.40.111 Section 4.3
- **DH:** ITSP.40.111 Section 4.4
- **MD5:** ITSP.40.111 Section 5.3
- **SHA-1:** ITSP.40.111 Section 5.2
- **DES:** ITSP.40.111 Section 5.4
- **3DES:** ITSP.40.111 Section 5.5
- **RC4:** ITSP.40.111 Section 5.6
- **Post-Quantum:** ITSP.40.111 Section 6.x

## Control Mapping: NIST ↔ ITSG-33

| NIST 800-53 Rev. 5 | ITSG-33 | Equivalence |
|--------------------|---------|-------------|
| SC-13 Cryptographic Protection | ITSG-33 SC-13 Use of Cryptography | 1:1 mapping |

**Key Differences:**
- ITSG-33 SC-13 explicitly references Canadian security classifications (Protected A/B/C)
- ITSP.40.111 provides specific algorithm approval guidance
- ITSP.40.062 adds protocol configuration requirements
- Both frameworks require FIPS-validated/CMVP-validated crypto

**Shared Requirements:**
- CMVP-validated cryptographic modules (FIPS 140-2/140-3)
- Strong key sizes appropriate for data classification
- Approved cryptographic algorithms
- Crypto-agility and lifecycle management

## Example Reports

### ITSG-33 Report Sample

```json
{
  "metadata": {
    "report_id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "ITSG-33 SC-13 Cryptographic Protection Assessment - Protected A",
    "published": "2025-11-17T10:30:00Z",
    "version": "1.0.0",
    "oscal_version": "1.1.2"
  },
  "control_assessment": {
    "control_id": "ITSG-33 SC-13",
    "control_name": "Use of Cryptography / Cryptographic Protection",
    "security_classification": "PROTECTED_A",
    "nist_mapping": "NIST 800-53 Rev. 5 SC-13"
  },
  "summary": {
    "total_vulnerabilities": 2,
    "compliance_score": 25,
    "cccs_prohibited_algorithms": ["MD5"],
    "itsp_40_111_compliant": false,
    "classification_compliant": false
  },
  "findings": [
    {
      "finding_id": "...",
      "control_id": "ITSG-33 SC-13",
      "cccs_approval_status": "prohibited",
      "itsp_references": ["ITSP.40.111 Section 5.3"],
      "applicable_classifications": []
    }
  ]
}
```

## Roadmap

### Future Enhancements

1. **Protocol Detection (ITSP.40.062)**
   - TLS/SSL version detection
   - Cipher suite validation
   - SSH protocol configuration
   - IPSec configuration checks

2. **CMVP Online Validation**
   - Real-time API integration with NIST/CCCS CMVP
   - Automatic certificate updates
   - Expiry tracking

3. **Bilingual Reporting (EN/FR)**
   - French language support
   - Bilingual PDF/HTML reports
   - Canadian government branding

4. **Evidence Packages**
   - Compliance evidence bundles
   - Audit-ready documentation
   - Attestation templates

5. **Export Formats**
   - CSV/Excel exports
   - PDF reports
   - HTML dashboards

## References

- [ITSG-33 Annex 3A - Security Control Catalogue](https://www.cyber.gc.ca/en/guidance/annex-3a-security-control-catalogue-itsg-33)
- [ITSP.40.111 - Cryptographic Algorithms for UNCLASSIFIED, PROTECTED A, and PROTECTED B Information](https://www.cyber.gc.ca/sites/default/files/itsp.40.111-e_0.pdf)
- [ITSP.40.062 - Guidance on Securely Configuring Network Protocols](https://www.cyber.gc.ca/en/guidance/guidance-securely-configuring-network-protocols-itsp40062)
- [CMVP - Cryptographic Module Validation Program](https://www.cyber.gc.ca/en/tools-services/cryptographic-module-validation-program-cmvp)
- [NIST 800-53 Rev. 5 - Security and Privacy Controls](https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final)

## Support

For questions, issues, or feature requests related to Canadian compliance features:
- GitHub Issues: https://github.com/arcqubit/pqc-scanner/issues
- Documentation: https://github.com/arcqubit/pqc-scanner/docs

## License

MIT License - See LICENSE file for details
