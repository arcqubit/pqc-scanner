//! Multi-language source code parser for crypto pattern detection
//!
//! Supports: Rust, JavaScript, TypeScript, Python, Java, Go

use crate::types::*;
use lazy_static::lazy_static;
use regex::Regex;

// Lazy-compiled regex patterns for parsing
lazy_static! {
    // Rust patterns
    static ref RUST_USE_RE: Regex = Regex::new(r"^\s*use\s+([^;]+);")
        .expect("RUST_USE_RE: Invalid regex pattern - this is a compile-time bug");
    static ref RUST_FN_CALL_RE: Regex = Regex::new(r"(\w+(?:::\w+)?)\s*\(")
        .expect("RUST_FN_CALL_RE: Invalid regex pattern - this is a compile-time bug");
    static ref RUST_STRUCT_RE: Regex = Regex::new(r"^\s*(?:pub\s+)?struct\s+(\w+)")
        .expect("RUST_STRUCT_RE: Invalid regex pattern - this is a compile-time bug");

    // JavaScript/TypeScript patterns
    static ref JS_IMPORT_RE: Regex = Regex::new(r#"^\s*import\s+(?:.*?from\s+)?['"]([^'"]+)['"]"#)
        .expect("JS_IMPORT_RE: Invalid regex pattern - this is a compile-time bug");
    static ref JS_REQUIRE_RE: Regex = Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#)
        .expect("JS_REQUIRE_RE: Invalid regex pattern - this is a compile-time bug");
    static ref JS_FN_CALL_RE: Regex = Regex::new(r"(\w+)\s*\(")
        .expect("JS_FN_CALL_RE: Invalid regex pattern - this is a compile-time bug");
    static ref JS_CLASS_RE: Regex = Regex::new(r"^\s*class\s+(\w+)")
        .expect("JS_CLASS_RE: Invalid regex pattern - this is a compile-time bug");
    static ref JS_FUNCTION_RE: Regex = Regex::new(r"^\s*(?:async\s+)?function\s+(\w+)")
        .expect("JS_FUNCTION_RE: Invalid regex pattern - this is a compile-time bug");

    // Python patterns
    static ref PY_IMPORT_RE: Regex = Regex::new(r"^\s*(?:import|from)\s+([\w.]+)")
        .expect("PY_IMPORT_RE: Invalid regex pattern - this is a compile-time bug");
    static ref PY_FN_CALL_RE: Regex = Regex::new(r"(\w+)\s*\(")
        .expect("PY_FN_CALL_RE: Invalid regex pattern - this is a compile-time bug");
    static ref PY_CLASS_RE: Regex = Regex::new(r"^\s*class\s+(\w+)")
        .expect("PY_CLASS_RE: Invalid regex pattern - this is a compile-time bug");
    static ref PY_FUNCTION_RE: Regex = Regex::new(r"^\s*def\s+(\w+)")
        .expect("PY_FUNCTION_RE: Invalid regex pattern - this is a compile-time bug");

    // Java patterns
    static ref JAVA_IMPORT_RE: Regex = Regex::new(r"^\s*import\s+([\w.]+);")
        .expect("JAVA_IMPORT_RE: Invalid regex pattern - this is a compile-time bug");
    static ref JAVA_FN_CALL_RE: Regex = Regex::new(r"(\w+)\s*\(")
        .expect("JAVA_FN_CALL_RE: Invalid regex pattern - this is a compile-time bug");
    static ref JAVA_CLASS_RE: Regex = Regex::new(r"^\s*(?:public\s+)?class\s+(\w+)")
        .expect("JAVA_CLASS_RE: Invalid regex pattern - this is a compile-time bug");

    // Go patterns
    static ref GO_IMPORT_RE: Regex = Regex::new(r#"^\s*import\s+(?:\(|"([^"]+)")"#)
        .expect("GO_IMPORT_RE: Invalid regex pattern - this is a compile-time bug");
    static ref GO_FN_CALL_RE: Regex = Regex::new(r"(\w+)\s*\(")
        .expect("GO_FN_CALL_RE: Invalid regex pattern - this is a compile-time bug");
    static ref GO_STRUCT_RE: Regex = Regex::new(r"^\s*type\s+(\w+)\s+struct")
        .expect("GO_STRUCT_RE: Invalid regex pattern - this is a compile-time bug");
    static ref GO_FUNCTION_RE: Regex = Regex::new(r"^\s*func\s+(\w+)")
        .expect("GO_FUNCTION_RE: Invalid regex pattern - this is a compile-time bug");
}

/// Parser errors
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Parse error: {0}")]
    ParseFailure(String),

    #[error("Invalid source code")]
    InvalidSource,

    #[error("Source code too large for parsing: {0} bytes (max: {1})")]
    SourceTooLarge(usize, usize),
}

// Input validation constants
const MAX_PARSE_SIZE: usize = 5 * 1024 * 1024; // 5MB (smaller than audit since parsing is more expensive)

/// Main parsing function - dispatches to language-specific parsers
pub fn parse_file(source: &str, language: &str) -> Result<ParsedSource, ParseError> {
    // Validate language
    let lang = Language::from_string(language)
        .ok_or_else(|| ParseError::UnsupportedLanguage(language.to_string()))?;

    // Validate source size
    if source.is_empty() {
        return Err(ParseError::InvalidSource);
    }

    let source_size = source.len();
    if source_size > MAX_PARSE_SIZE {
        return Err(ParseError::SourceTooLarge(source_size, MAX_PARSE_SIZE));
    }

    match lang {
        Language::Rust => parse_rust(source),
        Language::JavaScript => parse_javascript(source),
        Language::TypeScript => parse_typescript(source),
        Language::Python => parse_python(source),
        Language::Java => parse_java(source),
        Language::Go => parse_go(source),
        _ => Err(ParseError::UnsupportedLanguage(language.to_string())),
    }
}

/// Parse Rust source code
fn parse_rust(source: &str) -> Result<ParsedSource, ParseError> {
    let mut parsed = ParsedSource::new(Language::Rust);

    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if let Some(caps) = RUST_USE_RE.captures(trimmed) {
            if let Some(import_match) = caps.get(1) {
                let import = import_match.as_str().trim().to_string();
                parsed.imports.push(import.clone());
                parsed.ast_nodes.push(AstNode {
                    node_type: NodeType::Import,
                    line: line_num,
                    column: 0,
                    content: import,
                });
            }
        }

        if let Some(caps) = RUST_STRUCT_RE.captures(trimmed) {
            if let Some(struct_match) = caps.get(1) {
                parsed.ast_nodes.push(AstNode {
                    node_type: NodeType::ClassDeclaration,
                    line: line_num,
                    column: 0,
                    content: struct_match.as_str().to_string(),
                });
            }
        }

        for caps in RUST_FN_CALL_RE.captures_iter(trimmed) {
            if let Some(fn_match) = caps.get(1) {
                let fn_name = fn_match.as_str().to_string();
                let column = line.find(&fn_name).unwrap_or(0);
                parsed.function_calls.push(FunctionCall {
                    name: fn_name.clone(),
                    line: line_num,
                    column,
                    args: vec![],
                });
            }
        }
    }

    Ok(parsed)
}

/// Parse JavaScript source code
fn parse_javascript(source: &str) -> Result<ParsedSource, ParseError> {
    let mut parsed = ParsedSource::new(Language::JavaScript);

    let import_re = Regex::new(r#"^\s*import\s+.*from\s+['"]([^'"]+)['"]"#).unwrap();
    let require_re = Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#).unwrap();
    let fn_call_re = Regex::new(r"(\w+(?:\.\w+)?)\s*\(").unwrap();

    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if let Some(caps) = import_re.captures(trimmed) {
            let import = caps.get(1).unwrap().as_str().to_string();
            parsed.imports.push(import.clone());
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::Import,
                line: line_num,
                column: 0,
                content: import,
            });
        }

        for caps in require_re.captures_iter(trimmed) {
            let import = caps.get(1).unwrap().as_str().to_string();
            parsed.imports.push(import.clone());
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::Import,
                line: line_num,
                column: 0,
                content: import,
            });
        }

        for caps in fn_call_re.captures_iter(trimmed) {
            let fn_name = caps.get(1).unwrap().as_str().to_string();
            let column = line.find(&fn_name).unwrap_or(0);
            parsed.function_calls.push(FunctionCall {
                name: fn_name.clone(),
                line: line_num,
                column,
                args: vec![],
            });
        }
    }

    Ok(parsed)
}

fn parse_typescript(source: &str) -> Result<ParsedSource, ParseError> {
    let mut result = parse_javascript(source)?;
    result.language = Language::TypeScript;
    Ok(result)
}

fn parse_python(source: &str) -> Result<ParsedSource, ParseError> {
    let mut parsed = ParsedSource::new(Language::Python);

    let import_re = Regex::new(r"^\s*import\s+(.+)").unwrap();
    let from_import_re = Regex::new(r"^\s*from\s+([^\s]+)\s+import").unwrap();
    let fn_call_re = Regex::new(r"(\w+(?:\.\w+)?)\s*\(").unwrap();

    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some(caps) = from_import_re.captures(trimmed) {
            let import = caps.get(1).unwrap().as_str().to_string();
            parsed.imports.push(import.clone());
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::Import,
                line: line_num,
                column: 0,
                content: import,
            });
        } else if let Some(caps) = import_re.captures(trimmed) {
            let import = caps.get(1).unwrap().as_str().trim().to_string();
            parsed.imports.push(import.clone());
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::Import,
                line: line_num,
                column: 0,
                content: import,
            });
        }

        for caps in fn_call_re.captures_iter(trimmed) {
            let fn_name = caps.get(1).unwrap().as_str().to_string();
            let column = line.find(&fn_name).unwrap_or(0);
            parsed.function_calls.push(FunctionCall {
                name: fn_name.clone(),
                line: line_num,
                column,
                args: vec![],
            });
        }
    }

    Ok(parsed)
}

fn parse_java(source: &str) -> Result<ParsedSource, ParseError> {
    let mut parsed = ParsedSource::new(Language::Java);

    let import_re = Regex::new(r"^\s*import\s+([^;]+);").unwrap();
    let fn_call_re = Regex::new(r"(\w+(?:\.\w+)?)\s*\(").unwrap();

    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if let Some(caps) = import_re.captures(trimmed) {
            let import = caps.get(1).unwrap().as_str().trim().to_string();
            parsed.imports.push(import.clone());
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::Import,
                line: line_num,
                column: 0,
                content: import,
            });
        }

        for caps in fn_call_re.captures_iter(trimmed) {
            let fn_name = caps.get(1).unwrap().as_str().to_string();
            let column = line.find(&fn_name).unwrap_or(0);
            parsed.function_calls.push(FunctionCall {
                name: fn_name.clone(),
                line: line_num,
                column,
                args: vec![],
            });
        }
    }

    Ok(parsed)
}

fn parse_go(source: &str) -> Result<ParsedSource, ParseError> {
    let mut parsed = ParsedSource::new(Language::Go);

    let import_re = Regex::new(r#"^\s*import\s+"([^"]+)""#).unwrap();
    let fn_call_re = Regex::new(r"(\w+(?:\.\w+)?)\s*\(").unwrap();

    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if let Some(caps) = import_re.captures(trimmed) {
            let import = caps.get(1).unwrap().as_str().to_string();
            parsed.imports.push(import.clone());
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::Import,
                line: line_num,
                column: 0,
                content: import,
            });
        }

        for caps in fn_call_re.captures_iter(trimmed) {
            let fn_name = caps.get(1).unwrap().as_str().to_string();
            let column = line.find(&fn_name).unwrap_or(0);
            parsed.function_calls.push(FunctionCall {
                name: fn_name.clone(),
                line: line_num,
                column,
                args: vec![],
            });
        }
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust() {
        let source = "use std::collections::HashMap;\nlet x = HashMap::new();";
        let result = parse_file(source, "rust").unwrap();
        assert_eq!(result.imports.len(), 1);
        assert!(
            result
                .function_calls
                .iter()
                .any(|f| f.name.contains("HashMap"))
        );
    }

    #[test]
    fn test_parse_javascript() {
        let source = "import crypto from 'crypto';\ncrypto.createCipher('aes', key);";
        let result = parse_file(source, "javascript").unwrap();
        assert!(!result.imports.is_empty());
        assert!(
            result
                .function_calls
                .iter()
                .any(|f| f.name.contains("crypto"))
        );
    }

    #[test]
    fn test_parse_python() {
        let source = "from Crypto.Cipher import AES\nAES.new(key, AES.MODE_CBC)";
        let result = parse_file(source, "python").unwrap();
        assert!(!result.imports.is_empty());
    }

    #[test]
    fn test_unsupported_language() {
        let result = parse_file("code", "cobol");
        assert!(result.is_err());
    }
}
