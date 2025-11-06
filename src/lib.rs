// PhotonIQ Quantum-Safe Crypto Audit Library
// Core Rust implementation for detecting quantum-vulnerable cryptography

pub mod audit;
pub mod detector;
pub mod parser;
pub mod types;

// Re-export public API
pub use audit::{analyze, score_vulnerability, AuditError};
pub use types::{
    AuditResult, AuditStats, CryptoType, Language, Severity, Vulnerability,
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
    let result = audit::analyze(source, language)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
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
