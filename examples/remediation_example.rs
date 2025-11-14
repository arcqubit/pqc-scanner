// Example demonstrating the auto-remediation feature

use pqc_scanner::{analyze, generate_remediations};

fn main() {
    println!("=== Cryptographic Auto-Remediation Example ===\n");

    // Example 1: MD5 Hash Remediation
    println!("Example 1: MD5 Hash Vulnerability");
    println!("-----------------------------------");
    let md5_source = r#"
import hashlib

def hash_password(password):
    return hashlib.md5(password.encode()).hexdigest()
"#;

    let audit = analyze(md5_source, "python").unwrap();
    let remediation = generate_remediations(&audit, "password.py");

    println!("Source code:\n{}", md5_source);
    println!("Vulnerabilities found: {}", audit.vulnerabilities.len());

    for fix in &remediation.fixes {
        println!("\nRemediation:");
        println!("  Algorithm: {}", fix.algorithm);
        println!("  Line: {}, Column: {}", fix.line, fix.column);
        println!("  Old code: {}", fix.old_code);
        println!("  New code: {}", fix.new_code);
        println!("  Confidence: {:.1}%", fix.confidence * 100.0);
        println!("  Auto-applicable: {}", fix.auto_applicable);
        println!("  Explanation: {}", fix.explanation);
    }

    // Example 2: Multiple Vulnerabilities
    println!("\n\nExample 2: Multiple Vulnerabilities");
    println!("-----------------------------------");
    let multi_source = r#"
import hashlib
from Crypto.PublicKey import RSA
from Crypto.Cipher import DES

def process_data(data, key):
    # Weak hashing
    hash1 = hashlib.md5(data).hexdigest()
    hash2 = hashlib.sha1(data).hexdigest()

    # Quantum-vulnerable key generation
    rsa_key = RSA.generate(1024)

    # Deprecated cipher
    cipher = DES.new(key, DES.MODE_ECB)

    return hash1, hash2, rsa_key, cipher
"#;

    let audit = analyze(multi_source, "python").unwrap();
    let remediation = generate_remediations(&audit, "crypto_utils.py");

    println!("Vulnerabilities found: {}", audit.vulnerabilities.len());
    println!("\nRemediation Summary:");
    println!(
        "  Total vulnerabilities: {}",
        remediation.summary.total_vulnerabilities
    );
    println!("  Auto-fixable: {}", remediation.summary.auto_fixable);
    println!(
        "  Manual review required: {}",
        remediation.summary.manual_review_required
    );
    println!(
        "  Average confidence: {:.1}%",
        remediation.summary.average_confidence * 100.0
    );

    println!("\nDetailed Fixes:");
    for (i, fix) in remediation.fixes.iter().enumerate() {
        println!("\nFix #{}", i + 1);
        println!("  Type: {}", fix.algorithm);
        println!("  Line {}: {}", fix.line, fix.old_code);
        println!("  Suggested: {}", fix.new_code);
        println!("  Auto-applicable: {}", fix.auto_applicable);
        if !fix.auto_applicable {
            println!("  ⚠ Manual review required!");
        }
    }

    if !remediation.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &remediation.warnings {
            println!("  ⚠ {}", warning);
        }
    }

    // Example 3: SHA-1 in JavaScript
    println!("\n\nExample 3: SHA-1 in JavaScript");
    println!("-----------------------------------");
    let js_source = r#"
const crypto = require('crypto');

function hashData(data) {
    return crypto.createHash('sha1').update(data).digest('hex');
}
"#;

    let audit = analyze(js_source, "javascript").unwrap();
    let remediation = generate_remediations(&audit, "utils.js");

    for fix in &remediation.fixes {
        println!("Original: {}", fix.old_code);
        println!("Fixed:    {}", fix.new_code);
        println!("Confidence: {:.0}%", fix.confidence * 100.0);
    }

    // Example 4: RSA Weak Key
    println!("\n\nExample 4: RSA Weak Key Size");
    println!("-----------------------------------");
    let rsa_source = r#"
from Crypto.PublicKey import RSA

# Generate weak 1024-bit RSA key
key = RSA.generate(1024)
"#;

    let audit = analyze(rsa_source, "python").unwrap();
    let remediation = generate_remediations(&audit, "keygen.py");

    for fix in &remediation.fixes {
        println!("Issue: {}", fix.algorithm);
        println!("Current: {}", fix.old_code);
        println!("Interim fix: {}", fix.new_code);
        println!("Requires manual review: {}", !fix.auto_applicable);
        println!("\nGuidance:");
        println!("  {}", fix.explanation);
    }

    // Example 5: DES/3DES Remediation
    println!("\n\nExample 5: DES/3DES Cipher");
    println!("-----------------------------------");
    let des_source = r#"
from Crypto.Cipher import DES

cipher = DES.new(key, DES.MODE_ECB)
ciphertext = cipher.encrypt(plaintext)
"#;

    let audit = analyze(des_source, "python").unwrap();
    let remediation = generate_remediations(&audit, "cipher.py");

    for fix in &remediation.fixes {
        println!("Migration: {}", fix.algorithm);
        println!("From: {}", fix.old_code);
        println!("To:   {}", fix.new_code);
        println!("\nNotes:");
        println!("  {}", fix.explanation);
    }

    // Example 6: JSON Export
    println!("\n\nExample 6: JSON Export");
    println!("-----------------------------------");
    let audit = analyze(md5_source, "python").unwrap();
    let remediation = generate_remediations(&audit, "password.py");

    let json = serde_json::to_string_pretty(&remediation).unwrap();
    println!("Remediation JSON:");
    println!("{}", json);

    println!("\n=== End of Examples ===");
}
