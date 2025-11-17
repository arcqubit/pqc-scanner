// Example: Canadian CCCS/CSE Cryptographic Compliance Assessment
// Demonstrates ITSG-33 SC-13, ITSP.40.111, and unified NIST+Canadian reporting

use pqc_scanner::{
    SecurityClassification, analyze, export_itsg33_json, export_unified_json,
    generate_itsg33_report, generate_unified_report,
};

fn main() {
    println!("=== Canadian Cryptographic Compliance Example ===\n");

    // Example code with various cryptographic vulnerabilities
    let source_code = r#"
// Node.js crypto examples
const crypto = require('crypto');

// MD5 - PROHIBITED by CCCS (ITSP.40.111 Section 5.3)
const md5Hash = crypto.createHash('md5').update('data').digest('hex');

// SHA-1 - PROHIBITED by CCCS (ITSP.40.111 Section 5.2)
const sha1Hash = crypto.createHash('sha1').update('data').digest('hex');

// RSA 2048-bit - CONDITIONALLY APPROVED for Protected A, but QUANTUM-VULNERABLE
const rsaKeys = crypto.generateKeyPairSync('rsa', {
    modulusLength: 2048,
    publicKeyEncoding: { type: 'spki', format: 'pem' },
    privateKeyEncoding: { type: 'pkcs8', format: 'pem' }
});

// ECDSA - CONDITIONALLY APPROVED, but QUANTUM-VULNERABLE
const ecKeys = crypto.generateKeyPairSync('ec', {
    namedCurve: 'secp256k1',
    publicKeyEncoding: { type: 'spki', format: 'pem' },
    privateKeyEncoding: { type: 'pkcs8', format: 'pem' }
});

// 3DES - DEPRECATED by CCCS (ITSP.40.111 Section 5.5)
const cipher = crypto.createCipheriv('des-ede3', key, iv);
"#;

    // Analyze the code
    println!("Analyzing code for cryptographic vulnerabilities...\n");
    let audit_result = match analyze(source_code, "javascript") {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error analyzing code: {}", e);
            return;
        }
    };

    println!("✓ Analysis complete");
    println!(
        "  Total vulnerabilities: {}",
        audit_result.stats.total_vulnerabilities
    );
    println!("  Risk score: {}/100\n", audit_result.risk_score);

    // ========================================
    // Example 1: ITSG-33 Report for Protected A
    // ========================================
    println!("=== Example 1: ITSG-33 SC-13 Assessment (Protected A) ===\n");

    let itsg33_report = generate_itsg33_report(
        &audit_result,
        SecurityClassification::ProtectedA,
        Some("example.js"),
    );

    println!("Control: {}", itsg33_report.control_assessment.control_id);
    println!(
        "Classification: {}",
        itsg33_report.control_assessment.security_classification
    );
    println!(
        "Implementation Status: {:?}",
        itsg33_report.control_assessment.implementation_status
    );
    println!(
        "Assessment Status: {:?}",
        itsg33_report.control_assessment.assessment_status
    );
    println!();

    println!("Compliance Summary:");
    println!(
        "  Compliance Score: {}/100",
        itsg33_report.summary.compliance_score
    );
    println!(
        "  ITSP.40.111 Compliant: {}",
        if itsg33_report.summary.itsp_40_111_compliant {
            "✓ YES"
        } else {
            "✗ NO"
        }
    );
    println!(
        "  Classification Compliant: {}",
        if itsg33_report.summary.classification_compliant {
            "✓ YES"
        } else {
            "✗ NO"
        }
    );
    println!();

    println!("CCCS Algorithm Status:");
    println!(
        "  Prohibited: {:?}",
        itsg33_report.summary.cccs_prohibited_algorithms
    );
    println!(
        "  Deprecated: {:?}",
        itsg33_report.summary.cccs_deprecated_algorithms
    );
    println!(
        "  Quantum-Vulnerable: {:?}",
        itsg33_report.summary.quantum_vulnerable_algorithms
    );
    println!();

    println!("CMVP Validation:");
    println!("  Required: {}", itsg33_report.summary.cmvp_required_count);
    println!(
        "  Validated: {}",
        itsg33_report.summary.cmvp_validated_count
    );
    println!();

    println!("Findings ({}):", itsg33_report.findings.len());
    for finding in &itsg33_report.findings {
        println!(
            "  - {} ({:?})",
            finding.finding_id, finding.cccs_approval_status
        );
        println!(
            "    ITSP References: {}",
            finding.itsp_references.join(", ")
        );
        println!(
            "    Applicable Classifications: {}",
            finding
                .applicable_classifications
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    println!();

    println!("Recommendations:");
    for (i, rec) in itsg33_report.recommendations.iter().enumerate() {
        println!("  {}. {}", i + 1, rec);
    }
    println!();

    // Export ITSG-33 report to JSON
    match export_itsg33_json(&itsg33_report) {
        Ok(json) => {
            println!("✓ ITSG-33 report exported to JSON ({} bytes)\n", json.len());
            // Optionally save to file
            // std::fs::write("itsg33_report.json", json).unwrap();
        }
        Err(e) => eprintln!("Error exporting ITSG-33 report: {}\n", e),
    }

    // ========================================
    // Example 2: Protected B Assessment
    // ========================================
    println!("=== Example 2: ITSG-33 SC-13 Assessment (Protected B) ===\n");

    let protected_b_report = generate_itsg33_report(
        &audit_result,
        SecurityClassification::ProtectedB,
        Some("example.js"),
    );

    println!(
        "Classification: {}",
        protected_b_report
            .control_assessment
            .security_classification
    );
    println!(
        "Compliance Score: {}/100",
        protected_b_report.summary.compliance_score
    );
    println!(
        "Classification Compliant: {}",
        if protected_b_report.summary.classification_compliant {
            "✓ YES"
        } else {
            "✗ NO"
        }
    );

    // For Protected B, RSA 2048-bit is insufficient (requires 3072-bit minimum)
    if !protected_b_report.summary.weak_key_sizes.is_empty() {
        println!(
            "\n⚠ Weak key sizes for Protected B: {:?}",
            protected_b_report.summary.weak_key_sizes
        );
    }
    println!();

    // ========================================
    // Example 3: Unified NIST + Canadian Report
    // ========================================
    println!("=== Example 3: Unified NIST + Canadian Compliance ===\n");

    let unified_report = generate_unified_report(
        &audit_result,
        SecurityClassification::ProtectedA,
        Some("example.js"),
    );

    println!("NIST 800-53 SC-13 Assessment:");
    println!(
        "  Control: {}",
        unified_report.nist_sc13_assessment.control_id
    );
    println!(
        "  Compliance Score: {}/100",
        unified_report.nist_summary.compliance_score
    );
    println!(
        "  Status: {:?}",
        unified_report.nist_sc13_assessment.assessment_status
    );
    println!();

    println!("ITSG-33 SC-13 Assessment:");
    println!(
        "  Control: {}",
        unified_report.itsg33_sc13_assessment.control_id
    );
    println!(
        "  Classification: {}",
        unified_report
            .itsg33_sc13_assessment
            .security_classification
    );
    println!(
        "  Compliance Score: {}/100",
        unified_report.canadian_summary.compliance_score
    );
    println!(
        "  Status: {:?}",
        unified_report.itsg33_sc13_assessment.assessment_status
    );
    println!();

    println!("Control Mapping:");
    for mapping in &unified_report.control_mapping {
        println!(
            "  {} ↔ {}",
            mapping.nist_control_id, mapping.itsg33_control_id
        );
        println!("  Equivalence: {}", mapping.equivalence);
        for note in &mapping.notes {
            println!("    • {}", note);
        }
    }
    println!();

    println!(
        "Unified Recommendations ({}):",
        unified_report.recommendations.len()
    );
    for (i, rec) in unified_report.recommendations.iter().enumerate().take(5) {
        println!("  {}. {}", i + 1, rec);
    }
    if unified_report.recommendations.len() > 5 {
        println!(
            "  ... and {} more",
            unified_report.recommendations.len() - 5
        );
    }
    println!();

    // Export unified report to JSON
    match export_unified_json(&unified_report) {
        Ok(json) => {
            println!("✓ Unified report exported to JSON ({} bytes)\n", json.len());
            // Optionally save to file
            // std::fs::write("unified_compliance_report.json", json).unwrap();
        }
        Err(e) => eprintln!("Error exporting unified report: {}\n", e),
    }

    // ========================================
    // Summary
    // ========================================
    println!("=== Summary ===\n");
    println!("This example demonstrated:");
    println!("  ✓ ITSG-33 SC-13 compliance assessment");
    println!("  ✓ Multi-classification support (Protected A, B, C)");
    println!("  ✓ CCCS algorithm approval status (ITSP.40.111)");
    println!("  ✓ CMVP validation requirements");
    println!("  ✓ Unified NIST + Canadian reporting");
    println!("  ✓ Control cross-mapping");
    println!("\nNext steps:");
    println!("  • Review ITSP.40.111 for algorithm migration guidance");
    println!("  • Ensure CMVP-validated cryptographic modules");
    println!("  • Plan post-quantum migration for quantum-vulnerable algorithms");
    println!("  • Implement crypto-agility for future algorithm transitions");
}
