// Auto-remediation module for cryptographic vulnerabilities
// Provides template-based code fixes for quantum-vulnerable algorithms

use crate::types::{AuditResult, CryptoType, Vulnerability};
use serde::{Deserialize, Serialize};

/// A suggested code fix for a cryptographic vulnerability
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

    /// Whether this fix can be auto-applied (true) or needs manual review (false)
    pub auto_applicable: bool,
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

    /// Number of auto-fixable issues
    pub auto_fixable: usize,

    /// Number requiring manual intervention
    pub manual_review_required: usize,

    /// Average confidence score
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

    let auto_fixable = fixes.iter().filter(|f| f.auto_applicable).count();
    let manual_review_required = fixes.len() - auto_fixable;

    let average_confidence = if !fixes.is_empty() {
        fixes.iter().map(|f| f.confidence).sum::<f32>() / fixes.len() as f32
    } else {
        0.0
    };

    RemediationResult {
        fixes,
        summary: RemediationSummary {
            total_vulnerabilities: audit_result.vulnerabilities.len(),
            auto_fixable,
            manual_review_required,
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
        explanation: "Replaced deprecated MD5 hash with SHA-256. Note: For cryptographic security, consider using SHA-3 or BLAKE2.".to_string(),
        auto_applicable: true,
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
        explanation:
            "Replaced deprecated SHA-1 hash with SHA-256. SHA-1 is vulnerable to collision attacks."
                .to_string(),
        auto_applicable: true,
    })
}

/// Generate remediation for RSA key usage
fn remediate_rsa(vuln: &Vulnerability, file_path: &str) -> Option<CodeFix> {
    let old_code = vuln.context.trim().to_string();

    // Check if this is a weak key size (< 2048 bits)
    let is_weak_key = vuln.key_size.is_some_and(|size| size < 2048);

    let (new_code, confidence, explanation, auto_applicable) = if is_weak_key {
        // For weak keys, suggest minimum 2048-bit RSA as interim solution
        let replacement = if let Some(size) = vuln.key_size {
            old_code.replace(&size.to_string(), "2048")
        } else {
            old_code.clone()
        };

        (
            replacement,
            0.7,
            "Upgraded RSA key size to 2048 bits (minimum secure size). CRITICAL: Plan migration to post-quantum algorithms (CRYSTALS-Dilithium for signatures, CRYSTALS-Kyber for encryption) as RSA is vulnerable to quantum attacks.".to_string(),
            false, // Requires manual review due to quantum vulnerability
        )
    } else {
        // For stronger RSA, just provide quantum migration warning
        (
            old_code.clone(),
            0.5,
            "WARNING: RSA is vulnerable to quantum computing attacks. Recommend migrating to CRYSTALS-Dilithium (signatures) or CRYSTALS-Kyber (encryption). No automatic fix available - requires architectural changes.".to_string(),
            false,
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
        auto_applicable,
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

    Some(CodeFix {
        file_path: file_path.to_string(),
        line: vuln.line,
        column: vuln.column,
        old_code,
        new_code,
        confidence: 0.75,
        algorithm: format!("{} → AES-256", algorithm),
        explanation: format!(
            "Replaced deprecated {} cipher with AES-256-GCM. {} has known vulnerabilities and small block size. Ensure proper key management and use authenticated encryption mode (GCM).",
            algorithm, algorithm
        ),
        auto_applicable: false, // Requires manual review for mode and key setup
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
        assert!(fix.auto_applicable);
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
        assert!(fix.auto_applicable);
    }

    #[test]
    fn test_remediate_sha1() {
        let vuln = create_test_vulnerability(CryptoType::Sha1, "hash = hashlib.sha1(data)", None);

        let fix = remediate_sha1(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "SHA-1 → SHA-256");
        assert_eq!(fix.new_code, "hash = hashlib.sha256(data)");
        assert!(fix.confidence >= 0.9);
        assert!(fix.auto_applicable);
    }

    #[test]
    fn test_remediate_rsa_weak_key() {
        let vuln =
            create_test_vulnerability(CryptoType::Rsa, "key = RSA.generate(1024)", Some(1024));

        let fix = remediate_rsa(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "RSA-1024 → RSA-2048 (interim)");
        assert!(fix.new_code.contains("2048"));
        assert!(!fix.auto_applicable); // Requires manual review
        assert!(fix.explanation.contains("CRYSTALS"));
    }

    #[test]
    fn test_remediate_rsa_strong_key() {
        let vuln =
            create_test_vulnerability(CryptoType::Rsa, "key = RSA.generate(4096)", Some(4096));

        let fix = remediate_rsa(&vuln, "test.py").unwrap();

        assert!(fix.algorithm.contains("PQC migration"));
        assert!(!fix.auto_applicable);
        assert!(fix.explanation.contains("quantum"));
        assert!(fix.confidence < 0.7);
    }

    #[test]
    fn test_remediate_des() {
        let vuln =
            create_test_vulnerability(CryptoType::Des, "cipher = DES.new(key, DES.MODE_ECB)", None);

        let fix = remediate_des_3des(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "DES → AES-256");
        assert!(fix.new_code.contains("AES"));
        assert!(!fix.new_code.contains("DES"));
        assert!(!fix.auto_applicable);
        assert!(fix.explanation.contains("GCM"));
    }

    #[test]
    fn test_remediate_3des() {
        let vuln =
            create_test_vulnerability(CryptoType::TripleDes, "cipher = TripleDES.new(key)", None);

        let fix = remediate_des_3des(&vuln, "test.py").unwrap();

        assert_eq!(fix.algorithm, "3DES → AES-256");
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
        assert!(remediation.summary.auto_fixable >= 2); // MD5 and SHA1
        assert!(remediation.summary.manual_review_required >= 2); // RSA and DES
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
        assert_eq!(remediation.warnings.len(), 1);
        assert!(remediation.warnings[0].contains("ECDSA"));
    }

    #[test]
    fn test_remediation_summary_statistics() {
        let mut audit_result = AuditResult::new(Language::JavaScript, 50);

        // Add mix of auto-fixable and manual vulnerabilities
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
        assert_eq!(remediation.summary.auto_fixable, 2); // MD5 and SHA1
        assert_eq!(remediation.summary.manual_review_required, 1); // RSA
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
            auto_applicable: true,
        };

        let json = serde_json::to_string(&fix).unwrap();
        let deserialized: CodeFix = serde_json::from_str(&json).unwrap();

        assert_eq!(fix.file_path, deserialized.file_path);
        assert_eq!(fix.confidence, deserialized.confidence);
        assert_eq!(fix.auto_applicable, deserialized.auto_applicable);
    }
}
