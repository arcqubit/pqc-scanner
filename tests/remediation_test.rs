// Integration tests for the remediation module

use pqc_scanner::{analyze, generate_remediations};

#[test]
fn test_md5_remediation_python() {
    let source = r#"
import hashlib

def hash_password(password):
    return hashlib.md5(password.encode()).hexdigest()
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "password.py");

    assert!(!remediation.fixes.is_empty());
    assert_eq!(remediation.summary.auto_fixable, 1);

    let fix = &remediation.fixes[0];
    assert_eq!(fix.algorithm, "MD5 → SHA-256");
    assert!(fix.new_code.contains("sha256"));
    assert!(fix.auto_applicable);
    assert!(fix.confidence >= 0.8);
}

#[test]
fn test_sha1_remediation_nodejs() {
    let source = r#"
const crypto = require('crypto');

function hashData(data) {
    return crypto.createHash('sha1').update(data).digest('hex');
}
"#;

    let audit_result = analyze(source, "javascript").unwrap();
    let remediation = generate_remediations(&audit_result, "utils.js");

    assert!(!remediation.fixes.is_empty());

    let fix = &remediation.fixes[0];
    assert_eq!(fix.algorithm, "SHA-1 → SHA-256");
    assert!(fix.new_code.contains("sha256"));
    assert!(fix.auto_applicable);
}

#[test]
fn test_rsa_weak_key_remediation() {
    let source = r#"
from Crypto.PublicKey import RSA

key = RSA.generate(1024)
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "crypto.py");

    assert!(!remediation.fixes.is_empty());

    let fix = &remediation.fixes[0];
    // The algorithm field should indicate RSA remediation
    assert!(fix.algorithm.contains("RSA"));
    assert!(!fix.auto_applicable); // Requires manual review
    assert!(fix.explanation.contains("CRYSTALS") || fix.explanation.contains("quantum"));
}

#[test]
fn test_des_remediation() {
    let source = r#"
from Crypto.Cipher import DES

cipher = DES.new(key, DES.MODE_ECB)
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "encryption.py");

    assert!(!remediation.fixes.is_empty());

    let fix = &remediation.fixes[0];
    assert_eq!(fix.algorithm, "DES → AES-256");
    assert!(fix.new_code.contains("AES"));
    assert!(!fix.new_code.contains("DES"));
    assert!(!fix.auto_applicable);
}

#[test]
fn test_3des_remediation() {
    let source = r#"
const crypto = require('crypto');

const cipher = crypto.createCipheriv('des-ede3', key, iv);
"#;

    let audit_result = analyze(source, "javascript").unwrap();
    let remediation = generate_remediations(&audit_result, "cipher.js");

    assert!(!remediation.fixes.is_empty());

    let fix = &remediation.fixes[0];
    assert!(fix.algorithm.contains("3DES") || fix.algorithm.contains("AES"));
    assert!(!fix.auto_applicable);
}

#[test]
fn test_multiple_vulnerabilities_remediation() {
    let source = r#"
import hashlib
from Crypto.PublicKey import RSA

def process_data(data):
    hash1 = hashlib.md5(data).hexdigest()
    hash2 = hashlib.sha1(data).hexdigest()
    key = RSA.generate(1024)
    return hash1, hash2, key
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "multi.py");

    // Should have fixes for all detected vulnerabilities
    assert!(!remediation.fixes.is_empty());
    assert_eq!(
        remediation.summary.total_vulnerabilities,
        audit_result.vulnerabilities.len()
    );

    // At least MD5 and SHA1 should be auto-fixable
    assert!(remediation.summary.auto_fixable >= 2);

    // Should have some manual review items (RSA)
    assert!(remediation.summary.manual_review_required >= 1);
    assert!(remediation.summary.average_confidence > 0.0);
}

#[test]
fn test_unsupported_algorithm_warning() {
    let source = r#"
from ecdsa import SigningKey, NIST192p

sk = SigningKey.generate(curve=NIST192p)
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "ecdsa.py");

    // ECDSA remediation not yet implemented
    assert!(!remediation.warnings.is_empty());
    assert!(remediation.warnings[0].contains("ECDSA"));
}

#[test]
fn test_remediation_confidence_scores() {
    let source = r#"
import hashlib

md5_hash = hashlib.md5()
sha1_hash = hashlib.sha1()
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "hashes.py");

    assert_eq!(remediation.fixes.len(), 2);

    // SHA1 should have higher confidence than MD5
    let sha1_fix = remediation
        .fixes
        .iter()
        .find(|f| f.algorithm.contains("SHA-1"))
        .unwrap();

    assert!(sha1_fix.confidence >= 0.9);
}

#[test]
fn test_remediation_empty_audit() {
    let source = r#"
def secure_function():
    return "No crypto here"
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "safe.py");

    assert!(remediation.fixes.is_empty());
    assert_eq!(remediation.summary.total_vulnerabilities, 0);
    assert_eq!(remediation.summary.auto_fixable, 0);
    assert_eq!(remediation.summary.average_confidence, 0.0);
}

#[test]
fn test_remediation_serialization() {
    let source = r#"
import hashlib
hash = hashlib.md5()
"#;

    let audit_result = analyze(source, "python").unwrap();
    let remediation = generate_remediations(&audit_result, "test.py");

    // Test JSON serialization
    let json = serde_json::to_string(&remediation).unwrap();
    let deserialized: pqc_scanner::RemediationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(remediation.fixes.len(), deserialized.fixes.len());
    assert_eq!(
        remediation.summary.total_vulnerabilities,
        deserialized.summary.total_vulnerabilities
    );
}
