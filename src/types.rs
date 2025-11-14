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
                "HIGH PRIORITY: Plan migration to post-quantum cryptography within 6-12 months"
                    .to_string(),
            );
        }

        // Check for specific crypto types
        let has_rsa = self
            .vulnerabilities
            .iter()
            .any(|v| v.crypto_type == CryptoType::Rsa);
        let has_ecdsa = self
            .vulnerabilities
            .iter()
            .any(|v| v.crypto_type == CryptoType::Ecdsa);
        let has_dh = self
            .vulnerabilities
            .iter()
            .any(|v| v.crypto_type == CryptoType::DiffieHellman);

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
                "Replace Diffie-Hellman key exchange with CRYSTALS-Kyber or NTRU".to_string(),
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

// NIST 800-53 SC-13 Control Assessment Types

/// NIST 800-53 SC-13 Control Implementation Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImplementationStatus {
    Implemented,
    PartiallyImplemented,
    PlannedForImplementation,
    AlternativeImplementation,
    NotApplicable,
}

/// Control Assessment Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssessmentStatus {
    Satisfied,
    NotSatisfied,
    Other,
}

/// NIST 800-53 SC-13 Control Finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlFinding {
    /// Finding ID
    pub finding_id: String,

    /// Control ID (e.g., "sc-13")
    pub control_id: String,

    /// Implementation status
    pub implementation_status: ImplementationStatus,

    /// Assessment status
    pub assessment_status: AssessmentStatus,

    /// Description of the finding
    pub description: String,

    /// Related vulnerabilities
    pub related_vulnerabilities: Vec<String>,

    /// Evidence collected
    pub evidence: Vec<Evidence>,

    /// Remediation recommendations
    pub remediation: String,

    /// Risk level
    pub risk_level: Severity,
}

/// Evidence for control assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Evidence ID
    pub evidence_id: String,

    /// Type of evidence
    pub evidence_type: EvidenceType,

    /// Description
    pub description: String,

    /// Source file and line
    pub source_location: Option<SourceLocation>,

    /// Timestamp
    pub collected_at: String,

    /// Related data
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceType {
    CodeAnalysis,
    StaticScan,
    ConfigurationReview,
    DocumentationReview,
    AutomatedTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
}

/// NIST 800-53 SC-13 Assessment Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SC13AssessmentReport {
    /// Report metadata
    pub metadata: ReportMetadata,

    /// Control assessment
    pub control_assessment: ControlAssessment,

    /// Summary statistics
    pub summary: AssessmentSummary,

    /// Detailed findings
    pub findings: Vec<ControlFinding>,

    /// Overall recommendations
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Report ID
    pub report_id: String,

    /// Report title
    pub title: String,

    /// Assessment date/time
    pub published: String,

    /// Last modified date/time
    pub last_modified: String,

    /// Version
    pub version: String,

    /// OSCAL version
    pub oscal_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlAssessment {
    /// Control ID
    pub control_id: String,

    /// Control name
    pub control_name: String,

    /// Control family
    pub control_family: String,

    /// Control description
    pub control_description: String,

    /// Implementation status
    pub implementation_status: ImplementationStatus,

    /// Assessment status
    pub assessment_status: AssessmentStatus,

    /// Assessment method
    pub assessment_method: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentSummary {
    /// Total files scanned
    pub files_scanned: usize,

    /// Total lines scanned
    pub lines_scanned: usize,

    /// Total vulnerabilities found
    pub total_vulnerabilities: usize,

    /// Quantum-vulnerable algorithms detected
    pub quantum_vulnerable_algorithms: Vec<String>,

    /// Deprecated algorithms detected
    pub deprecated_algorithms: Vec<String>,

    /// Weak key sizes detected
    pub weak_key_sizes: Vec<String>,

    /// Compliance score (0-100)
    pub compliance_score: u32,

    /// Risk score (0-100)
    pub risk_score: u32,
}

// OSCAL Assessment Results Schema Types

/// OSCAL Assessment Results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OscalAssessmentResults {
    /// OSCAL version
    pub oscal_version: String,

    /// Assessment results
    pub assessment_results: AssessmentResults,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentResults {
    /// UUID
    pub uuid: String,

    /// Metadata
    pub metadata: OscalMetadata,

    /// Import SSP (System Security Plan)
    pub import_ssp: ImportSSP,

    /// Results
    pub results: Vec<AssessmentResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscalMetadata {
    /// Title
    pub title: String,

    /// Published timestamp
    pub published: String,

    /// Last modified timestamp
    #[serde(rename = "last-modified")]
    pub last_modified: String,

    /// Version
    pub version: String,

    /// OSCAL version
    #[serde(rename = "oscal-version")]
    pub oscal_version: String,

    /// Roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,

    /// Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<Party>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub uuid: String,
    #[serde(rename = "type")]
    pub party_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSSP {
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentResult {
    /// UUID
    pub uuid: String,

    /// Title
    pub title: String,

    /// Description
    pub description: String,

    /// Start timestamp
    pub start: String,

    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Reviewed controls
    pub reviewed_controls: ReviewedControls,

    /// Observations
    pub observations: Vec<Observation>,

    /// Findings
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewedControls {
    pub control_selections: Vec<ControlSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlSelection {
    pub include_controls: Vec<ControlRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ControlRef {
    pub control_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub uuid: String,
    pub description: String,
    pub methods: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relevant_evidence: Option<Vec<RelevantEvidence>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevantEvidence {
    pub href: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Finding {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub target: Target,
    pub implementation_status: OscalImplementationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_observations: Option<Vec<RelatedObservation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "type")]
    pub target_type: String,
    #[serde(rename = "target-id")]
    pub target_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TargetStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetStatus {
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscalImplementationStatus {
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedObservation {
    pub observation_uuid: String,
}
