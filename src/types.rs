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

// Canadian CCCS/CSE Cryptographic Compliance Types

/// Canadian Security Classification Levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityClassification {
    Unclassified,
    ProtectedA,
    ProtectedB,
    ProtectedC,
}

impl fmt::Display for SecurityClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityClassification::Unclassified => write!(f, "Unclassified"),
            SecurityClassification::ProtectedA => write!(f, "Protected A"),
            SecurityClassification::ProtectedB => write!(f, "Protected B"),
            SecurityClassification::ProtectedC => write!(f, "Protected C"),
        }
    }
}

/// CCCS Algorithm Approval Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CCCSApprovalStatus {
    Approved,
    ConditionallyApproved,
    Deprecated,
    Prohibited,
    UnderReview,
}

impl fmt::Display for CCCSApprovalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CCCSApprovalStatus::Approved => write!(f, "Approved"),
            CCCSApprovalStatus::ConditionallyApproved => write!(f, "Conditionally Approved"),
            CCCSApprovalStatus::Deprecated => write!(f, "Deprecated"),
            CCCSApprovalStatus::Prohibited => write!(f, "Prohibited"),
            CCCSApprovalStatus::UnderReview => write!(f, "Under Review"),
        }
    }
}

/// Algorithm validation against CCCS standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmValidation {
    pub algorithm: String,
    pub cccs_status: CCCSApprovalStatus,
    pub itsp_reference: String,
    pub approved_key_sizes: Vec<u32>,
    pub approved_modes: Vec<String>,
    pub cmvp_required: bool,
    pub conditions: Vec<String>,
    pub sunset_date: Option<String>,
}

/// Protocol types for ITSP.40.062 compliance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProtocolType {
    Tls,
    Ssh,
    IpSec,
    Https,
    Other(String),
}

impl fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolType::Tls => write!(f, "TLS"),
            ProtocolType::Ssh => write!(f, "SSH"),
            ProtocolType::IpSec => write!(f, "IPSec"),
            ProtocolType::Https => write!(f, "HTTPS"),
            ProtocolType::Other(s) => write!(f, "{}", s),
        }
    }
}

/// Protocol detection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDetection {
    pub protocol_type: ProtocolType,
    pub version: String,
    pub cipher_suites: Vec<String>,
    pub key_exchange: Vec<String>,
    pub configuration: std::collections::HashMap<String, String>,
    pub line: usize,
    pub column: usize,
    pub context: String,
}

/// Configuration violation for ITSP.40.062
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationViolation {
    pub parameter: String,
    pub current_value: String,
    pub required_value: String,
    pub itsp_reference: String,
    pub severity: Severity,
}

/// Protocol compliance against ITSP.40.062
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolCompliance {
    pub protocol: ProtocolDetection,
    pub compliant: bool,
    pub violations: Vec<ConfigurationViolation>,
    pub recommendations: Vec<String>,
}

/// FIPS validation level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FIPSLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}

impl fmt::Display for FIPSLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FIPSLevel::Level1 => write!(f, "Level 1"),
            FIPSLevel::Level2 => write!(f, "Level 2"),
            FIPSLevel::Level3 => write!(f, "Level 3"),
            FIPSLevel::Level4 => write!(f, "Level 4"),
        }
    }
}

/// CMVP certificate status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CMVPStatus {
    Active,
    Historical,
    Revoked,
}

/// CMVP certificate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMVPCertificate {
    pub certificate_number: String,
    pub vendor: String,
    pub module_name: String,
    pub validation_level: FIPSLevel,
    pub algorithms: Vec<String>,
    pub expiry_date: Option<String>,
    pub status: CMVPStatus,
}

/// CMVP validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMVPValidation {
    pub algorithm_used: String,
    pub implementation: Option<String>,
    pub cmvp_cert: Option<CMVPCertificate>,
    pub requires_cmvp: bool,
    pub compliant: bool,
}

/// ITSG-33 Control Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITSG33ControlAssessment {
    pub control_id: String,
    pub control_name: String,
    pub control_family: String,
    pub control_description: String,
    pub implementation_status: ImplementationStatus,
    pub assessment_status: AssessmentStatus,
    pub assessment_method: Vec<String>,
    pub security_classification: SecurityClassification,
    pub nist_mapping: Option<String>,
}

/// Canadian Assessment Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanadianAssessmentSummary {
    // Existing fields from AssessmentSummary
    pub files_scanned: usize,
    pub lines_scanned: usize,
    pub total_vulnerabilities: usize,
    pub quantum_vulnerable_algorithms: Vec<String>,
    pub deprecated_algorithms: Vec<String>,
    pub weak_key_sizes: Vec<String>,
    pub compliance_score: u32,
    pub risk_score: u32,

    // Canadian-specific fields
    pub cccs_approved_algorithms: Vec<String>,
    pub cccs_deprecated_algorithms: Vec<String>,
    pub cccs_prohibited_algorithms: Vec<String>,
    pub cmvp_validated_count: usize,
    pub cmvp_required_count: usize,
    pub itsp_40_111_compliant: bool,
    pub itsp_40_062_compliant: bool,
    pub security_classification: SecurityClassification,
    pub classification_compliant: bool,
}

/// Canadian Control Finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanadianFinding {
    pub finding_id: String,
    pub control_id: String,
    pub implementation_status: ImplementationStatus,
    pub assessment_status: AssessmentStatus,
    pub description: String,
    pub related_vulnerabilities: Vec<String>,
    pub evidence: Vec<Evidence>,
    pub remediation: String,
    pub risk_level: Severity,

    // Canadian-specific fields
    pub cccs_approval_status: CCCSApprovalStatus,
    pub itsp_references: Vec<String>,
    pub cmvp_validation: Option<CMVPValidation>,
    pub applicable_classifications: Vec<SecurityClassification>,
}

/// ITSG-33 SC-13 Assessment Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITSG33Report {
    pub metadata: ReportMetadata,
    pub control_assessment: ITSG33ControlAssessment,
    pub summary: CanadianAssessmentSummary,
    pub findings: Vec<CanadianFinding>,
    pub protocol_compliance: Vec<ProtocolCompliance>,
    pub cmvp_validations: Vec<CMVPValidation>,
    pub recommendations: Vec<String>,
}

/// Unified compliance report (NIST + Canadian)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedComplianceReport {
    pub metadata: ReportMetadata,

    // NIST components
    pub nist_sc13_assessment: ControlAssessment,
    pub nist_summary: AssessmentSummary,
    pub nist_findings: Vec<ControlFinding>,

    // Canadian components
    pub itsg33_sc13_assessment: ITSG33ControlAssessment,
    pub canadian_summary: CanadianAssessmentSummary,
    pub canadian_findings: Vec<CanadianFinding>,

    // Cross-mapping
    pub control_mapping: Vec<ControlCrossReference>,

    // Unified recommendations
    pub recommendations: Vec<String>,
}

/// Control cross-reference between frameworks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCrossReference {
    pub nist_control_id: String,
    pub itsg33_control_id: String,
    pub equivalence: String,
    pub notes: Vec<String>,
}

/// Report language for bilingual output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportLanguage {
    English,
    French,
}

/// Compliance evidence package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvidencePackage {
    pub assessment_date: String,
    pub system_name: String,
    pub classification: SecurityClassification,

    // Algorithm inventory
    pub algorithms_used: Vec<AlgorithmInventoryItem>,

    // CMVP certificates
    pub cmvp_validations: Vec<CMVPValidation>,

    // Protocol configurations
    pub protocol_configurations: Vec<ProtocolCompliance>,

    // Control mappings
    pub itsg33_controls: Vec<ITSG33ControlAssessment>,
    pub nist_controls: Vec<ControlAssessment>,

    // Attestations
    pub attestations: Vec<ComplianceAttestation>,
}

/// Algorithm inventory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmInventoryItem {
    pub algorithm: String,
    pub crypto_type: CryptoType,
    pub usage_count: usize,
    pub locations: Vec<SourceLocation>,
    pub cccs_status: CCCSApprovalStatus,
    pub nist_status: String,
    pub cmvp_required: bool,
    pub cmvp_validated: bool,
}

/// Compliance attestation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAttestation {
    pub attestation_id: String,
    pub framework: String,
    pub control_id: String,
    pub status: AssessmentStatus,
    pub attested_by: Option<String>,
    pub attestation_date: String,
    pub notes: Vec<String>,
}
