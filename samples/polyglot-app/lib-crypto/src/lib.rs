/**
 * Crypto Library (Rust)
 * WARNING: Mixed implementation with some intentional vulnerabilities
 * DO NOT use vulnerable parts in production
 */

use sha1::Sha1;
use sha2::{Sha256, Digest};
use md5::Md5;

/// Legacy hash function using MD5
/// VULNERABLE: MD5 is cryptographically broken
pub fn legacy_hash(data: &[u8]) -> String {
    // VULNERABLE: MD5 hash
    let mut hasher = Md5::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Legacy SHA-1 hash for backward compatibility
/// VULNERABLE: SHA-1 is deprecated
pub fn legacy_sha1(data: &[u8]) -> String {
    // VULNERABLE: SHA-1 hash
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Secure hash function using SHA-256
/// SECURE: This is the recommended approach
pub fn secure_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Verify integrity using MD5 (legacy)
/// VULNERABLE: MD5 verification
pub fn verify_legacy(data: &[u8], expected_hash: &str) -> bool {
    // VULNERABLE: MD5 comparison
    legacy_hash(data) == expected_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_hash() {
        let data = b"test data";
        let hash = legacy_hash(data);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_secure_hash() {
        let data = b"test data";
        let hash = secure_hash(data);
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
    }
}
