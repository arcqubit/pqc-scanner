// Example: Scan Directory and Generate Reports
//
// This example demonstrates how to:
// 1. Scan a directory of source files
// 2. Generate compliance reports
// 3. Save reports to output file
//
// Usage:
//   cargo run --example scan_directory -- --path <directory> --output <file.json>

use pqc_scanner::{
    analyze, export_oscal_json, export_sc13_json, generate_oscal_json, generate_sc13_report,
};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let mut path = String::from("src");
    let mut output = String::from("report.json");
    let mut format = String::from("sc13"); // sc13 or oscal

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                if i + 1 < args.len() {
                    path = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--format" => {
                if i + 1 < args.len() {
                    format = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    println!("=== PQC Scanner - Directory Scan ===");
    println!("Path: {}", path);
    println!("Output: {}", output);
    println!("Format: {}\n", format);

    // Scan directory and collect source files
    let source_files = scan_directory(&path);

    if source_files.is_empty() {
        eprintln!("No source files found in: {}", path);
        std::process::exit(1);
    }

    println!("Found {} source files", source_files.len());

    // Analyze all files
    let mut all_vulnerabilities = Vec::new();
    let mut total_lines = 0;
    let mut total_files = 0;

    for (_file_path, content, language) in source_files {
        println!("Scanning: {}", _file_path);

        match analyze(&content, &language) {
            Ok(result) => {
                total_lines += result.stats.lines_scanned;
                total_files += 1;

                for vuln in result.vulnerabilities {
                    all_vulnerabilities.push(vuln);
                }
            }
            Err(e) => {
                eprintln!("  Error scanning {}: {}", _file_path, e);
            }
        }
    }

    println!("\n=== Scan Results ===");
    println!("Files scanned: {}", total_files);
    println!("Lines scanned: {}", total_lines);
    println!("Vulnerabilities found: {}\n", all_vulnerabilities.len());

    // Create a synthetic audit result for report generation
    use pqc_scanner::Language;

    let mut audit_result = pqc_scanner::AuditResult {
        vulnerabilities: all_vulnerabilities.clone(),
        stats: pqc_scanner::AuditStats {
            lines_scanned: total_lines,
            total_vulnerabilities: all_vulnerabilities.len(),
            critical_count: all_vulnerabilities
                .iter()
                .filter(|v| matches!(v.severity, pqc_scanner::Severity::Critical))
                .count(),
            high_count: all_vulnerabilities
                .iter()
                .filter(|v| matches!(v.severity, pqc_scanner::Severity::High))
                .count(),
            medium_count: all_vulnerabilities
                .iter()
                .filter(|v| matches!(v.severity, pqc_scanner::Severity::Medium))
                .count(),
            low_count: all_vulnerabilities
                .iter()
                .filter(|v| matches!(v.severity, pqc_scanner::Severity::Low))
                .count(),
        },
        risk_score: calculate_risk_score(&all_vulnerabilities),
        language: Language::JavaScript, // Default, doesn't matter for report
        recommendations: Vec::new(),
    };

    audit_result.generate_recommendations();

    // Generate report
    let sc13_report = generate_sc13_report(&audit_result, Some(&path));

    println!("Compliance Score: {}/100", sc13_report.summary.compliance_score);
    println!("Risk Score: {}/100", audit_result.risk_score);

    // Export based on format
    let report_json = if format == "oscal" {
        let oscal = generate_oscal_json(&sc13_report, Some(&path));
        export_oscal_json(&oscal).expect("Failed to export OSCAL JSON")
    } else {
        export_sc13_json(&sc13_report).expect("Failed to export SC-13 JSON")
    };

    // Ensure output directory exists
    if let Some(parent) = Path::new(&output).parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    // Write report to file
    fs::write(&output, &report_json).expect("Failed to write report");

    println!("\n✓ Report saved to: {}", output);
    println!("✓ Report size: {} bytes", report_json.len());

    if audit_result.stats.critical_count > 0 {
        println!(
            "\n⚠️  WARNING: {} critical vulnerabilities found!",
            audit_result.stats.critical_count
        );
        std::process::exit(1);
    }
}

fn scan_directory(path: &str) -> Vec<(String, String, String)> {
    let mut source_files = Vec::new();

    let path_obj = Path::new(path);
    if !path_obj.exists() {
        eprintln!("Path does not exist: {}", path);
        return source_files;
    }

    if path_obj.is_file() {
        // Single file
        if let Some(language) = detect_language(path) {
            if let Ok(content) = fs::read_to_string(path) {
                source_files.push((path.to_string(), content, language));
            }
        }
    } else if path_obj.is_dir() {
        // Directory - recursively scan
        scan_directory_recursive(path, &mut source_files);
    }

    source_files
}

fn scan_directory_recursive(dir: &str, files: &mut Vec<(String, String, String)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();

            // Skip hidden directories and common ignore patterns
            if path_str.contains("/.") || path_str.contains("node_modules") || path_str.contains("target") || path_str.contains("build") {
                continue;
            }

            if path.is_file() {
                if let Some(language) = detect_language(&path_str) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        files.push((path_str, content, language));
                    }
                }
            } else if path.is_dir() {
                scan_directory_recursive(&path_str, files);
            }
        }
    }
}

fn detect_language(filename: &str) -> Option<String> {
    if filename.ends_with(".js") || filename.ends_with(".jsx") || filename.ends_with(".ts") || filename.ends_with(".tsx") {
        Some("javascript".to_string())
    } else if filename.ends_with(".py") {
        Some("python".to_string())
    } else if filename.ends_with(".java") {
        Some("java".to_string())
    } else if filename.ends_with(".cpp") || filename.ends_with(".cc") || filename.ends_with(".cxx") || filename.ends_with(".c") || filename.ends_with(".h") || filename.ends_with(".hpp") {
        Some("cpp".to_string())
    } else if filename.ends_with(".go") {
        Some("go".to_string())
    } else if filename.ends_with(".rs") {
        Some("rust".to_string())
    } else if filename.ends_with(".cs") {
        Some("csharp".to_string())
    } else {
        None
    }
}

fn calculate_risk_score(vulnerabilities: &[pqc_scanner::Vulnerability]) -> u32 {
    if vulnerabilities.is_empty() {
        return 0;
    }

    let mut score = 0u32;
    for vuln in vulnerabilities {
        score += match vuln.severity {
            pqc_scanner::Severity::Critical => 25,
            pqc_scanner::Severity::High => 15,
            pqc_scanner::Severity::Medium => 8,
            pqc_scanner::Severity::Low => 3,
        };
    }

    // Cap at 100
    score.min(100)
}
