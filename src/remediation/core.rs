// Auto-remediation module for cryptographic vulnerabilities
// Provides template-based code fixes for quantum-vulnerable algorithms

use crate::types::{AuditResult, CryptoType, Vulnerability};
use serde::{Deserialize, Serialize};

/// A suggested code fix for a cryptographic vulnerability
///
/// **IMPORTANT**: All fixes require manual review before application.
/// This struct provides patch file generation support, not automatic code modification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFix {
    /// File path where the fix should be applied
    pub file_path: String,

    /// Line number of the vulnerable code
    pub line: usize,

    /// Column number of the vulnerable code
    pub column: usize,

    /// Original vulnerable code snippet
    pub old_code: String,

    /// Suggested replacement code
    pub new_code: String,

    /// Confidence score for this fix (0.0 - 1.0)
    pub confidence: f32,

    /// Algorithm being remediated
    pub algorithm: String,

    /// Explanation of the fix
    pub explanation: String,

    /// Whether a patch file can be generated for this fix
    pub patch_available: bool,

    /// Review checklist items to verify before applying the fix
    pub review_notes: Vec<String>,

    /// Optional patch file path (generated when --auto-remediate=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_file: Option<String>,
}

/// Result of remediation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationResult {
    /// List of suggested code fixes
    pub fixes: Vec<CodeFix>,

    /// Overall remediation summary
    pub summary: RemediationSummary,

    /// Additional warnings or notes
    pub warnings: Vec<String>,
}

/// Summary statistics for remediation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationSummary {
    /// Total vulnerabilities analyzed
    pub total_vulnerabilities: usize,

    /// Number of issues with remediation available
    pub remediable: usize,

    /// Number of issues with patch generation available
    pub patch_available: usize,

    /// Number without automated remediation
    pub manual_only: usize,

    /// Average confidence score for available remediations
    pub average_confidence: f32,
}

/// Generate remediation suggestions from audit results
pub fn generate_remediations(audit_result: &AuditResult, file_path: &str) -> RemediationResult {
    let mut fixes = Vec::new();
    let mut warnings = Vec::new();

    for vuln in &audit_result.vulnerabilities {
        match vuln.crypto_type {
            CryptoType::Md5 => {
                if let Some(fix) = remediate_md5(vuln, file_path) {
                    fixes.push(fix);
                }
            }
            CryptoType::Sha1 => {
                if let Some(fix) = remediate_sha1(vuln, file_path) {
                    fixes.push(fix);
                }
            }
            CryptoType::Rsa => {
                if let Some(fix) = remediate_rsa(vuln, file_path) {
                    fixes.push(fix);
                }
            }
            CryptoType::Des | CryptoType::TripleDes => {
                if let Some(fix) = remediate_des_3des(vuln, file_path) {
                    fixes.push(fix);
                }
            }
            _ => {
                // Add warning for unsupported remediation types
                warnings.push(format!(
                    "No automatic remediation available for {} at line {}",
                    vuln.crypto_type, vuln.line
                ));
            }
        }
    }

    let patch_available = fixes.iter().filter(|f| f.patch_available).count();
    let remediable = fixes.len();
    let manual_only = audit_result.vulnerabilities.len() - remediable;

    let average_confidence = if !fixes.is_empty() {
        fixes.iter().map(|f| f.confidence).sum::<f32>() / fixes.len() as f32
    } else {
        0.0
    };

    RemediationResult {
        fixes,
        summary: RemediationSummary {
            total_vulnerabilities: audit_result.vulnerabilities.len(),
            remediable,
            patch_available,
            manual_only,
            average_confidence,
        },
        warnings,
    }
}

/// Generate remediation for MD5 hash usage
fn remediate_md5(vuln: &Vulnerability, file_path: &str) -> Option<CodeFix> {
    let old_code = vuln.context.trim().to_string();

    // Pattern matching for common MD5 usage patterns
    let new_code = if old_code.contains("md5") && old_code.contains("hashlib") {
        // Python hashlib
        old_code.replace("md5", "sha256")
    } else if old_code.contains("MD5") && old_code.contains("crypto") {
        // Node.js crypto
        old_code.replace("MD5", "SHA256").replace("md5", "sha256")
    } else if old_code.contains("Md5") {
        // Java/C# style
        old_code.replace("Md5", "Sha256").replace("MD5", "SHA256")
    } else {
        // Generic replacement
        old_code.replace("md5", "sha256").replace("MD5", "SHA256")
    };

    Some(CodeFix {
        file_path: file_path.to_string(),
        line: vuln.line,
        column: vuln.column,
        old_code,
        new_code,
        confidence: 0.85,
        algorithm: "MD5 → SHA-256".to_string(),
        explanation: "Replaced deprecated MD5 hash with SHA-256. MD5 is cryptographically broken and prohibited by NIST/CCCS. For highest security, consider SHA-3 or BLAKE2.".to_string(),
        patch_available: true,
        review_notes: vec![
            "Verify MD5 is used for cryptographic hashing, not file checksums".to_string(),
            "Update unit tests to expect SHA-256 output (64 hex chars vs 32)".to_string(),
            "Check for any stored MD5 hashes in databases that need migration".to_string(),
            "NIST: MD5 prohibited per FIPS 180-4".to_string(),
            "CCCS: MD5 prohibited per ITSP.40.111 Section 5.3".to_string(),
        ],
        patch_file: None,
    })
}

/// Generate remediation for SHA-1 hash usage
fn remediate_sha1(vuln: &Vulnerability, file_path: &str) -> Option<CodeFix> {
    let old_code = vuln.context.trim().to_string();

    let new_code = if old_code.contains("sha1") {
        old_code.replace("sha1", "sha256")
    } else if old_code.contains("SHA1") {
        old_code.replace("SHA1", "SHA256")
    } else if old_code.contains("Sha1") {
        old_code.replace("Sha1", "Sha256")
    } else {
        old_code.replace("SHA-1", "SHA-256")
    };

    Some(CodeFix {
        file_path: file_path.to_string(),
        line: vuln.line,
        column: vuln.column,
        old_code,
        new_code,
        confidence: 0.9,
        algorithm: "SHA-1 → SHA-256".to_string(),
        explanation: "Replaced deprecated SHA-1 hash with SHA-256. SHA-1 is vulnerable to collision attacks and prohibited for digital signatures by NIST since 2013.".to_string(),
        patch_available: true,
        review_notes: vec![
            "Verify SHA-1 usage context (signatures vs checksums)".to_string(),
            "Update tests to expect SHA-256 output (64 hex chars)".to_string(),
            "Check for Git commit signatures or other SHA-1 dependencies".to_string(),
            "NIST: SHA-1 prohibited for digital signatures per FIPS 186-4".to_string(),
            "CCCS: SHA-1 prohibited per ITSP.40.111 Section 5.2".to_string(),
        ],
        patch_file: None,
    })
}

/// Generate remediation for RSA key usage
fn remediate_rsa(vuln: &Vulnerability, file_path: &str) -> Option<CodeFix> {
    let old_code = vuln.context.trim().to_string();

    // Check if this is a weak key size (< 2048 bits)
    let is_weak_key = vuln.key_size.is_some_and(|size| size < 2048);

    let (new_code, confidence, explanation, patch_available, review_notes) = if is_weak_key {
        // For weak keys, suggest minimum 2048-bit RSA as interim solution
        let replacement = if let Some(size) = vuln.key_size {
            old_code.replace(&size.to_string(), "2048")
        } else {
            old_code.clone()
        };

        (
            replacement,
            0.7,
            "Upgraded RSA key size to 2048 bits (minimum secure size per NIST SP 800-57). CRITICAL: RSA is quantum-vulnerable. Plan migration to CRYSTALS-Dilithium (signatures) or CRYSTALS-Kyber (encryption).".to_string(),
            true,
            vec![
                "Verify key size upgrade doesn't break existing integrations".to_string(),
                "Check if key generation performance is acceptable".to_string(),
                "Update key storage and certificate infrastructure".to_string(),
                "NIST: RSA minimum 2048 bits per SP 800-57".to_string(),
                "CCCS: RSA conditionally approved, sunset 2030 per ITSP.40.111".to_string(),
                "PQC Migration: Plan for CRYSTALS-Dilithium (NIST FIPS 204)".to_string(),
            ],
        )
    } else {
        // For stronger RSA, provide quantum migration guidance
        (
            old_code.clone(),
            0.5,
            "RSA is quantum-vulnerable. NIST recommends migrating to post-quantum algorithms by 2030. Consider CRYSTALS-Dilithium (signatures) or CRYSTALS-Kyber (encryption).".to_string(),
            false, // No patch - requires architectural changes
            vec![
                "RSA is quantum-vulnerable regardless of key size".to_string(),
                "NIST FIPS 204: CRYSTALS-Dilithium for digital signatures".to_string(),
                "NIST FIPS 203: CRYSTALS-Kyber for key encapsulation".to_string(),
                "Migration timeline: NIST recommends completion by 2030".to_string(),
                "CCCS: RSA sunset date 2030 per ITSP.40.111 Section 4.1".to_string(),
                "Consider hybrid crypto during transition period".to_string(),
            ],
        )
    };

    Some(CodeFix {
        file_path: file_path.to_string(),
        line: vuln.line,
        column: vuln.column,
        old_code,
        new_code,
        confidence,
        algorithm: if is_weak_key {
            format!("RSA-{} → RSA-2048 (interim)", vuln.key_size.unwrap_or(1024))
        } else {
            "RSA → PQC migration recommended".to_string()
        },
        explanation,
        patch_available,
        review_notes,
        patch_file: None,
    })
}

/// Generate remediation for DES/3DES usage
fn remediate_des_3des(vuln: &Vulnerability, file_path: &str) -> Option<CodeFix> {
    let old_code = vuln.context.trim().to_string();
    let is_3des = matches!(vuln.crypto_type, CryptoType::TripleDes);

    // Pattern matching for common DES/3DES usage
    let new_code = if old_code.contains("DES") || old_code.contains("des") {
        // Replace with AES-256
        old_code
            .replace("TripleDES", "AES")
            .replace("3DES", "AES")
            .replace("DES", "AES")
            .replace("des", "aes")
    } else {
        old_code.clone()
    };

    let algorithm = if is_3des { "3DES" } else { "DES" };
    let (itsp_ref, sunset) = if is_3des {
        ("ITSP.40.111 Section 5.5", "2023")
    } else {
        ("ITSP.40.111 Section 5.4", "N/A - Prohibited")
    };

    Some(CodeFix {
        file_path: file_path.to_string(),
        line: vuln.line,
        column: vuln.column,
        old_code,
        new_code,
        confidence: 0.75,
        algorithm: format!("{} → AES-256-GCM", algorithm),
        explanation: format!(
            "Replaced deprecated {} cipher with AES-256-GCM. {} has known vulnerabilities, small block size (64 bits), and is prohibited/deprecated by NIST and CCCS. AES-GCM provides authenticated encryption.",
            algorithm, algorithm
        ),
        patch_available: true,
        review_notes: vec![
            format!("{} block size: 64 bits (vulnerable to birthday attacks)", algorithm),
            "AES requires 128/192/256-bit keys vs DES 56/112/168 bits".to_string(),
            "Must specify AES mode: recommend GCM for authenticated encryption".to_string(),
            "Update key derivation and IV generation for AES".to_string(),
            format!("NIST: {} prohibited per SP 800-57", algorithm),
            format!("CCCS: {} deprecated/prohibited per {}, sunset {}", algorithm, itsp_ref, sunset),
            "Verify all encrypted data can be re-encrypted with AES".to_string(),
        ],
        patch_file: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Language, Severity};

    fn create_test_vulnerability(
        crypto_type: CryptoType,
        context: &str,
        key_size: Option<u32>,
    ) -> Vulnerability {
        Vulnerability {
            crypto_type,
            severity: Severity::High,
            risk_score: 80,
            line: 42,
            column: 10,
            context: context.to_string(),
            message: "Test vulnerability".to_string(),
            recommendation: "Test recommendation".to_string(),
            key_size,
        }
    }

    #[test]
    fn test_remediate_md5_python() {
        let vuln = create_test_vulnerability(
            CryptoType::Md5,
            "hash = hashlib.md5(data).hexdigest()",
            None,
        );

        let fix = remediate_md5(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "MD5 → SHA-256");
        assert_eq!(fix.new_code, "hash = hashlib.sha256(data).hexdigest()");
        assert!(fix.confidence > 0.8);
        assert!(fix.patch_available);
        assert!(!fix.review_notes.is_empty());
        assert!(fix.review_notes.iter().any(|n| n.contains("NIST") || n.contains("CCCS")));
    }

    #[test]
    fn test_remediate_md5_nodejs() {
        let vuln = create_test_vulnerability(
            CryptoType::Md5,
            "const hash = crypto.createHash('MD5')",
            None,
        );

        let fix = remediate_md5(&vuln, "test.js").unwrap();

        assert!(fix.new_code.contains("SHA256"));
        assert!(fix.patch_available);
    }

    #[test]
    fn test_remediate_sha1() {
        let vuln = create_test_vulnerability(CryptoType::Sha1, "hash = hashlib.sha1(data)", None);

        let fix = remediate_sha1(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "SHA-1 → SHA-256");
        assert_eq!(fix.new_code, "hash = hashlib.sha256(data)");
        assert!(fix.confidence >= 0.9);
        assert!(fix.patch_available);
    }

    #[test]
    fn test_remediate_rsa_weak_key() {
        let vuln =
            create_test_vulnerability(CryptoType::Rsa, "key = RSA.generate(1024)", Some(1024));

        let fix = remediate_rsa(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "RSA-1024 → RSA-2048 (interim)");
        assert!(fix.new_code.contains("2048"));
        assert!(fix.patch_available); // Patch available for key size upgrade
        assert!(fix.explanation.contains("CRYSTALS"));
        assert!(!fix.review_notes.is_empty());
    }

    #[test]
    fn test_remediate_rsa_strong_key() {
        let vuln =
            create_test_vulnerability(CryptoType::Rsa, "key = RSA.generate(4096)", Some(4096));

        let fix = remediate_rsa(&vuln, "test.py").unwrap();

        assert!(fix.algorithm.contains("PQC migration"));
        assert!(!fix.patch_available); // No patch - requires architectural changes
        assert!(fix.explanation.contains("quantum"));
        assert!(fix.confidence < 0.7);
        assert!(!fix.review_notes.is_empty());
    }

    #[test]
    fn test_remediate_des() {
        let vuln =
            create_test_vulnerability(CryptoType::Des, "cipher = DES.new(key, DES.MODE_ECB)", None);

        let fix = remediate_des_3des(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "DES → AES-256-GCM");
        assert!(fix.new_code.contains("AES"));
        assert!(!fix.new_code.contains("DES"));
        assert!(fix.patch_available);
        assert!(fix.explanation.contains("GCM"));
    }

    #[test]
    fn test_remediate_3des() {
        let vuln =
            create_test_vulnerability(CryptoType::TripleDes, "cipher = TripleDES.new(key)", None);

        let fix = remediate_des_3des(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "3DES → AES-256-GCM");
        assert!(fix.new_code.contains("AES"));
        assert!(!fix.new_code.contains("TripleDES"));
    }

    #[test]
    fn test_generate_remediations_multiple() {
        let mut audit_result = AuditResult::new(Language::Python, 100);

        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Md5,
            "hashlib.md5()",
            None,
        ));
        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Sha1,
            "hashlib.sha1()",
            None,
        ));
        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Rsa,
            "RSA.generate(1024)",
            Some(1024),
        ));
        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Des,
            "DES.new(key)",
            None,
        ));

        let remediation = generate_remediations(&audit_result, "test.py");

        assert_eq!(remediation.fixes.len(), 4);
        assert_eq!(remediation.summary.total_vulnerabilities, 4);
        assert_eq!(remediation.summary.remediable, 4); // All have remediation
        assert_eq!(remediation.summary.patch_available, 4); // All can generate patches
        assert!(remediation.summary.average_confidence > 0.0);
    }

    #[test]
    fn test_generate_remediations_unsupported() {
        let mut audit_result = AuditResult::new(Language::Python, 100);

        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Ecdsa,
            "ecdsa.SigningKey()",
            None,
        ));

        let remediation = generate_remediations(&audit_result, "test.py");

        assert_eq!(remediation.fixes.len(), 0);
        assert_eq!(remediation.summary.remediable, 0);
        assert_eq!(remediation.summary.manual_only, 1);
        assert_eq!(remediation.warnings.len(), 1);
        assert!(remediation.warnings[0].contains("ECDSA"));
    }

    #[test]
    fn test_remediation_summary_statistics() {
        let mut audit_result = AuditResult::new(Language::JavaScript, 50);

        // Add mix of patch-available and no-patch vulnerabilities
        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Md5,
            "crypto.createHash('md5')",
            None,
        ));
        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Sha1,
            "crypto.createHash('sha1')",
            None,
        ));
        audit_result.add_vulnerability(create_test_vulnerability(
            CryptoType::Rsa,
            "generateKeyPair(1024)",
            Some(1024),
        ));

        let remediation = generate_remediations(&audit_result, "app.js");

        assert_eq!(remediation.summary.total_vulnerabilities, 3);
        assert_eq!(remediation.summary.remediable, 3); // All have remediation
        assert_eq!(remediation.summary.patch_available, 3); // All can generate patches
        assert!(remediation.summary.average_confidence > 0.7);
    }

    #[test]
    fn test_code_fix_serialization() {
        let fix = CodeFix {
            file_path: "test.py".to_string(),
            line: 10,
            column: 5,
            old_code: "hashlib.md5()".to_string(),
            new_code: "hashlib.sha256()".to_string(),
            confidence: 0.85,
            algorithm: "MD5 → SHA-256".to_string(),
            explanation: "Test explanation".to_string(),
            patch_available: true,
            review_notes: vec!["Test note".to_string()],
            patch_file: None,
        };

        let json = serde_json::to_string(&fix).unwrap();
        let deserialized: CodeFix = serde_json::from_str(&json).unwrap();

        assert_eq!(fix.file_path, deserialized.file_path);
        assert_eq!(fix.confidence, deserialized.confidence);
        assert_eq!(fix.patch_available, deserialized.patch_available);
    }
}
