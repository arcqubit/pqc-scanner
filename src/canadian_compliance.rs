// Canadian CCCS/CSE Cryptographic Compliance Module
// ITSG-33 SC-13, ITSP.40.111, and ITSP.40.062 compliance assessment

use crate::algorithm_database;
use crate::types::*;
use chrono::Utc;
use uuid::Uuid;

const REPORT_VERSION: &str = "1.0.0";

/// Generate ITSG-33 SC-13 Assessment Report from AuditResult
pub fn generate_itsg33_report(
    audit_result: &AuditResult,
    classification: SecurityClassification,
    file_path: Option<&str>,
) -> ITSG33Report {
    let now = Utc::now();
    let timestamp = now.to_rfc3339();
    let report_id = Uuid::new_v4().to_string();

    // Determine implementation status
    let (implementation_status, assessment_status) =
        assess_canadian_implementation(audit_result, classification);

    // Generate metadata
    let metadata = ReportMetadata {
        report_id: report_id.clone(),
        title: format!(
            "ITSG-33 SC-13 Cryptographic Protection Assessment - {}",
            classification
        ),
        published: timestamp.clone(),
        last_modified: timestamp.clone(),
        version: REPORT_VERSION.to_string(),
        oscal_version: "1.1.2".to_string(),
    };

    // Generate ITSG-33 control assessment
    let control_assessment = generate_itsg33_control_assessment(
        &implementation_status,
        &assessment_status,
        classification,
    );

    // Generate Canadian summary
    let summary = generate_canadian_summary(audit_result, classification);

    // Generate Canadian findings
    let findings = generate_canadian_findings(audit_result, &timestamp, classification, file_path);

    // Protocol compliance (empty for now, will be populated by protocol detection)
    let protocol_compliance = Vec::new();

    // CMVP validations
    let cmvp_validations = generate_cmvp_validations(audit_result);

    // Generate recommendations
    let recommendations = generate_canadian_recommendations(audit_result, classification);

    ITSG33Report {
        metadata,
        control_assessment,
        summary,
        findings,
        protocol_compliance,
        cmvp_validations,
        recommendations,
    }
}

/// Generate unified NIST + Canadian compliance report
pub fn generate_unified_report(
    audit_result: &AuditResult,
    classification: SecurityClassification,
    file_path: Option<&str>,
) -> UnifiedComplianceReport {
    let now = Utc::now();
    let timestamp = now.to_rfc3339();

    // Generate NIST SC-13 report
    let nist_report = crate::compliance::generate_sc13_report(audit_result, file_path);

    // Generate ITSG-33 report
    let canadian_report = generate_itsg33_report(audit_result, classification, file_path);

    // Generate control mapping
    let control_mapping = vec![ControlCrossReference {
        nist_control_id: "SC-13".to_string(),
        itsg33_control_id: "ITSG-33 SC-13".to_string(),
        equivalence: "1:1 mapping - ITSG-33 is based on NIST 800-53".to_string(),
        notes: vec![
            "Both frameworks require FIPS-validated/CMVP-validated cryptography".to_string(),
            "Canadian framework adds specific ITSP.40.111 and ITSP.40.062 requirements".to_string(),
            "Security classification levels (Protected A/B/C) determine minimum key sizes"
                .to_string(),
        ],
    }];

    // Generate unified recommendations
    let mut recommendations = Vec::new();
    recommendations.push("Unified Compliance Assessment: Both NIST 800-53 SC-13 and ITSG-33 SC-13 are assessed".to_string());
    recommendations.push(
        "Use CMVP-validated cryptographic modules to satisfy both U.S. and Canadian requirements"
            .to_string(),
    );
    recommendations.extend(canadian_report.recommendations.clone());

    UnifiedComplianceReport {
        metadata: ReportMetadata {
            report_id: Uuid::new_v4().to_string(),
            title: "Unified NIST 800-53 SC-13 and ITSG-33 SC-13 Compliance Assessment".to_string(),
            published: timestamp.clone(),
            last_modified: timestamp,
            version: REPORT_VERSION.to_string(),
            oscal_version: "1.1.2".to_string(),
        },
        nist_sc13_assessment: nist_report.control_assessment,
        nist_summary: nist_report.summary,
        nist_findings: nist_report.findings,
        itsg33_sc13_assessment: canadian_report.control_assessment,
        canadian_summary: canadian_report.summary,
        canadian_findings: canadian_report.findings,
        control_mapping,
        recommendations,
    }
}

/// Assess implementation status for Canadian compliance
fn assess_canadian_implementation(
    audit_result: &AuditResult,
    classification: SecurityClassification,
) -> (ImplementationStatus, AssessmentStatus) {
    let total_vulns = audit_result.stats.total_vulnerabilities;

    // Check for prohibited algorithms
    let has_prohibited = audit_result
        .vulnerabilities
        .iter()
        .any(|v| algorithm_database::is_cccs_prohibited(&v.crypto_type));

    // Check for deprecated algorithms
    let has_deprecated = audit_result
        .vulnerabilities
        .iter()
        .any(|v| algorithm_database::is_cccs_deprecated(&v.crypto_type));

    // Check for key size violations
    let has_key_size_violations = audit_result.vulnerabilities.iter().any(|v| {
        if let Some(key_size) = v.key_size {
            !algorithm_database::validate_key_size(&v.crypto_type, key_size, classification)
        } else {
            false
        }
    });

    if total_vulns == 0 {
        return (
            ImplementationStatus::Implemented,
            AssessmentStatus::Satisfied,
        );
    }

    if has_prohibited {
        (
            ImplementationStatus::PartiallyImplemented,
            AssessmentStatus::NotSatisfied,
        )
    } else if has_deprecated || has_key_size_violations {
        (
            ImplementationStatus::PartiallyImplemented,
            AssessmentStatus::Other,
        )
    } else {
        (ImplementationStatus::Implemented, AssessmentStatus::Other)
    }
}

/// Generate ITSG-33 control assessment
fn generate_itsg33_control_assessment(
    implementation_status: &ImplementationStatus,
    assessment_status: &AssessmentStatus,
    classification: SecurityClassification,
) -> ITSG33ControlAssessment {
    ITSG33ControlAssessment {
        control_id: "ITSG-33 SC-13".to_string(),
        control_name: "Use of Cryptography / Cryptographic Protection".to_string(),
        control_family: "System and Communications Protection".to_string(),
        control_description: "The information system implements cryptographic protection in accordance with applicable Government of Canada legislation, TBS policies, directives, standards, and CCCS/CSE guidance (ITSP.40.111, ITSP.40.062).".to_string(),
        implementation_status: implementation_status.clone(),
        assessment_status: assessment_status.clone(),
        assessment_method: vec![
            "TEST".to_string(),
            "EXAMINE".to_string(),
            "INTERVIEW".to_string(),
        ],
        security_classification: classification,
        nist_mapping: Some("NIST 800-53 Rev. 5 SC-13".to_string()),
    }
}

/// Generate Canadian assessment summary
fn generate_canadian_summary(
    audit_result: &AuditResult,
    classification: SecurityClassification,
) -> CanadianAssessmentSummary {
    let mut quantum_vulnerable = Vec::new();
    let mut deprecated = Vec::new();
    let mut weak_keys = Vec::new();

    let mut cccs_deprecated_list = Vec::new();
    let mut cccs_prohibited = Vec::new();

    for vuln in &audit_result.vulnerabilities {
        let crypto_name = vuln.crypto_type.to_string();

        // Categorize by quantum vulnerability
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

        // Categorize by CCCS status
        if algorithm_database::is_cccs_prohibited(&vuln.crypto_type) {
            if !cccs_prohibited.contains(&crypto_name) {
                cccs_prohibited.push(crypto_name.clone());
            }
        } else if algorithm_database::is_cccs_deprecated(&vuln.crypto_type) {
            if !cccs_deprecated_list.contains(&crypto_name) {
                cccs_deprecated_list.push(crypto_name.clone());
            }
        }

        // Track weak key sizes
        if let Some(key_size) = vuln.key_size {
            if !algorithm_database::validate_key_size(&vuln.crypto_type, key_size, classification)
            {
                let key_info = format!("{} {}-bit", crypto_name, key_size);
                if !weak_keys.contains(&key_info) {
                    weak_keys.push(key_info);
                }
            }
        }
    }

    // Get approved algorithms for this classification
    let cccs_approved = algorithm_database::get_approved_algorithms(classification);

    // Calculate CMVP validation counts
    let cmvp_required_count = audit_result
        .vulnerabilities
        .iter()
        .filter(|v| {
            let status = algorithm_database::get_cccs_status(&v.crypto_type);
            matches!(
                status,
                CCCSApprovalStatus::Approved | CCCSApprovalStatus::ConditionallyApproved
            )
        })
        .count();
    let cmvp_validated_count = 0; // Will be updated with actual CMVP validation

    // Check ITSP compliance
    let itsp_40_111_compliant = cccs_prohibited.is_empty() && weak_keys.is_empty();
    let itsp_40_062_compliant = true; // Will be updated with protocol checks

    // Check classification compliance
    let classification_compliant = weak_keys.is_empty();

    // Calculate compliance score
    let compliance_score = calculate_canadian_compliance_score(
        audit_result,
        &cccs_prohibited,
        &cccs_deprecated_list,
        &weak_keys,
    );

    CanadianAssessmentSummary {
        files_scanned: 1,
        lines_scanned: audit_result.stats.lines_scanned,
        total_vulnerabilities: audit_result.stats.total_vulnerabilities,
        quantum_vulnerable_algorithms: quantum_vulnerable,
        deprecated_algorithms: deprecated,
        weak_key_sizes: weak_keys,
        compliance_score,
        risk_score: audit_result.risk_score,
        cccs_approved_algorithms: cccs_approved,
        cccs_deprecated_algorithms: cccs_deprecated_list,
        cccs_prohibited_algorithms: cccs_prohibited,
        cmvp_validated_count,
        cmvp_required_count,
        itsp_40_111_compliant,
        itsp_40_062_compliant,
        security_classification: classification,
        classification_compliant,
    }
}

/// Calculate Canadian compliance score
fn calculate_canadian_compliance_score(
    audit_result: &AuditResult,
    prohibited: &[String],
    deprecated: &[String],
    weak_keys: &[String],
) -> u32 {
    if audit_result.stats.total_vulnerabilities == 0 {
        return 100;
    }

    let mut score = 100u32;

    // Prohibited algorithms are the worst (-40 points each)
    score = score.saturating_sub(prohibited.len() as u32 * 40);

    // Deprecated algorithms (-20 points each)
    score = score.saturating_sub(deprecated.len() as u32 * 20);

    // Weak key sizes (-15 points each)
    score = score.saturating_sub(weak_keys.len() as u32 * 15);

    // Quantum-vulnerable algorithms (-10 points per unique type)
    let quantum_count = audit_result
        .vulnerabilities
        .iter()
        .filter(|v| {
            matches!(
                v.crypto_type,
                CryptoType::Rsa
                    | CryptoType::Ecdsa
                    | CryptoType::Ecdh
                    | CryptoType::Dsa
                    | CryptoType::DiffieHellman
            )
        })
        .map(|v| v.crypto_type.to_string())
        .collect::<std::collections::HashSet<_>>()
        .len();
    score = score.saturating_sub(quantum_count as u32 * 10);

    score
}

/// Generate Canadian findings with CCCS approval status
fn generate_canadian_findings(
    audit_result: &AuditResult,
    timestamp: &str,
    classification: SecurityClassification,
    file_path: Option<&str>,
) -> Vec<CanadianFinding> {
    let mut findings = Vec::new();

    // Group vulnerabilities by crypto type
    let mut vuln_groups: std::collections::HashMap<String, Vec<&Vulnerability>> =
        std::collections::HashMap::new();

    for vuln in &audit_result.vulnerabilities {
        let key = vuln.crypto_type.to_string();
        vuln_groups.entry(key).or_default().push(vuln);
    }

    // Create findings for each crypto type
    for (crypto_type_str, vulns) in vuln_groups {
        let finding_id = Uuid::new_v4().to_string();
        let highest_severity = vulns.iter().map(|v| v.severity).max().unwrap();
        let crypto_type = &vulns[0].crypto_type;

        // Get CCCS approval status
        let cccs_approval_status = algorithm_database::get_cccs_status(crypto_type);

        // Determine applicable classifications
        let applicable_classifications = determine_applicable_classifications(
            crypto_type,
            vulns[0].key_size,
            &cccs_approval_status,
        );

        // Get ITSP references
        let itsp_references = vec![algorithm_database::get_itsp_reference(crypto_type)];

        // Determine implementation status
        let (impl_status, assess_status) = match cccs_approval_status {
            CCCSApprovalStatus::Prohibited => (
                ImplementationStatus::NotApplicable,
                AssessmentStatus::NotSatisfied,
            ),
            CCCSApprovalStatus::Deprecated => (
                ImplementationStatus::PartiallyImplemented,
                AssessmentStatus::Other,
            ),
            _ => {
                if highest_severity >= Severity::High {
                    (
                        ImplementationStatus::PartiallyImplemented,
                        AssessmentStatus::Other,
                    )
                } else {
                    (ImplementationStatus::Implemented, AssessmentStatus::Other)
                }
            }
        };

        // Collect evidence
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

            let evidence_data = serde_json::json!({
                "crypto_type": crypto_type_str,
                "cccs_status": cccs_approval_status.to_string(),
                "severity": format!("{:?}", vuln.severity),
                "risk_score": vuln.risk_score,
                "key_size": vuln.key_size,
                "classification": classification.to_string(),
            });

            evidence.push(Evidence {
                evidence_id,
                evidence_type: EvidenceType::StaticScan,
                description: format!(
                    "Detected {} (CCCS Status: {}) at line {} column {}: {}",
                    crypto_type_str,
                    cccs_approval_status,
                    vuln.line,
                    vuln.column,
                    vuln.message
                ),
                source_location: Some(source_location),
                collected_at: timestamp.to_string(),
                data: evidence_data,
            });
        }

        let description = generate_canadian_finding_description(
            &crypto_type_str,
            vulns.len(),
            &cccs_approval_status,
            classification,
            vulns[0].key_size,
        );

        let remediation = generate_canadian_remediation(crypto_type, &cccs_approval_status);

        // CMVP validation (if applicable)
        let cmvp_validation = match cccs_approval_status {
            CCCSApprovalStatus::Approved | CCCSApprovalStatus::ConditionallyApproved => {
                Some(CMVPValidation {
                    algorithm_used: crypto_type_str.clone(),
                    implementation: None,
                    cmvp_cert: None,
                    requires_cmvp: algorithm_database::is_cmvp_required(classification),
                    compliant: false, // Will be updated with actual validation
                })
            }
            _ => None,
        };

        findings.push(CanadianFinding {
            finding_id,
            control_id: "ITSG-33 SC-13".to_string(),
            implementation_status: impl_status,
            assessment_status: assess_status,
            description,
            related_vulnerabilities: related_vulns,
            evidence,
            remediation,
            risk_level: highest_severity,
            cccs_approval_status,
            itsp_references,
            cmvp_validation,
            applicable_classifications,
        });
    }

    findings
}

/// Determine which classification levels this algorithm/key size is acceptable for
fn determine_applicable_classifications(
    crypto_type: &CryptoType,
    key_size: Option<u32>,
    cccs_status: &CCCSApprovalStatus,
) -> Vec<SecurityClassification> {
    if matches!(
        cccs_status,
        CCCSApprovalStatus::Prohibited | CCCSApprovalStatus::Deprecated
    ) {
        return Vec::new();
    }

    let mut applicable = Vec::new();
    let classifications = vec![
        SecurityClassification::Unclassified,
        SecurityClassification::ProtectedA,
        SecurityClassification::ProtectedB,
        SecurityClassification::ProtectedC,
    ];

    for classification in classifications {
        if let Some(ks) = key_size {
            if algorithm_database::validate_key_size(crypto_type, ks, classification) {
                applicable.push(classification);
            }
        } else {
            // No key size info, assume conditionally approved
            if matches!(
                cccs_status,
                CCCSApprovalStatus::Approved | CCCSApprovalStatus::ConditionallyApproved
            ) {
                applicable.push(classification);
            }
        }
    }

    applicable
}

/// Generate Canadian finding description
fn generate_canadian_finding_description(
    crypto_type: &str,
    count: usize,
    cccs_status: &CCCSApprovalStatus,
    classification: SecurityClassification,
    key_size: Option<u32>,
) -> String {
    let mut desc = format!(
        "Found {} instance(s) of {} cryptographic algorithm. ",
        count, crypto_type
    );

    desc.push_str(&format!("CCCS Status: {}. ", cccs_status));

    match cccs_status {
        CCCSApprovalStatus::Prohibited => {
            desc.push_str(
                "This algorithm is PROHIBITED by CCCS and must not be used under any circumstances. ",
            );
        }
        CCCSApprovalStatus::Deprecated => {
            desc.push_str("This algorithm is DEPRECATED by CCCS and should be migrated immediately. ");
        }
        CCCSApprovalStatus::ConditionallyApproved => {
            desc.push_str("This algorithm is CONDITIONALLY APPROVED for legacy systems only. ");
            desc.push_str("Plan post-quantum migration. ");
        }
        CCCSApprovalStatus::Approved => {
            desc.push_str("This algorithm is APPROVED by CCCS. ");
        }
        CCCSApprovalStatus::UnderReview => {
            desc.push_str("This algorithm is UNDER REVIEW by CCCS. ");
        }
    }

    if let Some(ks) = key_size {
        desc.push_str(&format!(
            "Key size: {} bits for {} classification. ",
            ks, classification
        ));
    }

    desc
}

/// Generate Canadian remediation recommendation
fn generate_canadian_remediation(
    crypto_type: &CryptoType,
    cccs_status: &CCCSApprovalStatus,
) -> String {
    match cccs_status {
        CCCSApprovalStatus::Prohibited => {
            format!(
                "IMMEDIATE ACTION REQUIRED: Replace {} with CCCS-approved alternatives. See ITSP.40.111 for approved algorithms.",
                crypto_type
            )
        }
        CCCSApprovalStatus::Deprecated => {
            format!(
                "Migrate from {} to CCCS-approved alternatives (e.g., AES, SHA-256). See ITSP.40.111 Annex A.",
                crypto_type
            )
        }
        CCCSApprovalStatus::ConditionallyApproved => {
            let conditions = algorithm_database::get_approval_conditions(crypto_type);
            let mut remediation = format!(
                "{} is conditionally approved. Conditions: ",
                crypto_type
            );
            remediation.push_str(&conditions.join("; "));
            remediation.push_str(
                ". Recommended: Plan migration to post-quantum algorithms (CRYSTALS-Kyber, CRYSTALS-Dilithium).",
            );
            remediation
        }
        _ => format!(
            "Ensure {} implementation uses CMVP-validated cryptographic modules in FIPS-approved mode.",
            crypto_type
        ),
    }
}

/// Generate CMVP validations for algorithms
fn generate_cmvp_validations(audit_result: &AuditResult) -> Vec<CMVPValidation> {
    let mut validations = Vec::new();

    for vuln in &audit_result.vulnerabilities {
        let cccs_status = algorithm_database::get_cccs_status(&vuln.crypto_type);

        if matches!(
            cccs_status,
            CCCSApprovalStatus::Approved | CCCSApprovalStatus::ConditionallyApproved
        ) {
            validations.push(CMVPValidation {
                algorithm_used: vuln.crypto_type.to_string(),
                implementation: None,
                cmvp_cert: None,
                requires_cmvp: true,
                compliant: false, // Will be updated with actual validation
            });
        }
    }

    validations
}

/// Generate Canadian recommendations
fn generate_canadian_recommendations(
    audit_result: &AuditResult,
    classification: SecurityClassification,
) -> Vec<String> {
    let mut recommendations = Vec::new();

    recommendations.push(format!(
        "ITSG-33 SC-13 Control Objective: Implement cryptographic protection in accordance with CCCS/CSE guidance for {} information.",
        classification
    ));

    recommendations.push(
        "Use CMVP-validated cryptographic modules (FIPS 140-2/140-3) in FIPS-approved mode."
            .to_string(),
    );

    // Check for prohibited algorithms
    let has_prohibited = audit_result
        .vulnerabilities
        .iter()
        .any(|v| algorithm_database::is_cccs_prohibited(&v.crypto_type));

    if has_prohibited {
        recommendations.push(
            "CRITICAL: Prohibited algorithms detected (MD5, SHA-1, DES, RC4). Immediate replacement required per ITSP.40.111.".to_string()
        );
    }

    // Check for deprecated algorithms
    let has_deprecated = audit_result
        .vulnerabilities
        .iter()
        .any(|v| algorithm_database::is_cccs_deprecated(&v.crypto_type));

    if has_deprecated {
        recommendations.push(
            "HIGH PRIORITY: Deprecated algorithms detected (3DES, DSA). Plan migration to approved alternatives.".to_string()
        );
    }

    // Check for quantum-vulnerable algorithms
    if audit_result.stats.high_count > 0 {
        recommendations.push(
            "Quantum-vulnerable algorithms detected (RSA, ECDSA, ECDH, DH). Plan post-quantum migration by 2030 per CCCS guidance.".to_string()
        );

        recommendations.push(
            "Recommended PQC Algorithms: CRYSTALS-Kyber (key encapsulation), CRYSTALS-Dilithium (digital signatures), SPHINCS+ (stateless signatures).".to_string()
        );
    }

    recommendations.push(
        "Implement crypto-agility to facilitate algorithm transitions as CCCS guidance evolves."
            .to_string(),
    );

    recommendations.push(
        "Maintain cryptographic inventory and ensure compliance with ITSP.40.111 and ITSP.40.062."
            .to_string(),
    );

    recommendations.push(format!(
        "Reference: ITSG-33 Annex 3A SC-13, ITSP.40.111 (Cryptographic Algorithms), ITSP.40.062 (Network Protocols), CMVP (Cryptographic Module Validation Program)."
    ));

    recommendations
}

/// Export ITSG-33 report to JSON
pub fn export_itsg33_json(report: &ITSG33Report) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

/// Export unified report to JSON
pub fn export_unified_json(report: &UnifiedComplianceReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
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
    fn test_generate_itsg33_report() {
        let audit_result = create_test_audit_result();
        let report =
            generate_itsg33_report(&audit_result, SecurityClassification::ProtectedA, Some("test.js"));

        assert_eq!(report.control_assessment.control_id, "ITSG-33 SC-13");
        assert_eq!(
            report.control_assessment.security_classification,
            SecurityClassification::ProtectedA
        );
        assert!(report.findings.len() >= 2); // RSA and MD5
    }

    #[test]
    fn test_generate_unified_report() {
        let audit_result = create_test_audit_result();
        let report = generate_unified_report(
            &audit_result,
            SecurityClassification::ProtectedB,
            Some("test.js"),
        );

        assert_eq!(report.nist_sc13_assessment.control_id, "sc-13");
        assert_eq!(report.itsg33_sc13_assessment.control_id, "ITSG-33 SC-13");
        assert!(!report.control_mapping.is_empty());
    }

    #[test]
    fn test_canadian_compliance_score() {
        let audit_result = create_test_audit_result();
        let summary = generate_canadian_summary(&audit_result, SecurityClassification::ProtectedA);

        assert!(!summary.cccs_prohibited_algorithms.is_empty());
        assert!(summary.compliance_score < 100);
        assert!(!summary.itsp_40_111_compliant); // Has prohibited algorithms
    }
}
