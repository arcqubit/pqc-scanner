// Integration Tests for Quantum-Safe Crypto Auditor
use pqc_scanner::{CryptoType, Severity, analyze};

#[test]
fn test_end_to_end_rust_audit() {
    let source = r#"
        use rsa::{RsaPrivateKey, RsaPublicKey};
        use ecdsa::SigningKey;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
        let signing_key = SigningKey::random(&mut rng);
    "#;

    let result = analyze(source, "rust").unwrap();

    assert!(result.vulnerabilities.len() > 0);
    assert!(result.risk_score > 0);

    let has_rsa = result
        .vulnerabilities
        .iter()
        .any(|v| v.crypto_type == CryptoType::Rsa);
    let has_ecdsa = result
        .vulnerabilities
        .iter()
        .any(|v| v.crypto_type == CryptoType::Ecdsa);

    assert!(has_rsa || has_ecdsa);
}

#[test]
fn test_javascript_crypto_detection() {
    let source = r#"
        const crypto = require('crypto');
        const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
            modulusLength: 2048,
        });
        const ecdh = crypto.createECDH('secp256k1');
    "#;

    let result = analyze(source, "javascript").unwrap();
    assert!(result.vulnerabilities.len() > 0);
    assert!(result.risk_score > 50);
}

#[test]
fn test_python_crypto_detection() {
    let source = r#"
        import hashlib
        from Crypto.PublicKey import RSA, DSA
        md5_hash = hashlib.md5(data).hexdigest()
        rsa_key = RSA.generate(2048)
    "#;

    let result = analyze(source, "python").unwrap();
    assert!(result.vulnerabilities.len() > 0);

    let has_md5 = result
        .vulnerabilities
        .iter()
        .any(|v| v.crypto_type == CryptoType::Md5);
    let has_rsa = result
        .vulnerabilities
        .iter()
        .any(|v| v.crypto_type == CryptoType::Rsa);

    assert!(has_md5 || has_rsa);
}

#[test]
fn test_clean_quantum_safe_code() {
    let source = r#"
        use pqc_kyber::*;
        use pqc_dilithium::*;
        let (pk, sk) = keypair(&mut rng);
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
    "#;

    let result = analyze(source, "rust").unwrap();
    assert_eq!(result.vulnerabilities.len(), 0);
    assert_eq!(result.risk_score, 0);
}

#[test]
fn test_empty_source_handling() {
    let result = analyze("", "rust");
    assert!(result.is_err());
}

#[test]
fn test_unsupported_language() {
    let result = analyze("some code", "cobol");
    assert!(result.is_err());
}

#[test]
fn test_risk_score_calculation() {
    let source = r#"
        md5_hash = hashlib.md5()
        sha1_hash = hashlib.sha1()
        rsa_key = RSA.generate(1024)
    "#;

    let result = analyze(source, "python").unwrap();
    assert!(result.risk_score >= 90);
}

#[test]
fn test_severity_levels() {
    let source = r#"
        rsa_key = RSA.generate(512)
        md5_hash = hashlib.md5()
    "#;

    let result = analyze(source, "python").unwrap();

    let has_critical = result
        .vulnerabilities
        .iter()
        .any(|v| matches!(v.severity, Severity::Critical));

    assert!(has_critical);

    for vuln in &result.vulnerabilities {
        assert!(!vuln.recommendation.is_empty());
    }
}

#[test]
fn test_language_support() {
    let languages = vec!["rust", "javascript", "typescript", "python", "java", "go"];
    let code = "const x = RSA.generate(2048);";

    for lang in languages {
        let result = analyze(code, lang);
        assert!(result.is_ok(), "Failed for language: {}", lang);
    }
}
