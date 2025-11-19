use crate::types::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuditError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Invalid source code")]
    InvalidSource,

    #[error("Source code too large: {0} bytes (max: {1})")]
    SourceTooLarge(usize, usize),

    #[error("Too many lines: {0} (max: {1})")]
    TooManyLines(usize, usize),

    #[error("Parse error: {0}")]
    ParseError(String),
}

// Input validation constants
const MAX_SOURCE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const MAX_LINES: usize = 500_000;

// Lazy-compiled regex patterns for crypto detection
lazy_static! {
    // RSA patterns with key size detection
    static ref RSA_PATTERN: Regex = Regex::new(
        r"(?i)(RSA|Rsa|rsa)[^a-zA-Z]*([\d]{3,4})?|(generate.*rsa.*key|rsa.*keygen)"
    ).unwrap();

    static ref RSA_KEYGEN: Regex = Regex::new(
        r"(?i)(RSA\.generate|generateKeyPair.*RSA|KeyPairGenerator\.getInstance.*RSA|rsa\.GenerateKey)"
    ).unwrap();

    static ref RSA_KEY_SIZE: Regex = Regex::new(
        r"(?i)(?:rsa|RSA)[^0-9]*(512|1024|2048|3072|4096|8192)"
    ).unwrap();

    // ECDSA/ECDH patterns
    static ref ECDSA_PATTERN: Regex = Regex::new(
        r"(?i)(ECDSA|ECC|elliptic.*curve|secp256k1|secp384r1|prime256v1|P-256|P-384|P-521)"
    ).unwrap();

    static ref ECDH_PATTERN: Regex = Regex::new(
        r"(?i)(ECDH|ecdh|elliptic.*diffie|curve25519)"
    ).unwrap();

    // DSA and Diffie-Hellman
    static ref DSA_PATTERN: Regex = Regex::new(
        r"(?i)(DSA|dsa)[^a-zA-Z]|(Digital.*Signature.*Algorithm)"
    ).unwrap();

    static ref DH_PATTERN: Regex = Regex::new(
        r"(?i)(diffie.*hellman|DH_|DHE|DHE_)"
    ).unwrap();

    // Deprecated hash functions
    static ref SHA1_PATTERN: Regex = Regex::new(
        r"(?i)(SHA1|sha1|SHA-1|sha-1)[^0-9]"
    ).unwrap();

    static ref MD5_PATTERN: Regex = Regex::new(
        r"(?i)(MD5|md5)"
    ).unwrap();

    // Deprecated ciphers - simpler pattern without lookahead
    static ref DES_PATTERN: Regex = Regex::new(
        r"(?i)(DES_|_DES|\.DES|DES\.|\bDES\b)"
    ).unwrap();

    static ref TRIPLE_DES_PATTERN: Regex = Regex::new(
        r"(?i)(3DES|TripleDES|DESede)"
    ).unwrap();

    static ref RC4_PATTERN: Regex = Regex::new(
        r"(?i)(RC4|rc4|ARCFOUR)"
    ).unwrap();
}

/// Main audit function - analyzes source code for quantum-vulnerable cryptography
pub fn analyze(source: &str, language: &str) -> Result<AuditResult, AuditError> {
    // Parse language
    let lang = parse_language(language)?;

    // Validate source is not empty
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Err(AuditError::InvalidSource);
    }

    // Validate source size
    let source_size = source.len();
    if source_size > MAX_SOURCE_SIZE {
        return Err(AuditError::SourceTooLarge(source_size, MAX_SOURCE_SIZE));
    }

    let lines: Vec<&str> = source.lines().collect();
    let line_count = lines.len();

    // Validate line count
    if line_count > MAX_LINES {
        return Err(AuditError::TooManyLines(line_count, MAX_LINES));
    }

    let mut result = AuditResult::new(lang, line_count);

    // Scan each line for crypto patterns
    for (line_idx, line) in lines.iter().enumerate() {
        let line_num = line_idx + 1;

        // Detect RSA
        if let Some(vuln) = detect_rsa(line, line_num) {
            result.add_vulnerability(vuln);
        }

        // Detect ECDSA
        if let Some(vuln) = detect_ecdsa(line, line_num) {
            result.add_vulnerability(vuln);
        }

        // Detect ECDH
        if let Some(vuln) = detect_ecdh(line, line_num) {
            result.add_vulnerability(vuln);
        }

        // Detect DSA
        if let Some(vuln) = detect_dsa(line, line_num) {
            result.add_vulnerability(vuln);
        }

        // Detect Diffie-Hellman
        if let Some(vuln) = detect_diffie_hellman(line, line_num) {
            result.add_vulnerability(vuln);
        }

        // Detect deprecated hash functions
        if let Some(vuln) = detect_sha1(line, line_num) {
            result.add_vulnerability(vuln);
        }

        if let Some(vuln) = detect_md5(line, line_num) {
            result.add_vulnerability(vuln);
        }

        // Detect deprecated ciphers
        if let Some(vuln) = detect_des(line, line_num) {
            result.add_vulnerability(vuln);
        }

        if let Some(vuln) = detect_triple_des(line, line_num) {
            result.add_vulnerability(vuln);
        }

        if let Some(vuln) = detect_rc4(line, line_num) {
            result.add_vulnerability(vuln);
        }
    }

    // Calculate overall risk score
    result.calculate_risk_score();

    // Generate recommendations
    result.generate_recommendations();

    Ok(result)
}

/// Parse language string to enum
fn parse_language(lang: &str) -> Result<Language, AuditError> {
    Language::from_string(lang).ok_or_else(|| AuditError::UnsupportedLanguage(lang.to_string()))
}

/// Detect RSA usage and determine risk
fn detect_rsa(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !RSA_PATTERN.is_match(line) && !RSA_KEYGEN.is_match(line) {
        return None;
    }

    // Try to extract key size
    let key_size = RSA_KEY_SIZE
        .captures(line)
        .and_then(|cap| cap.get(1))
        .and_then(|m| u32::from_str(m.as_str()).ok());

    let (severity, risk_score, message) = match key_size {
        Some(size) if size < 2048 => (
            Severity::Critical,
            100,
            format!(
                "RSA with {}-bit key is critically vulnerable to quantum attacks",
                size
            ),
        ),
        Some(size) if size < 4096 => (
            Severity::High,
            85,
            format!(
                "RSA with {}-bit key will be vulnerable to quantum computers",
                size
            ),
        ),
        Some(size) => (
            Severity::High,
            80,
            format!(
                "RSA with {}-bit key is quantum-vulnerable (Shor's algorithm)",
                size
            ),
        ),
        None => (
            Severity::High,
            85,
            "RSA detected - vulnerable to quantum attacks via Shor's algorithm".to_string(),
        ),
    };

    let column = line
        .find(|c: char| c.to_lowercase().any(|c| c == 'r'))
        .unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Rsa,
        severity,
        risk_score,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message,
        recommendation:
            "Replace with CRYSTALS-Dilithium (signatures) or CRYSTALS-Kyber (encryption)"
                .to_string(),
        key_size,
    })
}

/// Detect ECDSA usage
fn detect_ecdsa(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !ECDSA_PATTERN.is_match(line) {
        return None;
    }

    let column = line
        .find(|c: char| c.to_uppercase().any(|c| "ECDSA".contains(c)))
        .unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Ecdsa,
        severity: Severity::High,
        risk_score: 85,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "ECDSA (Elliptic Curve Digital Signature Algorithm) is quantum-vulnerable"
            .to_string(),
        recommendation: "Replace with CRYSTALS-Dilithium or SPHINCS+ for post-quantum signatures"
            .to_string(),
        key_size: None,
    })
}

/// Detect ECDH usage
fn detect_ecdh(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !ECDH_PATTERN.is_match(line) {
        return None;
    }

    let column = line
        .find(|c: char| c.to_uppercase().any(|c| "ECDH".contains(c)))
        .unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Ecdh,
        severity: Severity::High,
        risk_score: 85,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "ECDH (Elliptic Curve Diffie-Hellman) is quantum-vulnerable".to_string(),
        recommendation: "Replace with CRYSTALS-Kyber or NTRU for quantum-safe key exchange"
            .to_string(),
        key_size: None,
    })
}

/// Detect DSA usage
fn detect_dsa(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !DSA_PATTERN.is_match(line) {
        return None;
    }

    let column = line.find("DSA").or_else(|| line.find("dsa")).unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Dsa,
        severity: Severity::High,
        risk_score: 90,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "DSA (Digital Signature Algorithm) is quantum-vulnerable".to_string(),
        recommendation: "Replace with CRYSTALS-Dilithium for post-quantum digital signatures"
            .to_string(),
        key_size: None,
    })
}

/// Detect Diffie-Hellman usage
fn detect_diffie_hellman(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !DH_PATTERN.is_match(line) {
        return None;
    }

    let column = line.find("diffie").or_else(|| line.find("DH")).unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::DiffieHellman,
        severity: Severity::High,
        risk_score: 85,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "Diffie-Hellman key exchange is quantum-vulnerable".to_string(),
        recommendation:
            "Replace with CRYSTALS-Kyber or FrodoKEM for quantum-safe key encapsulation".to_string(),
        key_size: None,
    })
}

/// Detect SHA-1 usage
fn detect_sha1(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !SHA1_PATTERN.is_match(line) {
        return None;
    }

    let column = line.find("SHA1").or_else(|| line.find("sha1")).unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Sha1,
        severity: Severity::Critical,
        risk_score: 95,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "SHA-1 is cryptographically broken and should not be used".to_string(),
        recommendation: "Replace with SHA-256, SHA-384, or SHA-512".to_string(),
        key_size: None,
    })
}

/// Detect MD5 usage
fn detect_md5(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !MD5_PATTERN.is_match(line) {
        return None;
    }

    let column = line.find("MD5").or_else(|| line.find("md5")).unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Md5,
        severity: Severity::Critical,
        risk_score: 100,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "MD5 is cryptographically broken and must not be used".to_string(),
        recommendation: "Replace with SHA-256 or SHA-3".to_string(),
        key_size: None,
    })
}

/// Detect DES usage
fn detect_des(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !DES_PATTERN.is_match(line) {
        return None;
    }

    let column = line.find("DES").or_else(|| line.find("des")).unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Des,
        severity: Severity::Critical,
        risk_score: 95,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "DES is obsolete and cryptographically weak".to_string(),
        recommendation: "Replace with AES-256 or ChaCha20".to_string(),
        key_size: None,
    })
}

/// Detect 3DES usage
fn detect_triple_des(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !TRIPLE_DES_PATTERN.is_match(line) {
        return None;
    }

    let column = line
        .find("3DES")
        .or_else(|| line.find("TripleDES"))
        .unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::TripleDes,
        severity: Severity::High,
        risk_score: 80,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "3DES (Triple DES) is deprecated and should be replaced".to_string(),
        recommendation: "Replace with AES-256 or ChaCha20-Poly1305".to_string(),
        key_size: None,
    })
}

/// Detect RC4 usage
fn detect_rc4(line: &str, line_num: usize) -> Option<Vulnerability> {
    if !RC4_PATTERN.is_match(line) {
        return None;
    }

    let column = line.find("RC4").or_else(|| line.find("rc4")).unwrap_or(0);

    Some(Vulnerability {
        crypto_type: CryptoType::Rc4,
        severity: Severity::Critical,
        risk_score: 95,
        line: line_num,
        column,
        context: line.trim().to_string(),
        message: "RC4 is cryptographically broken and must not be used".to_string(),
        recommendation: "Replace with AES-GCM or ChaCha20-Poly1305".to_string(),
        key_size: None,
    })
}

/// Calculate risk score for a crypto type and key size
pub fn score_vulnerability(crypto_type: &CryptoType, key_size: Option<u32>) -> u32 {
    match crypto_type {
        CryptoType::Rsa => match key_size {
            Some(size) if size < 1024 => 100, // Critical
            Some(size) if size < 2048 => 100, // Critical
            Some(size) if size < 4096 => 85,  // High
            _ => 80,                          // High (any RSA is quantum-vulnerable)
        },
        CryptoType::Ecdsa | CryptoType::Ecdh => 85, // High
        CryptoType::Dsa => 90,                      // High
        CryptoType::DiffieHellman => 85,            // High
        CryptoType::Sha1 => 95,                     // Critical (broken)
        CryptoType::Md5 => 100,                     // Critical (broken)
        CryptoType::Des => 95,                      // Critical (weak)
        CryptoType::TripleDes => 80,                // High (deprecated)
        CryptoType::Rc4 => 95,                      // Critical (broken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_language() {
        assert!(parse_language("rust").is_ok());
        assert!(parse_language("javascript").is_ok());
        assert!(parse_language("python").is_ok());
        assert!(parse_language("unknown").is_err());
    }

    #[test]
    fn test_detect_rsa_1024() {
        let line = "RSA.generate(1024)";
        let vuln = detect_rsa(line, 1).unwrap();
        assert_eq!(vuln.crypto_type, CryptoType::Rsa);
        assert_eq!(vuln.severity, Severity::Critical);
        assert_eq!(vuln.key_size, Some(1024));
    }

    #[test]
    fn test_detect_rsa_2048() {
        let line = "generateKeyPair('rsa', { modulusLength: 2048 })";
        let vuln = detect_rsa(line, 1).unwrap();
        assert_eq!(vuln.crypto_type, CryptoType::Rsa);
        assert_eq!(vuln.severity, Severity::High);
        assert_eq!(vuln.key_size, Some(2048));
    }

    #[test]
    fn test_detect_ecdsa() {
        let line = "crypto.createSign('ecdsa-with-SHA256')";
        let vuln = detect_ecdsa(line, 1).unwrap();
        assert_eq!(vuln.crypto_type, CryptoType::Ecdsa);
        assert_eq!(vuln.severity, Severity::High);
    }

    #[test]
    fn test_detect_md5() {
        let line = "hashlib.md5(data).hexdigest()";
        let vuln = detect_md5(line, 1).unwrap();
        assert_eq!(vuln.crypto_type, CryptoType::Md5);
        assert_eq!(vuln.severity, Severity::Critical);
        assert_eq!(vuln.risk_score, 100);
    }

    #[test]
    fn test_analyze_with_multiple_vulns() {
        let source = r#"
            import crypto
            const rsa = crypto.generateKeyPair('rsa', { modulusLength: 1024 })
            const ecdsa = crypto.createSign('ecdsa-with-SHA256')
            const md5Hash = crypto.createHash('md5')
        "#;

        let result = analyze(source, "javascript").unwrap();
        // May detect RSA in import statement as well
        assert!(result.vulnerabilities.len() >= 3);
        assert!(result.stats.critical_count > 0);
        assert!(result.risk_score > 80);
    }

    #[test]
    fn test_score_vulnerability() {
        assert_eq!(score_vulnerability(&CryptoType::Rsa, Some(1024)), 100);
        assert_eq!(score_vulnerability(&CryptoType::Rsa, Some(2048)), 85);
        assert_eq!(score_vulnerability(&CryptoType::Md5, None), 100);
        assert_eq!(score_vulnerability(&CryptoType::Ecdsa, None), 85);
    }

    #[test]
    fn test_empty_source() {
        let result = analyze("", "rust");
        assert!(result.is_err());
    }

    #[test]
    fn test_clean_code() {
        let source = r#"
            // Using quantum-safe crypto
            use pqc_kyber::*;
            let (pk, sk) = keypair();
        "#;

        let result = analyze(source, "rust").unwrap();
        assert_eq!(result.vulnerabilities.len(), 0);
        assert_eq!(result.risk_score, 0);
    }
}
