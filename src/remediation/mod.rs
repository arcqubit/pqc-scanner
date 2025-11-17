//! Auto-remediation module for cryptographic vulnerabilities
//!
//! This module provides intelligent code transformation suggestions for quantum-vulnerable
//! cryptographic algorithms. All fixes require manual review before application.
//!
//! ## Safety-First Approach
//!
//! **IMPORTANT**: This module NEVER automatically applies code changes. It generates:
//! - Remediation reports with detailed analysis
//! - Patch files (when `--auto-remediate=true` flag is used)
//! - Review checklists for each suggested fix
//!
//! ## Usage
//!
//! ```rust
//! use pqc_scanner::{analyze, generate_remediations};
//!
//! let source = r#"
//!     import hashlib
//!     hash = hashlib.md5(data).hexdigest()
//! "#;
//!
//! let audit_result = analyze(source, "python").unwrap();
//! let remediation = generate_remediations(&audit_result, "crypto.py");
//!
//! // Review the suggestions
//! for fix in &remediation.fixes {
//!     println!("Fix: {}", fix.algorithm);
//!     println!("Confidence: {:.0}%", fix.confidence * 100.0);
//!     println!("Patch available: {}", fix.patch_available);
//!
//!     // Always review these before applying!
//!     for note in &fix.review_notes {
//!         println!("  - {}", note);
//!     }
//! }
//! ```
//!
//! ## Module Structure
//!
//! - `core`: Core remediation logic and fix generation
//! - `templates`: Language-specific remediation templates (TODO: Phase 2)
//! - `patterns`: Advanced pattern matching engine (TODO: Phase 2)
//!
//! ## Supported Remediations
//!
//! ### ✅ Complete Algorithm Coverage (10/10 crypto types)
//!
//! #### Hash Functions
//! - **MD5** → SHA-256 (85% confidence, patch available)
//! - **SHA-1** → SHA-256 (90% confidence, patch available)
//!
//! #### Symmetric Ciphers
//! - **DES** → AES-256-GCM (75% confidence, patch available)
//! - **3DES** → AES-256-GCM (75% confidence, patch available)
//! - **RC4** → AES-256-GCM or ChaCha20-Poly1305 (80% confidence, patch available)
//!
//! #### Asymmetric Cryptography
//! - **RSA weak keys** → RSA-2048 interim (70% confidence, patch available)
//! - **RSA strong keys** → CRYSTALS-Dilithium guidance (50% confidence, no patch)
//! - **DSA** → Ed25519 or CRYSTALS-Dilithium (70% confidence, patch available)
//! - **ECDSA** → Ed25519 or CRYSTALS-Dilithium (65% confidence, patch available)
//!
//! #### Key Exchange
//! - **Diffie-Hellman** → ECDH/X25519 or CRYSTALS-Kyber (65% confidence, patch available)
//! - **ECDH** → X25519 or CRYSTALS-Kyber (65% confidence, patch available)

// Re-export public API from core module
pub use core::{CodeFix, RemediationResult, RemediationSummary, generate_remediations};

mod core;

// Future modules (Phase 2)
// mod templates;
// mod patterns;
// mod pqc_migration;
// mod compliance_integration;
