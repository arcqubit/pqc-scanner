// Comprehensive error handling tests for critical fixes
// Tests input validation, empty collection handling, and path validation

use pqc_scanner::{AuditError, ParseError, analyze, generate_remediations, parse_file};

#[test]
fn test_empty_source_rejection() {
    let result = analyze("", "rust");
    assert!(matches!(result, Err(AuditError::InvalidSource)));

    let result = analyze("   \n  \t  ", "rust");
    assert!(matches!(result, Err(AuditError::InvalidSource)));
}

#[test]
fn test_oversized_source_rejection() {
    // Create source larger than 10MB
    let huge_source = "x".repeat(11 * 1024 * 1024);
    let result = analyze(&huge_source, "rust");
    assert!(matches!(result, Err(AuditError::SourceTooLarge(_, _))));

    // Extract actual values for assertion
    if let Err(AuditError::SourceTooLarge(size, max)) = result {
        assert!(size > max);
        assert_eq!(max, 10 * 1024 * 1024);
    }
}

#[test]
fn test_too_many_lines_rejection() {
    // Create source with over 500k lines
    let many_lines = "line\n".repeat(600_000);
    let result = analyze(&many_lines, "rust");
    assert!(matches!(result, Err(AuditError::TooManyLines(_, _))));

    if let Err(AuditError::TooManyLines(count, max)) = result {
        assert!(count > max);
        assert_eq!(max, 500_000);
    }
}

#[test]
fn test_valid_source_accepted() {
    let valid_source = r#"
        use crypto::md5::Md5;

        fn hash_password(pass: &str) -> String {
            let mut hasher = Md5::new();
            hasher.update(pass);
            hasher.finish()
        }
    "#;

    let result = analyze(valid_source, "rust");
    assert!(result.is_ok());
}

#[test]
fn test_parser_empty_source_rejection() {
    let result = parse_file("", "rust");
    assert!(matches!(result, Err(ParseError::InvalidSource)));
}

#[test]
fn test_parser_oversized_source_rejection() {
    // Parser has a 5MB limit
    let huge_source = "x".repeat(6 * 1024 * 1024);
    let result = parse_file(&huge_source, "rust");
    assert!(matches!(result, Err(ParseError::SourceTooLarge(_, _))));

    if let Err(ParseError::SourceTooLarge(size, max)) = result {
        assert!(size > max);
        assert_eq!(max, 5 * 1024 * 1024);
    }
}

#[test]
fn test_path_traversal_rejection() {
    use pqc_scanner::{AuditResult, Language};

    let mut audit_result = AuditResult::new(Language::Rust, 10);

    // Test various path traversal attempts
    let bad_paths = vec![
        "../../../etc/passwd",
        "../../sensitive/data",
        "path/../../../root",
    ];

    for bad_path in bad_paths {
        let result = generate_remediations(&audit_result, bad_path);
        assert!(!result.warnings.is_empty());
        assert!(result.warnings[0].contains("Path traversal"));
    }
}

#[test]
fn test_null_byte_path_rejection() {
    use pqc_scanner::{AuditResult, Language};

    let audit_result = AuditResult::new(Language::Rust, 10);
    let bad_path = "path/with/\0/null";

    let result = generate_remediations(&audit_result, bad_path);
    assert!(!result.warnings.is_empty());
    assert!(result.warnings[0].contains("null byte"));
}

#[test]
fn test_empty_path_rejection() {
    use pqc_scanner::{AuditResult, Language};

    let audit_result = AuditResult::new(Language::Rust, 10);

    let result = generate_remediations(&audit_result, "");
    assert!(!result.warnings.is_empty());
    assert!(result.warnings[0].contains("Empty file path"));
}

#[test]
fn test_oversized_path_rejection() {
    use pqc_scanner::{AuditResult, Language};

    let audit_result = AuditResult::new(Language::Rust, 10);
    let long_path = "a".repeat(5000);

    let result = generate_remediations(&audit_result, &long_path);
    assert!(!result.warnings.is_empty());
    assert!(result.warnings[0].contains("too long"));
}

#[test]
fn test_valid_path_accepted() {
    use pqc_scanner::{AuditResult, Language};

    let audit_result = AuditResult::new(Language::Rust, 10);
    let valid_path = "src/main.rs";

    let result = generate_remediations(&audit_result, valid_path);
    // Should not have path validation warnings
    let has_path_warning = result
        .warnings
        .iter()
        .any(|w| w.contains("Invalid file path"));
    assert!(!has_path_warning);
}

#[test]
fn test_unsupported_language() {
    let result = analyze("some code", "invalid_lang");
    assert!(matches!(result, Err(AuditError::UnsupportedLanguage(_))));
}

#[test]
fn test_parser_unsupported_language() {
    let result = parse_file("some code", "invalid_lang");
    assert!(matches!(result, Err(ParseError::UnsupportedLanguage(_))));
}

#[test]
fn test_edge_case_max_size() {
    // Test exactly at the limit (should pass)
    let max_source = "x".repeat(10 * 1024 * 1024);
    let result = analyze(&max_source, "rust");
    // At exactly max size, should be accepted
    assert!(result.is_ok());
}

#[test]
fn test_edge_case_max_lines() {
    // Test exactly at the limit (should pass)
    let max_lines = "line\n".repeat(500_000);
    let result = analyze(&max_lines, "rust");
    // At exactly max lines, should be accepted
    assert!(result.is_ok());
}

#[test]
fn test_unicode_in_source() {
    let unicode_source = r#"
        // This comment has unicode: 你好世界
        use crypto::sha256::Sha256;

        fn test() {
            let msg = "Hello 世界";
        }
    "#;

    let result = analyze(unicode_source, "rust");
    assert!(result.is_ok());
}

#[test]
fn test_multiline_code() {
    let multiline = r#"
        fn complex_function() {
            let rsa_key = generate_rsa_key(2048);

            for i in 0..100 {
                process(i);
            }

            match result {
                Ok(v) => v,
                Err(e) => panic!("error"),
            }
        }
    "#;

    let result = analyze(multiline, "rust");
    assert!(result.is_ok());
}
