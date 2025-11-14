//! Multi-language source code parser for crypto pattern detection
//!
//! Supports: Rust, JavaScript, TypeScript, Python, Java, Go

use crate::types::*;
use regex::Regex;

/// Parser errors
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Parse error: {0}")]
    ParseFailure(String),

    #[error("Invalid source code")]
    InvalidSource,
}

/// Main parsing function - dispatches to language-specific parsers
pub fn parse_file(source: &str, language: &str) -> Result<ParsedSource, ParseError> {
    let lang = Language::from_string(language)
        .ok_or_else(|| ParseError::UnsupportedLanguage(language.to_string()))?;

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

    let use_re = Regex::new(r"^\s*use\s+([^;]+);").unwrap();
    let fn_call_re = Regex::new(r"(\w+(?:::\w+)?)\s*\(").unwrap();
    let struct_re = Regex::new(r"^\s*(?:pub\s+)?struct\s+(\w+)").unwrap();

    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if let Some(caps) = use_re.captures(trimmed) {
            let import = caps.get(1).unwrap().as_str().trim().to_string();
            parsed.imports.push(import.clone());
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::Import,
                line: line_num,
                column: 0,
                content: import,
            });
        }

        if let Some(caps) = struct_re.captures(trimmed) {
            parsed.ast_nodes.push(AstNode {
                node_type: NodeType::ClassDeclaration,
                line: line_num,
                column: 0,
                content: caps.get(1).unwrap().as_str().to_string(),
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
