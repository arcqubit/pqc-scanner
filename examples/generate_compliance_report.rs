// Example: Generate NIST 800-53 SC-13 Compliance Report
//
// This example demonstrates how to:
// 1. Scan source code for cryptographic vulnerabilities
// 2. Generate a NIST 800-53 SC-13 compliance assessment report
// 3. Export the report as JSON
// 4. Generate OSCAL-compliant assessment results

use pqc_scanner::{
    analyze, export_oscal_json, export_sc13_json, generate_oscal_json, generate_sc13_report,
};

fn main() {
    // Sample source code with multiple cryptographic issues
    let source = r#"
// Vulnerable JavaScript cryptographic code
const crypto = require('crypto');

// CRITICAL: MD5 is cryptographically broken
const md5Hash = crypto.createHash('md5').update('data').digest('hex');

// HIGH: RSA with 1024-bit key is quantum-vulnerable
const rsaKey = crypto.generateKeyPairSync('rsa', {
    modulusLength: 1024,
});

// HIGH: ECDSA is quantum-vulnerable
const ecdsaKey = crypto.createSign('ecdsa-with-SHA256');

// CRITICAL: SHA-1 is cryptographically broken
const sha1Hash = crypto.createHash('sha1').update('data').digest('hex');

// DEPRECATED: 3DES should be replaced
const cipher = crypto.createCipheriv('des-ede3', key, iv);
"#;

    println!("=== PhotonIQ Quantum-Safe Crypto Auditor ===\n");

    // Step 1: Perform the audit
    println!("Step 1: Analyzing source code...");
    let audit_result = analyze(source, "javascript").expect("Failed to analyze source code");

    println!("  ✓ Scanned {} lines", audit_result.stats.lines_scanned);
    println!(
        "  ✓ Found {} vulnerabilities",
        audit_result.stats.total_vulnerabilities
    );
    println!("  ✓ Risk Score: {}/100\n", audit_result.risk_score);

    // Display vulnerabilities
    println!("Vulnerabilities Found:");
    for (i, vuln) in audit_result.vulnerabilities.iter().enumerate() {
        println!(
            "  {}. [{:?}] {} at line {}",
            i + 1,
            vuln.severity,
            vuln.crypto_type,
            vuln.line
        );
        println!("     Message: {}", vuln.message);
        println!("     Fix: {}", vuln.recommendation);
    }
    println!();

    // Step 2: Generate NIST 800-53 SC-13 Assessment Report
    println!("Step 2: Generating NIST 800-53 SC-13 Compliance Report...");
    let sc13_report = generate_sc13_report(&audit_result, Some("example.js"));

    println!("  ✓ Report ID: {}", sc13_report.metadata.report_id);
    println!(
        "  ✓ Control: {} - {}",
        sc13_report.control_assessment.control_id, sc13_report.control_assessment.control_name
    );
    println!(
        "  ✓ Implementation Status: {:?}",
        sc13_report.control_assessment.implementation_status
    );
    println!(
        "  ✓ Assessment Status: {:?}",
        sc13_report.control_assessment.assessment_status
    );
    println!(
        "  ✓ Compliance Score: {}/100",
        sc13_report.summary.compliance_score
    );
    println!("  ✓ Total Findings: {}\n", sc13_report.findings.len());

    // Display findings summary
    println!("SC-13 Findings:");
    for (i, finding) in sc13_report.findings.iter().enumerate() {
        println!(
            "  {}. [{:?}] {}",
            i + 1,
            finding.risk_level,
            finding.description.lines().next().unwrap_or("")
        );
        println!(
            "     Evidence: {} instances collected",
            finding.evidence.len()
        );
        println!("     Status: {:?}", finding.assessment_status);
    }
    println!();

    // Display recommendations
    println!("Compliance Recommendations:");
    for (i, rec) in sc13_report.recommendations.iter().enumerate() {
        println!("  {}. {}", i + 1, rec);
    }
    println!();

    // Step 3: Export SC-13 Report as JSON
    println!("Step 3: Exporting SC-13 Report as JSON...");
    let sc13_json = export_sc13_json(&sc13_report).expect("Failed to export SC-13 JSON");

    let output_file = "templates/sc13-compliance-report.json";
    std::fs::write(output_file, &sc13_json).expect("Failed to write SC-13 report");
    println!("  ✓ Saved to: {}", output_file);
    println!("  ✓ Size: {} bytes\n", sc13_json.len());

    // Step 4: Generate OSCAL Assessment Results
    println!("Step 4: Generating OSCAL Assessment Results...");
    let oscal = generate_oscal_json(&sc13_report, Some("example.js"));

    println!("  ✓ OSCAL Version: {}", oscal.oscal_version);
    println!("  ✓ Assessment UUID: {}", oscal.assessment_results.uuid);
    println!(
        "  ✓ Results: {} assessment result(s)",
        oscal.assessment_results.results.len()
    );

    if let Some(result) = oscal.assessment_results.results.first() {
        println!("  ✓ Observations: {}", result.observations.len());
        println!("  ✓ Findings: {}", result.findings.len());
    }
    println!();

    // Step 5: Export OSCAL as JSON
    println!("Step 5: Exporting OSCAL Report as JSON...");
    let oscal_json = export_oscal_json(&oscal).expect("Failed to export OSCAL JSON");

    let oscal_output = "templates/oscal-assessment-results.json";
    std::fs::write(oscal_output, &oscal_json).expect("Failed to write OSCAL report");
    println!("  ✓ Saved to: {}", oscal_output);
    println!("  ✓ Size: {} bytes\n", oscal_json.len());

    // Summary
    println!("=== Summary ===");
    println!(
        "✓ Generated NIST 800-53 SC-13 compliance report: {}",
        output_file
    );
    println!(
        "✓ Generated OSCAL 1.1.2 assessment results: {}",
        oscal_output
    );
    println!(
        "✓ Compliance Score: {}/100",
        sc13_report.summary.compliance_score
    );
    println!("✓ Risk Score: {}/100", audit_result.risk_score);

    if audit_result.stats.critical_count > 0 {
        println!(
            "\n⚠️  WARNING: {} critical vulnerabilities require immediate attention!",
            audit_result.stats.critical_count
        );
    }

    println!("\nReports are ready for compliance review and audit submission.");
}
