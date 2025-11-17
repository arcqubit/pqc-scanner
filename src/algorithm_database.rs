// CCCS Algorithm Database
// Loads and queries CCCS-approved algorithms from ITSP.40.111

use crate::types::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Algorithm database entry from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmDatabaseEntry {
    pub algorithm: String,
    pub cccs_status: String,
    pub itsp_reference: String,
    pub approved_key_sizes: Vec<u32>,
    pub approved_modes: Vec<String>,
    pub cmvp_required: bool,
    pub conditions: Vec<String>,
    pub sunset_date: Option<String>,
    pub description: String,
}

/// Classification requirements from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationRequirements {
    pub minimum_aes_key_size: u32,
    pub minimum_rsa_key_size: u32,
    pub minimum_ecc_key_size: u32,
    pub approved_hash: Vec<String>,
    pub cmvp_required: bool,
}

/// Root database structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmDatabase {
    pub metadata: DatabaseMetadata,
    pub algorithms: HashMap<String, AlgorithmDatabaseEntry>,
    pub classification_requirements: HashMap<String, ClassificationRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    pub version: String,
    pub updated: String,
    pub source: String,
    pub description: String,
}

/// CMVP certificate from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMVPCertificateEntry {
    pub certificate_number: String,
    pub vendor: String,
    pub module_name: String,
    pub validation_level: String,
    pub algorithms: Vec<String>,
    pub expiry_date: Option<String>,
    pub status: String,
    pub description: String,
}

/// Library to CMVP mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMapping {
    pub library_name: String,
    pub common_packages: Vec<String>,
    pub cmvp_cert_numbers: Vec<String>,
    pub notes: String,
}

/// CMVP database structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMVPDatabase {
    pub metadata: DatabaseMetadata,
    pub certificates: Vec<CMVPCertificateEntry>,
    pub library_mappings: HashMap<String, LibraryMapping>,
}

// Load databases from embedded JSON
static ALGORITHM_DB: Lazy<AlgorithmDatabase> = Lazy::new(|| {
    let json_data = include_str!("../data/cccs_algorithms.json");
    serde_json::from_str(json_data).expect("Failed to parse CCCS algorithms database")
});

static CMVP_DB: Lazy<CMVPDatabase> = Lazy::new(|| {
    let json_data = include_str!("../data/cmvp_certificates.json");
    serde_json::from_str(json_data).expect("Failed to parse CMVP certificates database")
});

/// Get algorithm validation info
pub fn get_algorithm_validation(algorithm: &str) -> Option<AlgorithmValidation> {
    let entry = ALGORITHM_DB.algorithms.get(algorithm)?;

    let cccs_status = parse_cccs_status(&entry.cccs_status);

    Some(AlgorithmValidation {
        algorithm: entry.algorithm.clone(),
        cccs_status,
        itsp_reference: entry.itsp_reference.clone(),
        approved_key_sizes: entry.approved_key_sizes.clone(),
        approved_modes: entry.approved_modes.clone(),
        cmvp_required: entry.cmvp_required,
        conditions: entry.conditions.clone(),
        sunset_date: entry.sunset_date.clone(),
    })
}

/// Parse CCCS status string to enum
fn parse_cccs_status(status: &str) -> CCCSApprovalStatus {
    match status {
        "approved" => CCCSApprovalStatus::Approved,
        "conditionally-approved" => CCCSApprovalStatus::ConditionallyApproved,
        "deprecated" => CCCSApprovalStatus::Deprecated,
        "prohibited" => CCCSApprovalStatus::Prohibited,
        "under-review" => CCCSApprovalStatus::UnderReview,
        _ => CCCSApprovalStatus::UnderReview,
    }
}

/// Get CCCS approval status for a crypto type
pub fn get_cccs_status(crypto_type: &CryptoType) -> CCCSApprovalStatus {
    let algorithm_name = match crypto_type {
        CryptoType::Rsa => "RSA",
        CryptoType::Ecdsa => "ECDSA",
        CryptoType::Ecdh => "ECDH",
        CryptoType::Dsa => "DSA",
        CryptoType::DiffieHellman => "DH",
        CryptoType::Sha1 => "SHA-1",
        CryptoType::Md5 => "MD5",
        CryptoType::Des => "DES",
        CryptoType::TripleDes => "3DES",
        CryptoType::Rc4 => "RC4",
    };

    get_algorithm_validation(algorithm_name)
        .map(|v| v.cccs_status)
        .unwrap_or(CCCSApprovalStatus::UnderReview)
}

/// Check if algorithm is CCCS-approved
pub fn is_cccs_approved(crypto_type: &CryptoType) -> bool {
    matches!(get_cccs_status(crypto_type), CCCSApprovalStatus::Approved)
}

/// Check if algorithm is prohibited by CCCS
pub fn is_cccs_prohibited(crypto_type: &CryptoType) -> bool {
    matches!(get_cccs_status(crypto_type), CCCSApprovalStatus::Prohibited)
}

/// Check if algorithm is deprecated
pub fn is_cccs_deprecated(crypto_type: &CryptoType) -> bool {
    matches!(get_cccs_status(crypto_type), CCCSApprovalStatus::Deprecated)
}

/// Get classification requirements
pub fn get_classification_requirements(
    classification: SecurityClassification,
) -> Option<&'static ClassificationRequirements> {
    let key = match classification {
        SecurityClassification::Unclassified => "UNCLASSIFIED",
        SecurityClassification::ProtectedA => "PROTECTED_A",
        SecurityClassification::ProtectedB => "PROTECTED_B",
        SecurityClassification::ProtectedC => "PROTECTED_C",
    };

    ALGORITHM_DB.classification_requirements.get(key)
}

/// Validate key size against classification requirements
pub fn validate_key_size(
    crypto_type: &CryptoType,
    key_size: u32,
    classification: SecurityClassification,
) -> bool {
    let requirements = match get_classification_requirements(classification) {
        Some(req) => req,
        None => return false,
    };

    match crypto_type {
        CryptoType::Rsa => key_size >= requirements.minimum_rsa_key_size,
        CryptoType::Ecdsa | CryptoType::Ecdh => key_size >= requirements.minimum_ecc_key_size,
        _ => true, // Other types don't have size requirements in this context
    }
}

/// Check if CMVP validation is required for classification level
pub fn is_cmvp_required(classification: SecurityClassification) -> bool {
    get_classification_requirements(classification)
        .map(|req| req.cmvp_required)
        .unwrap_or(false)
}

/// Get CMVP certificate by number
pub fn get_cmvp_certificate(cert_number: &str) -> Option<CMVPCertificate> {
    let entry = CMVP_DB
        .certificates
        .iter()
        .find(|cert| cert.certificate_number == cert_number)?;

    let validation_level = match entry.validation_level.as_str() {
        "Level1" => FIPSLevel::Level1,
        "Level2" => FIPSLevel::Level2,
        "Level3" => FIPSLevel::Level3,
        "Level4" => FIPSLevel::Level4,
        _ => FIPSLevel::Level1,
    };

    let status = match entry.status.as_str() {
        "active" => CMVPStatus::Active,
        "historical" => CMVPStatus::Historical,
        "revoked" => CMVPStatus::Revoked,
        _ => CMVPStatus::Historical,
    };

    Some(CMVPCertificate {
        certificate_number: entry.certificate_number.clone(),
        vendor: entry.vendor.clone(),
        module_name: entry.module_name.clone(),
        validation_level,
        algorithms: entry.algorithms.clone(),
        expiry_date: entry.expiry_date.clone(),
        status,
    })
}

/// Find CMVP certificates for a library
pub fn find_cmvp_for_library(library_name: &str) -> Vec<CMVPCertificate> {
    // Try exact match first
    if let Some(mapping) = CMVP_DB.library_mappings.get(library_name) {
        return mapping
            .cmvp_cert_numbers
            .iter()
            .filter_map(|num| get_cmvp_certificate(num))
            .collect();
    }

    // Try partial match on common packages
    for (_, mapping) in CMVP_DB.library_mappings.iter() {
        if mapping
            .common_packages
            .iter()
            .any(|pkg| library_name.to_lowercase().contains(&pkg.to_lowercase()))
        {
            return mapping
                .cmvp_cert_numbers
                .iter()
                .filter_map(|num| get_cmvp_certificate(num))
                .collect();
        }
    }

    Vec::new()
}

/// Get all approved algorithms for a classification level
pub fn get_approved_algorithms(classification: SecurityClassification) -> Vec<String> {
    ALGORITHM_DB
        .algorithms
        .iter()
        .filter(|(_, entry)| entry.cccs_status == "approved")
        .filter(|(_, entry)| {
            // Check if algorithm meets classification requirements
            if let Some(reqs) = get_classification_requirements(classification) {
                // For algorithms with key sizes, verify they meet minimum requirements
                if entry.algorithm == "AES" {
                    return entry
                        .approved_key_sizes
                        .iter()
                        .any(|&size| size >= reqs.minimum_aes_key_size);
                }
                if entry.algorithm == "RSA" {
                    return entry
                        .approved_key_sizes
                        .iter()
                        .any(|&size| size >= reqs.minimum_rsa_key_size);
                }
                true
            } else {
                false
            }
        })
        .map(|(name, _)| name.clone())
        .collect()
}

/// Get all prohibited algorithms
pub fn get_prohibited_algorithms() -> Vec<String> {
    ALGORITHM_DB
        .algorithms
        .iter()
        .filter(|(_, entry)| entry.cccs_status == "prohibited")
        .map(|(name, _)| name.clone())
        .collect()
}

/// Get all deprecated algorithms
pub fn get_deprecated_algorithms() -> Vec<String> {
    ALGORITHM_DB
        .algorithms
        .iter()
        .filter(|(_, entry)| entry.cccs_status == "deprecated")
        .map(|(name, _)| name.clone())
        .collect()
}

/// Get ITSP reference for an algorithm
pub fn get_itsp_reference(crypto_type: &CryptoType) -> String {
    let algorithm_name = match crypto_type {
        CryptoType::Rsa => "RSA",
        CryptoType::Ecdsa => "ECDSA",
        CryptoType::Ecdh => "ECDH",
        CryptoType::Dsa => "DSA",
        CryptoType::DiffieHellman => "DH",
        CryptoType::Sha1 => "SHA-1",
        CryptoType::Md5 => "MD5",
        CryptoType::Des => "DES",
        CryptoType::TripleDes => "3DES",
        CryptoType::Rc4 => "RC4",
    };

    get_algorithm_validation(algorithm_name)
        .map(|v| v.itsp_reference)
        .unwrap_or_else(|| "ITSP.40.111".to_string())
}

/// Get conditions for conditionally approved algorithms
pub fn get_approval_conditions(crypto_type: &CryptoType) -> Vec<String> {
    let algorithm_name = match crypto_type {
        CryptoType::Rsa => "RSA",
        CryptoType::Ecdsa => "ECDSA",
        CryptoType::Ecdh => "ECDH",
        CryptoType::DiffieHellman => "DH",
        _ => return Vec::new(),
    };

    get_algorithm_validation(algorithm_name)
        .map(|v| v.conditions)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_databases() {
        // Verify databases load successfully
        assert!(!ALGORITHM_DB.algorithms.is_empty());
        assert!(!CMVP_DB.certificates.is_empty());
    }

    #[test]
    fn test_get_algorithm_validation() {
        let aes = get_algorithm_validation("AES");
        assert!(aes.is_some());
        let aes = aes.unwrap();
        assert_eq!(aes.cccs_status, CCCSApprovalStatus::Approved);
        assert!(aes.cmvp_required);

        let md5 = get_algorithm_validation("MD5");
        assert!(md5.is_some());
        let md5 = md5.unwrap();
        assert_eq!(md5.cccs_status, CCCSApprovalStatus::Prohibited);
    }

    #[test]
    fn test_cccs_status() {
        assert!(is_cccs_prohibited(&CryptoType::Md5));
        assert!(is_cccs_prohibited(&CryptoType::Sha1));
        assert!(is_cccs_prohibited(&CryptoType::Des));
        assert!(is_cccs_deprecated(&CryptoType::TripleDes));
    }

    #[test]
    fn test_classification_requirements() {
        let unclass = get_classification_requirements(SecurityClassification::Unclassified);
        assert!(unclass.is_some());
        assert_eq!(unclass.unwrap().minimum_aes_key_size, 128);

        let protected_c = get_classification_requirements(SecurityClassification::ProtectedC);
        assert!(protected_c.is_some());
        assert_eq!(protected_c.unwrap().minimum_aes_key_size, 256);
        assert!(protected_c.unwrap().cmvp_required);
    }

    #[test]
    fn test_validate_key_size() {
        assert!(validate_key_size(
            &CryptoType::Rsa,
            2048,
            SecurityClassification::ProtectedA
        ));
        assert!(!validate_key_size(
            &CryptoType::Rsa,
            1024,
            SecurityClassification::ProtectedA
        ));
        assert!(validate_key_size(
            &CryptoType::Rsa,
            4096,
            SecurityClassification::ProtectedC
        ));
    }

    #[test]
    fn test_cmvp_required() {
        assert!(!is_cmvp_required(SecurityClassification::Unclassified));
        assert!(is_cmvp_required(SecurityClassification::ProtectedA));
        assert!(is_cmvp_required(SecurityClassification::ProtectedB));
    }

    #[test]
    fn test_get_cmvp_certificate() {
        let cert = get_cmvp_certificate("4282");
        assert!(cert.is_some());
        let cert = cert.unwrap();
        assert_eq!(cert.vendor, "OpenSSL Software Foundation");
        assert!(cert.algorithms.contains(&"AES".to_string()));
    }

    #[test]
    fn test_find_cmvp_for_library() {
        let openssl_certs = find_cmvp_for_library("openssl");
        assert!(!openssl_certs.is_empty());
        assert!(openssl_certs[0].vendor.contains("OpenSSL"));
    }

    #[test]
    fn test_get_approved_algorithms() {
        let approved = get_approved_algorithms(SecurityClassification::ProtectedA);
        assert!(!approved.is_empty());
        assert!(approved.contains(&"AES".to_string()));
    }

    #[test]
    fn test_get_prohibited_algorithms() {
        let prohibited = get_prohibited_algorithms();
        assert!(prohibited.contains(&"MD5".to_string()));
        assert!(prohibited.contains(&"SHA-1".to_string()));
        assert!(prohibited.contains(&"DES".to_string()));
    }
}
