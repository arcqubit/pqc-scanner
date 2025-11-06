use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported programming languages for audit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Java,
    Go,
    Cpp,
    Csharp,
}

impl Language {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Some(Language::Rust),
            "javascript" | "js" => Some(Language::JavaScript),
            "typescript" | "ts" => Some(Language::TypeScript),
            "python" | "py" => Some(Language::Python),
            "java" => Some(Language::Java),
            "go" | "golang" => Some(Language::Go),
            "cpp" | "c++" | "cxx" => Some(Language::Cpp),
            "csharp" | "cs" | "c#" => Some(Language::Csharp),
            _ => None,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::JavaScript => write!(f, "javascript"),
            Language::TypeScript => write!(f, "typescript"),
            Language::Python => write!(f, "python"),
            Language::Java => write!(f, "java"),
            Language::Go => write!(f, "go"),
            Language::Cpp => write!(f, "cpp"),
            Language::Csharp => write!(f, "csharp"),
        }
    }
}

/// Severity levels for vulnerabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of cryptographic algorithms detected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CryptoType {
    Rsa,
    Ecdsa,
    Ecdh,
    Dsa,
    DiffieHellman,
    Sha1,
    Md5,
    Des,
    TripleDes,
    Rc4,
}

impl fmt::Display for CryptoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoType::Rsa => write!(f, "RSA"),
            CryptoType::Ecdsa => write!(f, "ECDSA"),
            CryptoType::Ecdh => write!(f, "ECDH"),
            CryptoType::Dsa => write!(f, "DSA"),
            CryptoType::DiffieHellman => write!(f, "Diffie-Hellman"),
            CryptoType::Sha1 => write!(f, "SHA-1"),
            CryptoType::Md5 => write!(f, "MD5"),
            CryptoType::Des => write!(f, "DES"),
            CryptoType::TripleDes => write!(f, "3DES"),
            CryptoType::Rc4 => write!(f, "RC4"),
        }
    }
}

/// Individual vulnerability finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// Type of crypto algorithm found
    pub crypto_type: CryptoType,

    /// Severity level
    pub severity: Severity,

    /// Risk score (0-100)
    pub risk_score: u32,

    /// Line number in source code
    pub line: usize,

    /// Column number in source code
    pub column: usize,

    /// Context snippet from source
    pub context: String,

    /// Detailed description
    pub message: String,

    /// Recommendation for remediation
    pub recommendation: String,

    /// Key size detected (if applicable)
    pub key_size: Option<u32>,
}

/// Complete audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    /// List of vulnerabilities found
    pub vulnerabilities: Vec<Vulnerability>,

    /// Overall risk score (0-100)
    pub risk_score: u32,

    /// Language audited
    pub language: Language,

    /// Summary recommendations
    pub recommendations: Vec<String>,

    /// Statistics
    pub stats: AuditStats,
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStats {
    pub total_vulnerabilities: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub lines_scanned: usize,
}

impl AuditResult {
    pub fn new(language: Language, lines_scanned: usize) -> Self {
        Self {
            vulnerabilities: Vec::new(),
            risk_score: 0,
            language,
            recommendations: Vec::new(),
            stats: AuditStats {
                total_vulnerabilities: 0,
                critical_count: 0,
                high_count: 0,
                medium_count: 0,
                low_count: 0,
                lines_scanned,
            },
        }
    }

    pub fn add_vulnerability(&mut self, vuln: Vulnerability) {
        match vuln.severity {
            Severity::Critical => self.stats.critical_count += 1,
            Severity::High => self.stats.high_count += 1,
            Severity::Medium => self.stats.medium_count += 1,
            Severity::Low => self.stats.low_count += 1,
        }
        self.stats.total_vulnerabilities += 1;
        self.vulnerabilities.push(vuln);
    }

    pub fn calculate_risk_score(&mut self) {
        if self.vulnerabilities.is_empty() {
            self.risk_score = 0;
            return;
        }

        // Weighted average of all vulnerability risk scores
        let total_score: u32 = self.vulnerabilities.iter().map(|v| v.risk_score).sum();
        self.risk_score = total_score / self.vulnerabilities.len() as u32;
    }

    pub fn generate_recommendations(&mut self) {
        if self.stats.critical_count > 0 {
            self.recommendations.push(
                "CRITICAL: Immediately migrate to quantum-safe algorithms (CRYSTALS-Kyber, CRYSTALS-Dilithium)".to_string()
            );
        }

        if self.stats.high_count > 0 {
            self.recommendations.push(
                "HIGH PRIORITY: Plan migration to post-quantum cryptography within 6-12 months".to_string()
            );
        }

        // Check for specific crypto types
        let has_rsa = self.vulnerabilities.iter().any(|v| v.crypto_type == CryptoType::Rsa);
        let has_ecdsa = self.vulnerabilities.iter().any(|v| v.crypto_type == CryptoType::Ecdsa);
        let has_dh = self.vulnerabilities.iter().any(|v| v.crypto_type == CryptoType::DiffieHellman);

        if has_rsa {
            self.recommendations.push(
                "Replace RSA with CRYSTALS-Dilithium for digital signatures or CRYSTALS-Kyber for encryption".to_string()
            );
        }

        if has_ecdsa {
            self.recommendations.push(
                "Replace ECDSA/ECDH with CRYSTALS-Dilithium (signatures) or CRYSTALS-Kyber (key exchange)".to_string()
            );
        }

        if has_dh {
            self.recommendations.push(
                "Replace Diffie-Hellman key exchange with CRYSTALS-Kyber or NTRU".to_string()
            );
        }

        self.recommendations.push(
            "Follow NIST Post-Quantum Cryptography Standardization guidelines: https://csrc.nist.gov/projects/post-quantum-cryptography".to_string()
        );
    }
}

// Parser types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Import,
    FunctionDeclaration,
    ClassDeclaration,
    VariableDeclaration,
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: NodeType,
    pub line: usize,
    pub column: usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedSource {
    pub language: Language,
    pub ast_nodes: Vec<AstNode>,
    pub imports: Vec<String>,
    pub function_calls: Vec<FunctionCall>,
}

impl ParsedSource {
    pub fn new(language: Language) -> Self {
        Self {
            language,
            ast_nodes: Vec::new(),
            imports: Vec::new(),
            function_calls: Vec::new(),
        }
    }
}
