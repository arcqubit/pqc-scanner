// PQC Scanner - Quantum-Safe Crypto Audit Library
// Core Rust implementation for detecting quantum-vulnerable cryptography

pub mod algorithm_database;
pub mod audit;
pub mod canadian_compliance;
pub mod compliance;
pub mod detector;
pub mod parser;
pub mod remediation;
pub mod types;

// Re-export public API
pub use audit::{AuditError, analyze, score_vulnerability};
pub use canadian_compliance::{
    export_itsg33_json, export_unified_json, generate_itsg33_report, generate_unified_report,
};
pub use compliance::{
    export_oscal_json, export_sc13_json, generate_oscal_json, generate_sc13_report,
};
pub use remediation::{CodeFix, RemediationResult, RemediationSummary, generate_remediations};
pub use types::{
    AuditResult, AuditStats, CryptoType, ITSG33Report, Language, OscalAssessmentResults,
    SC13AssessmentReport, SecurityClassification, Severity, UnifiedComplianceReport, Vulnerability,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// WASM initialization
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// WASM-compatible audit function
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn audit_code(source: &str, language: &str) -> Result<JsValue, JsValue> {
    let result = audit::analyze(source, language).map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

// WASM-compatible SC-13 compliance report generation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_compliance_report(
    source: &str,
    language: &str,
    file_path: Option<String>,
) -> Result<JsValue, JsValue> {
    let audit_result =
        audit::analyze(source, language).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let report = compliance::generate_sc13_report(&audit_result, file_path.as_deref());

    serde_wasm_bindgen::to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

// WASM-compatible OSCAL JSON generation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_oscal_report(
    source: &str,
    language: &str,
    file_path: Option<String>,
) -> Result<JsValue, JsValue> {
    let audit_result =
        audit::analyze(source, language).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let sc13_report = compliance::generate_sc13_report(&audit_result, file_path.as_deref());

    let oscal = compliance::generate_oscal_json(&sc13_report, file_path.as_deref());

    serde_wasm_bindgen::to_value(&oscal).map_err(|e| JsValue::from_str(&e.to_string()))
}

// WASM-compatible remediation generation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_remediation(
    source: &str,
    language: &str,
    file_path: &str,
) -> Result<JsValue, JsValue> {
    let audit_result =
        audit::analyze(source, language).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let remediation = remediation::generate_remediations(&audit_result, file_path);

    serde_wasm_bindgen::to_value(&remediation).map_err(|e| JsValue::from_str(&e.to_string()))
}

// WASM-compatible ITSG-33 Canadian compliance report generation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_itsg33_compliance_report(
    source: &str,
    language: &str,
    classification: &str,
    file_path: Option<String>,
) -> Result<JsValue, JsValue> {
    let audit_result =
        audit::analyze(source, language).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let security_classification = match classification.to_lowercase().as_str() {
        "unclassified" => SecurityClassification::Unclassified,
        "protected-a" | "protected_a" => SecurityClassification::ProtectedA,
        "protected-b" | "protected_b" => SecurityClassification::ProtectedB,
        "protected-c" | "protected_c" => SecurityClassification::ProtectedC,
        _ => SecurityClassification::ProtectedA, // Default to Protected A
    };

    let report = canadian_compliance::generate_itsg33_report(
        &audit_result,
        security_classification,
        file_path.as_deref(),
    );

    serde_wasm_bindgen::to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

// WASM-compatible unified NIST + Canadian compliance report generation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_unified_compliance_report(
    source: &str,
    language: &str,
    classification: &str,
    file_path: Option<String>,
) -> Result<JsValue, JsValue> {
    let audit_result =
        audit::analyze(source, language).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let security_classification = match classification.to_lowercase().as_str() {
        "unclassified" => SecurityClassification::Unclassified,
        "protected-a" | "protected_a" => SecurityClassification::ProtectedA,
        "protected-b" | "protected_b" => SecurityClassification::ProtectedB,
        "protected-c" | "protected_c" => SecurityClassification::ProtectedC,
        _ => SecurityClassification::ProtectedA,
    };

    let report = canadian_compliance::generate_unified_report(
        &audit_result,
        security_classification,
        file_path.as_deref(),
    );

    serde_wasm_bindgen::to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api() {
        let source = "const rsa = crypto.createPublicKey('rsa');";
        let result = analyze(source, "javascript").unwrap();
        assert!(!result.vulnerabilities.is_empty());
    }
}
