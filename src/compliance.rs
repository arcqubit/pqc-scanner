// NIST 800-53 SC-13 Compliance Reporting
// Generates detailed compliance reports with OSCAL JSON output

use crate::types::*;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

const OSCAL_VERSION: &str = "1.1.2";
const REPORT_VERSION: &str = "1.0.0";

/// Generate NIST 800-53 SC-13 Assessment Report from AuditResult
pub fn generate_sc13_report(
    audit_result: &AuditResult,
    file_path: Option<&str>,
) -> SC13AssessmentReport {
    let now = Utc::now();
    let timestamp = now.to_rfc3339();
    let report_id = Uuid::new_v4().to_string();

    // Determine implementation status based on vulnerabilities
    let (implementation_status, assessment_status) = assess_implementation(audit_result);

    // Generate metadata
    let metadata = ReportMetadata {
        report_id: report_id.clone(),
        title: "NIST 800-53 SC-13 Cryptographic Protection Assessment".to_string(),
        published: timestamp.clone(),
        last_modified: timestamp.clone(),
        version: REPORT_VERSION.to_string(),
        oscal_version: OSCAL_VERSION.to_string(),
    };

    // Generate control assessment
    let control_assessment = ControlAssessment {
        control_id: "sc-13".to_string(),
        control_name: "Cryptographic Protection".to_string(),
        control_family: "System and Communications Protection".to_string(),
        control_description: "The information system implements [Assignment: organization-defined cryptographic uses and type of cryptography required for each use] in accordance with applicable federal laws, Executive Orders, directives, policies, regulations, and standards.".to_string(),
        implementation_status: implementation_status.clone(),
        assessment_status: assessment_status.clone(),
        assessment_method: vec![
            "TEST".to_string(),
            "EXAMINE".to_string(),
            "INTERVIEW".to_string(),
        ],
    };

    // Generate summary
    let summary = generate_summary(audit_result);

    // Generate detailed findings
    let findings = generate_findings(audit_result, &timestamp, file_path);

    // Generate recommendations
    let recommendations = generate_compliance_recommendations(audit_result);

    SC13AssessmentReport {
        metadata,
        control_assessment,
        summary,
        findings,
        recommendations,
    }
}

/// Assess implementation status based on vulnerabilities
fn assess_implementation(audit_result: &AuditResult) -> (ImplementationStatus, AssessmentStatus) {
    let total_vulns = audit_result.stats.total_vulnerabilities;
    let critical_count = audit_result.stats.critical_count;
    let high_count = audit_result.stats.high_count;

    if total_vulns == 0 {
        return (
            ImplementationStatus::Implemented,
            AssessmentStatus::Satisfied,
        );
    }

    if critical_count > 0 || high_count > 5 {
        (
            ImplementationStatus::PartiallyImplemented,
            AssessmentStatus::NotSatisfied,
        )
    } else if high_count > 0 {
        (
            ImplementationStatus::PartiallyImplemented,
            AssessmentStatus::Other,
        )
    } else {
        (ImplementationStatus::Implemented, AssessmentStatus::Other)
    }
}

/// Generate assessment summary
fn generate_summary(audit_result: &AuditResult) -> AssessmentSummary {
    let mut quantum_vulnerable = Vec::new();
    let mut deprecated = Vec::new();
    let mut weak_keys = Vec::new();

    for vuln in &audit_result.vulnerabilities {
        let crypto_name = vuln.crypto_type.to_string();

        // Categorize algorithms
        match vuln.crypto_type {
            CryptoType::Rsa
            | CryptoType::Ecdsa
            | CryptoType::Ecdh
            | CryptoType::Dsa
            | CryptoType::DiffieHellman => {
                if !quantum_vulnerable.contains(&crypto_name) {
                    quantum_vulnerable.push(crypto_name.clone());
                }
            }
            CryptoType::Sha1
            | CryptoType::Md5
            | CryptoType::Des
            | CryptoType::TripleDes
            | CryptoType::Rc4 => {
                if !deprecated.contains(&crypto_name) {
                    deprecated.push(crypto_name.clone());
                }
            }
        }

        // Track weak key sizes
        if let Some(key_size) = vuln.key_size {
            let key_info = format!("{} {}-bit", crypto_name, key_size);
            if key_size < 2048 && !weak_keys.contains(&key_info) {
                weak_keys.push(key_info);
            }
        }
    }

    // Calculate compliance score (inverse of risk score)
    let compliance_score = if audit_result.risk_score == 0 {
        100
    } else {
        100 - audit_result.risk_score
    };

    AssessmentSummary {
        files_scanned: 1,
        lines_scanned: audit_result.stats.lines_scanned,
        total_vulnerabilities: audit_result.stats.total_vulnerabilities,
        quantum_vulnerable_algorithms: quantum_vulnerable,
        deprecated_algorithms: deprecated,
        weak_key_sizes: weak_keys,
        compliance_score,
        risk_score: audit_result.risk_score,
    }
}

/// Generate detailed findings with evidence
fn generate_findings(
    audit_result: &AuditResult,
    timestamp: &str,
    file_path: Option<&str>,
) -> Vec<ControlFinding> {
    let mut findings = Vec::new();

    // Group vulnerabilities by crypto type
    let mut vuln_groups: std::collections::HashMap<String, Vec<&Vulnerability>> =
        std::collections::HashMap::new();

    for vuln in &audit_result.vulnerabilities {
        let key = vuln.crypto_type.to_string();
        vuln_groups.entry(key).or_default().push(vuln);
    }

    // Create findings for each crypto type
    for (crypto_type, vulns) in vuln_groups {
        let finding_id = Uuid::new_v4().to_string();
        let highest_severity = vulns.iter().map(|v| v.severity).max().unwrap();

        // Determine implementation status for this finding
        let (impl_status, assess_status) = if highest_severity >= Severity::High {
            (
                ImplementationStatus::NotApplicable,
                AssessmentStatus::NotSatisfied,
            )
        } else {
            (
                ImplementationStatus::PartiallyImplemented,
                AssessmentStatus::Other,
            )
        };

        // Collect evidence for all instances
        let mut evidence = Vec::new();
        let mut related_vulns = Vec::new();

        for (idx, vuln) in vulns.iter().enumerate() {
            let evidence_id = format!("{}-{}", finding_id, idx);
            related_vulns.push(format!(
                "{}:{}:{}",
                file_path.unwrap_or("source"),
                vuln.line,
                vuln.column
            ));

            let source_location = SourceLocation {
                file_path: file_path.unwrap_or("source").to_string(),
                line: vuln.line,
                column: vuln.column,
                snippet: vuln.context.clone(),
            };

            let evidence_data = json!({
                "crypto_type": crypto_type,
                "severity": format!("{:?}", vuln.severity),
                "risk_score": vuln.risk_score,
                "key_size": vuln.key_size,
                "message": vuln.message,
            });

            evidence.push(Evidence {
                evidence_id,
                evidence_type: EvidenceType::StaticScan,
                description: format!(
                    "Detected {} at line {} column {}: {}",
                    crypto_type, vuln.line, vuln.column, vuln.message
                ),
                source_location: Some(source_location),
                collected_at: timestamp.to_string(),
                data: evidence_data,
            });
        }

        let description = format!(
            "Found {} instance(s) of {} cryptographic algorithm usage. \
            This algorithm is {} and poses a {} risk to cryptographic protection.",
            vulns.len(),
            crypto_type,
            if is_quantum_vulnerable(&vulns[0].crypto_type) {
                "quantum-vulnerable"
            } else {
                "cryptographically deprecated"
            },
            format!("{:?}", highest_severity).to_lowercase()
        );

        let remediation = vulns[0].recommendation.clone();

        findings.push(ControlFinding {
            finding_id,
            control_id: "sc-13".to_string(),
            implementation_status: impl_status,
            assessment_status: assess_status,
            description,
            related_vulnerabilities: related_vulns,
            evidence,
            remediation,
            risk_level: highest_severity,
        });
    }

    findings
}

/// Check if crypto type is quantum vulnerable
fn is_quantum_vulnerable(crypto_type: &CryptoType) -> bool {
    matches!(
        crypto_type,
        CryptoType::Rsa
            | CryptoType::Ecdsa
            | CryptoType::Ecdh
            | CryptoType::Dsa
            | CryptoType::DiffieHellman
    )
}

/// Generate compliance-specific recommendations
fn generate_compliance_recommendations(audit_result: &AuditResult) -> Vec<String> {
    let mut recommendations = Vec::new();

    recommendations.push(
        "SC-13 Control Objective: Implement FIPS 140-2/140-3 validated cryptographic modules for cryptographic protection.".to_string()
    );

    if audit_result.stats.critical_count > 0 {
        recommendations.push(
            "CRITICAL: Immediate action required. The use of cryptographically broken algorithms (MD5, SHA-1, DES, RC4) violates SC-13 requirements and poses significant security risks.".to_string()
        );
    }

    if audit_result.stats.high_count > 0 {
        recommendations.push(
            "HIGH PRIORITY: Transition to post-quantum cryptography (PQC) algorithms to comply with NIST SP 800-208 and prepare for quantum computing threats.".to_string()
        );

        recommendations.push(
            "Recommended PQC Algorithms: CRYSTALS-Kyber (key encapsulation), CRYSTALS-Dilithium (digital signatures), SPHINCS+ (stateless signatures).".to_string()
        );
    }

    recommendations.push(
        "Implement crypto-agility: Design systems to easily swap cryptographic algorithms as new standards emerge.".to_string()
    );

    recommendations.push(
        "Maintain a cryptographic inventory: Document all cryptographic implementations and their compliance status.".to_string()
    );

    recommendations.push(
        "Reference: NIST SP 800-53 Rev. 5 SC-13, NIST SP 800-175B (Cryptographic Algorithm Validation Program), NIST Post-Quantum Cryptography Standardization.".to_string()
    );

    recommendations
}

/// Generate OSCAL Assessment Results JSON
pub fn generate_oscal_json(
    sc13_report: &SC13AssessmentReport,
    _file_path: Option<&str>,
) -> OscalAssessmentResults {
    let now = Utc::now();
    let timestamp = now.to_rfc3339();
    let assessment_uuid = Uuid::new_v4().to_string();

    // Create metadata
    let metadata = OscalMetadata {
        title: "Cryptographic Protection Assessment Results".to_string(),
        published: sc13_report.metadata.published.clone(),
        last_modified: sc13_report.metadata.last_modified.clone(),
        version: sc13_report.metadata.version.clone(),
        oscal_version: OSCAL_VERSION.to_string(),
        roles: Some(vec![Role {
            id: "assessor".to_string(),
            title: "Security Assessor".to_string(),
        }]),
        parties: Some(vec![Party {
            uuid: Uuid::new_v4().to_string(),
            party_type: "organization".to_string(),
            name: "Security Assessment Team".to_string(),
        }]),
    };

    // Create observations from evidence
    let mut observations = Vec::new();
    for finding in &sc13_report.findings {
        for evidence in &finding.evidence {
            let obs_uuid = Uuid::new_v4().to_string();

            let relevant_evidence = if let Some(ref loc) = evidence.source_location {
                Some(vec![RelevantEvidence {
                    href: format!("#{}:{}", loc.file_path, loc.line),
                    description: format!("Code location: {}:{}", loc.file_path, loc.line),
                }])
            } else {
                None
            };

            observations.push(Observation {
                uuid: obs_uuid,
                description: evidence.description.clone(),
                methods: vec!["TEST".to_string()],
                types: Some(vec![format!("{:?}", evidence.evidence_type)]),
                collected: Some(evidence.collected_at.clone()),
                relevant_evidence,
            });
        }
    }

    // Create findings
    let mut oscal_findings = Vec::new();
    for finding in &sc13_report.findings {
        let finding_uuid = Uuid::new_v4().to_string();

        let state = match finding.assessment_status {
            AssessmentStatus::Satisfied => "satisfied",
            AssessmentStatus::NotSatisfied => "not-satisfied",
            AssessmentStatus::Other => "other",
        };

        oscal_findings.push(Finding {
            uuid: finding_uuid,
            title: format!(
                "SC-13 Finding: {}",
                finding.description.split('.').next().unwrap_or("Finding")
            ),
            description: finding.description.clone(),
            target: Target {
                target_type: "objective-id".to_string(),
                target_id: "sc-13".to_string(),
                status: Some(TargetStatus {
                    state: state.to_string(),
                }),
            },
            implementation_status: OscalImplementationStatus {
                state: match finding.implementation_status {
                    ImplementationStatus::Implemented => "implemented",
                    ImplementationStatus::PartiallyImplemented => "partial",
                    ImplementationStatus::PlannedForImplementation => "planned",
                    ImplementationStatus::AlternativeImplementation => "alternative",
                    ImplementationStatus::NotApplicable => "not-applicable",
                }
                .to_string(),
            },
            related_observations: Some(
                observations
                    .iter()
                    .map(|obs| RelatedObservation {
                        observation_uuid: obs.uuid.clone(),
                    })
                    .collect(),
            ),
        });
    }

    // Create result
    let result = AssessmentResult {
        uuid: Uuid::new_v4().to_string(),
        title: "Quantum-Safe Cryptography Assessment".to_string(),
        description: format!(
            "Assessment of cryptographic implementations against NIST 800-53 SC-13 requirements. \
            Detected {} vulnerabilities across {} lines of code.",
            sc13_report.summary.total_vulnerabilities, sc13_report.summary.lines_scanned
        ),
        start: timestamp.clone(),
        end: Some(timestamp.clone()),
        reviewed_controls: ReviewedControls {
            control_selections: vec![ControlSelection {
                include_controls: vec![ControlRef {
                    control_id: "sc-13".to_string(),
                }],
            }],
        },
        observations,
        findings: oscal_findings,
    };

    // Create assessment results
    let assessment_results = AssessmentResults {
        uuid: assessment_uuid,
        metadata,
        import_ssp: ImportSSP {
            href: "#system-security-plan".to_string(),
        },
        results: vec![result],
    };

    OscalAssessmentResults {
        oscal_version: OSCAL_VERSION.to_string(),
        assessment_results,
    }
}

/// Export SC-13 report to JSON string
pub fn export_sc13_json(report: &SC13AssessmentReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

/// Export OSCAL Assessment Results to JSON string
pub fn export_oscal_json(oscal: &OscalAssessmentResults) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(oscal)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_audit_result() -> AuditResult {
        let mut result = AuditResult::new(Language::JavaScript, 100);

        result.add_vulnerability(Vulnerability {
            crypto_type: CryptoType::Rsa,
            severity: Severity::High,
            risk_score: 85,
            line: 10,
            column: 5,
            context: "const rsa = crypto.generateKeyPair('rsa', { modulusLength: 2048 })"
                .to_string(),
            message: "RSA detected - quantum vulnerable".to_string(),
            recommendation: "Replace with CRYSTALS-Kyber".to_string(),
            key_size: Some(2048),
        });

        result.add_vulnerability(Vulnerability {
            crypto_type: CryptoType::Md5,
            severity: Severity::Critical,
            risk_score: 100,
            line: 15,
            column: 10,
            context: "const hash = crypto.createHash('md5')".to_string(),
            message: "MD5 is cryptographically broken".to_string(),
            recommendation: "Replace with SHA-256".to_string(),
            key_size: None,
        });

        result.calculate_risk_score();
        result.generate_recommendations();
        result
    }

    #[test]
    fn test_generate_sc13_report() {
        let audit_result = create_test_audit_result();
        let report = generate_sc13_report(&audit_result, Some("test.js"));

        assert_eq!(
            report.metadata.title,
            "NIST 800-53 SC-13 Cryptographic Protection Assessment"
        );
        assert_eq!(report.control_assessment.control_id, "sc-13");
        assert_eq!(report.findings.len(), 2); // RSA and MD5
        assert!(report.summary.total_vulnerabilities > 0);
    }

    #[test]
    fn test_assess_implementation() {
        let mut result = AuditResult::new(Language::Rust, 50);

        // Test with no vulnerabilities
        let (impl_status, assess_status) = assess_implementation(&result);
        assert_eq!(impl_status, ImplementationStatus::Implemented);
        assert_eq!(assess_status, AssessmentStatus::Satisfied);

        // Test with critical vulnerability
        result.add_vulnerability(Vulnerability {
            crypto_type: CryptoType::Md5,
            severity: Severity::Critical,
            risk_score: 100,
            line: 1,
            column: 1,
            context: "md5".to_string(),
            message: "test".to_string(),
            recommendation: "test".to_string(),
            key_size: None,
        });

        let (impl_status, assess_status) = assess_implementation(&result);
        assert_eq!(impl_status, ImplementationStatus::PartiallyImplemented);
        assert_eq!(assess_status, AssessmentStatus::NotSatisfied);
    }

    #[test]
    fn test_generate_oscal_json() {
        let audit_result = create_test_audit_result();
        let report = generate_sc13_report(&audit_result, Some("test.js"));
        let oscal = generate_oscal_json(&report, Some("test.js"));

        assert_eq!(oscal.oscal_version, OSCAL_VERSION);
        assert!(!oscal.assessment_results.results.is_empty());
        assert!(!oscal.assessment_results.results[0].findings.is_empty());
    }

    #[test]
    fn test_export_json() {
        let audit_result = create_test_audit_result();
        let report = generate_sc13_report(&audit_result, Some("test.js"));

        let json_result = export_sc13_json(&report);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("sc-13"));
        assert!(json_str.contains("Cryptographic Protection"));
    }

    #[test]
    fn test_export_oscal_json() {
        let audit_result = create_test_audit_result();
        let report = generate_sc13_report(&audit_result, Some("test.js"));
        let oscal = generate_oscal_json(&report, Some("test.js"));

        let json_result = export_oscal_json(&oscal);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();
        assert!(json_str.contains("oscal-version"));
        assert!(json_str.contains("assessment-results"));
    }
}
