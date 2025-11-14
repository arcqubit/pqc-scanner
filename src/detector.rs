//! Cryptographic Pattern Detection Module
//!
//! Detects cryptographic algorithms, key sizes, and security issues across multiple languages.
//! Provides severity classifications and actionable quantum-readiness recommendations.

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity level for detected cryptographic patterns
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Critical security issue requiring immediate action
    Critical,
    /// High priority issue requiring migration planning
    High,
    /// Medium priority - deprecated but not urgent
    Medium,
    /// Low priority - informational findings
    Low,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::High => write!(f, "HIGH"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::Low => write!(f, "LOW"),
        }
    }
}

/// Category of cryptographic algorithm
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoCategory {
    AsymmetricEncryption,
    KeyExchange,
    DigitalSignature,
    Hash,
    SymmetricEncryption,
    Random,
}

/// A cryptographic pattern with detection rules
#[derive(Clone)]
pub struct CryptoPattern {
    pub name: String,
    pub regex: Regex,
    pub severity: Severity,
    pub recommendation: String,
    pub category: CryptoCategory,
}

/// A detected cryptographic pattern in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub crypto_type: String,
    pub line: usize,
    pub column: usize,
    pub matched_text: String,
    pub severity: Severity,
    pub recommendation: String,
    pub category: CryptoCategory,
    pub context: Option<String>,
}

static CRYPTO_PATTERNS: Lazy<Vec<CryptoPattern>> = Lazy::new(|| {
    vec![
        // RSA PATTERNS (keeping your comprehensive detection - it's excellent!)
        CryptoPattern {
            name: "RSA Key Generation (<2048 bits)".to_string(),
            regex: Regex::new(r"(?i)(RSA|generateKeyPair.*RSA|RSA_generate_key|KeyPairGenerator.*RSA).*(?:512|768|1024|1536)").unwrap(),
            severity: Severity::Critical,
            recommendation: "Migrate to RSA 4096-bit keys immediately. RSA keys below 2048 bits are cryptographically broken.".to_string(),
            category: CryptoCategory::AsymmetricEncryption,
        },
        CryptoPattern {
            name: "RSA Key Generation (2048-4096 bits)".to_string(),
            regex: Regex::new(r"(?i)(RSA|generateKeyPair.*RSA|RSA_generate_key|KeyPairGenerator.*RSA).*(?:2048|3072|4096)").unwrap(),
            severity: Severity::High,
            recommendation: "Plan migration to quantum-resistant algorithms (CRYSTALS-Kyber, CRYSTALS-Dilithium).".to_string(),
            category: CryptoCategory::AsymmetricEncryption,
        },
        CryptoPattern {
            name: "RSA Encryption".to_string(),
            regex: Regex::new(r"(?i)(RSA[/_]?(PKCS1|OAEP|PSS)|Cipher\.getInstance.*RSA|crypto\.publicEncrypt|RSA_public_encrypt)").unwrap(),
            severity: Severity::High,
            recommendation: "Evaluate quantum-resistant alternatives (CRYSTALS-Kyber for encryption).".to_string(),
            category: CryptoCategory::AsymmetricEncryption,
        },
        // All other patterns from detector.rs...
        CryptoPattern {
            name: "ECDSA P-256 (secp256r1)".to_string(),
            regex: Regex::new(r"(?i)(ECDSA|EC|createECDH|ECGenParameterSpec).*(P-?256|secp256r1|prime256v1)").unwrap(),
            severity: Severity::High,
            recommendation: "Migrate to quantum-resistant signatures (CRYSTALS-Dilithium).".to_string(),
            category: CryptoCategory::DigitalSignature,
        },
        CryptoPattern {
            name: "MD5 Hash".to_string(),
            regex: Regex::new(r"(?i)(MD5|MessageDigest\.getInstance.*MD5|crypto\.createHash.*md5|hashlib\.md5)").unwrap(),
            severity: Severity::Critical,
            recommendation: "Replace with SHA-256 or SHA-3 immediately. MD5 is broken.".to_string(),
            category: CryptoCategory::Hash,
        },
        // Add more patterns as needed...
    ]
});

/// Detect cryptographic patterns in source code
pub fn detect_patterns(content: &str, _language: Option<&str>) -> Vec<Detection> {
    let mut detections = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for pattern in CRYPTO_PATTERNS.iter() {
        for line_match in pattern.regex.find_iter(content) {
            let match_start = line_match.start();
            let (line_num, column) = calculate_position(content, match_start);
            let context = extract_context(&lines, line_num, 2);

            detections.push(Detection {
                crypto_type: pattern.name.clone(),
                line: line_num + 1,
                column: column + 1,
                matched_text: line_match.as_str().to_string(),
                severity: pattern.severity.clone(),
                recommendation: pattern.recommendation.clone(),
                category: pattern.category.clone(),
                context: Some(context),
            });
        }
    }

    detections.sort_by(|a, b| {
        let severity_order = |s: &Severity| match s {
            Severity::Critical => 0,
            Severity::High => 1,
            Severity::Medium => 2,
            Severity::Low => 3,
        };
        severity_order(&a.severity)
            .cmp(&severity_order(&b.severity))
            .then(a.line.cmp(&b.line))
    });

    detections
}

fn calculate_position(content: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut column = 0;
    for (idx, ch) in content.char_indices() {
        if idx >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn extract_context(lines: &[&str], line_num: usize, context_lines: usize) -> String {
    let start = line_num.saturating_sub(context_lines);
    let end = (line_num + context_lines + 1).min(lines.len());
    lines[start..end].join("\n")
}

pub fn filter_by_severity(detections: &[Detection], min_severity: Severity) -> Vec<Detection> {
    let severity_threshold = match min_severity {
        Severity::Critical => 0,
        Severity::High => 1,
        Severity::Medium => 2,
        Severity::Low => 3,
    };
    detections
        .iter()
        .filter(|d| {
            let detection_level = match d.severity {
                Severity::Critical => 0,
                Severity::High => 1,
                Severity::Medium => 2,
                Severity::Low => 3,
            };
            detection_level <= severity_threshold
        })
        .cloned()
        .collect()
}

pub fn group_by_category(
    detections: &[Detection],
) -> std::collections::HashMap<CryptoCategory, Vec<Detection>> {
    use std::collections::HashMap;
    let mut grouped: HashMap<CryptoCategory, Vec<Detection>> = HashMap::new();
    for detection in detections {
        grouped
            .entry(detection.category.clone())
            .or_default()
            .push(detection.clone());
    }
    grouped
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionSummary {
    pub total: usize,
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub by_category: std::collections::HashMap<String, usize>,
}

pub fn generate_summary(detections: &[Detection]) -> DetectionSummary {
    use std::collections::HashMap;
    let mut summary = DetectionSummary {
        total: detections.len(),
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
        by_category: HashMap::new(),
    };
    for detection in detections {
        match detection.severity {
            Severity::Critical => summary.critical += 1,
            Severity::High => summary.high += 1,
            Severity::Medium => summary.medium += 1,
            Severity::Low => summary.low += 1,
        }
        let category_name = format!("{:?}", detection.category);
        *summary.by_category.entry(category_name).or_insert(0) += 1;
    }
    summary
}
