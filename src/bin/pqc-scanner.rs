// PQC Scanner - CLI Entry Point
// Command-line interface for scanning directories for cryptographic vulnerabilities

use pqc_scanner::{
    Language, analyze, export_oscal_json, export_sc13_json, generate_oscal_json,
    generate_sc13_report,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

struct ScanOptions {
    target_path: String,
    report_dir: String,
    report_name: Option<String>,
    is_repo_url: bool,
    cleanup_after_scan: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle --version and --help flags
    if args.len() == 2 {
        match args[1].as_str() {
            "--version" | "-v" => {
                print_version();
                process::exit(0);
            }
            "--help" | "-h" => {
                print_usage(&args[0]);
                process::exit(0);
            }
            _ => {}
        }
    }

    if args.len() < 3 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "scan" => {
            let options = match parse_scan_args(&args[2..]) {
                Ok(opts) => opts,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    print_usage(&args[0]);
                    process::exit(1);
                }
            };

            if let Err(e) = scan_directory(options) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn parse_scan_args(args: &[String]) -> Result<ScanOptions, String> {
    if args.is_empty() {
        return Err("Missing target path or repository URL".to_string());
    }

    let mut target_path = None;
    let mut report_dir = "reports".to_string();
    let mut report_name = None;
    let mut cleanup_after_scan = true;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--report-dir" => {
                if i + 1 >= args.len() {
                    return Err("--report-dir requires a value".to_string());
                }
                report_dir = args[i + 1].clone();
                i += 2;
            }
            "--report-name" => {
                if i + 1 >= args.len() {
                    return Err("--report-name requires a value".to_string());
                }
                report_name = Some(args[i + 1].clone());
                i += 2;
            }
            "--keep-clone" => {
                cleanup_after_scan = false;
                i += 1;
            }
            arg => {
                if arg.starts_with("--") {
                    return Err(format!("Unknown option: {}", arg));
                }
                if target_path.is_none() {
                    target_path = Some(arg.to_string());
                    i += 1;
                } else {
                    return Err(format!("Unexpected argument: {}", arg));
                }
            }
        }
    }

    match target_path {
        Some(path) => {
            let is_repo_url = is_git_url(&path);
            Ok(ScanOptions {
                target_path: path,
                report_dir,
                report_name,
                is_repo_url,
                cleanup_after_scan,
            })
        }
        None => Err("Missing target path or repository URL".to_string()),
    }
}

fn is_git_url(path: &str) -> bool {
    path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("git@")
        || path.starts_with("ssh://")
        || path.ends_with(".git")
}

fn print_version() {
    println!("    ___                 ____        __    _ __ ");
    println!("   /   |  ____________ / __ \\__  __/ /_  (_) /_");
    println!("  / /| | / ___/ ___/ // / / / / / / __ \\/ / __/");
    println!(" / ___ |/ /  / /__/ // /_/ / /_/ / /_/ / / /_  ");
    println!("/_/  |_/_/   \\___/_/ \\___\\_\\__,_/_.___/_/\\__/  ");
    println!();
    println!("PQC Scanner - Quantum-Safe Cryptography Auditor");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("License: MIT");
    println!("Repository: https://github.com/arcqubit/pqc-scanner");
}

fn print_usage(program: &str) {
    eprintln!("    ___                 ____        __    _ __ ");
    eprintln!("   /   |  ____________ / __ \\__  __/ /_  (_) /_");
    eprintln!("  / /| | / ___/ ___/ // / / / / / / __ \\/ / __/");
    eprintln!(" / ___ |/ /  / /__/ // /_/ / /_/ / /_/ / / /_  ");
    eprintln!("/_/  |_/_/   \\___/_/ \\___\\_\\__,_/_.___/_/\\__/  ");
    eprintln!();
    eprintln!("PQC Scanner - Quantum-Safe Cryptography Auditor");
    eprintln!("Version: {}", env!("CARGO_PKG_VERSION"));
    eprintln!();
    eprintln!("Usage: {} [OPTIONS] | {} scan <directory|repo-url> [OPTIONS]", program, program);
    eprintln!();
    eprintln!("Global Options:");
    eprintln!("  -h, --help          Show this help message");
    eprintln!("  -v, --version       Show version information");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  scan <path>         Scan local directory for cryptographic vulnerabilities");
    eprintln!("  scan <repo-url>     Clone and scan remote Git repository");
    eprintln!();
    eprintln!("Scan Options:");
    eprintln!("  --report-dir <dir>     Output directory for reports (default: reports)");
    eprintln!("  --report-name <name>   Base name for report files (default: directory/repo name)");
    eprintln!("  --keep-clone           Keep cloned repository after scanning (default: cleanup)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} --version", program);
    eprintln!("  {} --help", program);
    eprintln!("  {} scan samples/vulnerable-app-1", program);
    eprintln!("  {} scan https://github.com/digininja/DVWA.git", program);
    eprintln!(
        "  {} scan https://github.com/org/repo.git --report-name my-audit --keep-clone",
        program
    );
}

fn scan_directory(mut options: ScanOptions) -> Result<(), String> {
    // If it's a repository URL, clone it first
    let cloned_path = if options.is_repo_url {
        println!("=== Cloning Repository ===");
        println!("URL: {}\n", options.target_path);

        match clone_repository(&options.target_path) {
            Ok(path) => {
                println!("✓ Cloned to: {}\n", path.display());

                // Update target_path to the cloned directory
                options.target_path = path.to_string_lossy().to_string();

                // Extract repo name for default report naming if not specified
                if options.report_name.is_none()
                    && let Some(name) = extract_repo_name(&options.target_path)
                {
                    options.report_name = Some(name);
                }

                Some(path)
            }
            Err(e) => {
                return Err(format!("Failed to clone repository: {}", e));
            }
        }
    } else {
        None
    };

    let target = PathBuf::from(&options.target_path);

    if !target.exists() {
        return Err(format!("Path does not exist: {}", options.target_path));
    }

    println!("    ___                 ____        __    _ __ ");
    println!("   /   |  ____________ / __ \\__  __/ /_  (_) /_");
    println!("  / /| | / ___/ ___/ // / / / / / / __ \\/ / __/");
    println!(" / ___ |/ /  / /__/ // /_/ / /_/ / /_/ / / /_  ");
    println!("/_/  |_/_/   \\___/_/ \\___\\_\\__,_/_.___/_/\\__/  ");
    println!();
    println!("=== PQC Scanner ===");
    println!("Scanning: {}\n", options.target_path);

    let mut total_files = 0;
    let mut total_vulnerabilities = 0;
    let mut critical_count = 0;
    let mut high_count = 0;
    let mut all_results = Vec::new();

    // Scan all supported files in directory
    if target.is_dir() {
        scan_dir_recursive(
            &target,
            &mut total_files,
            &mut total_vulnerabilities,
            &mut critical_count,
            &mut high_count,
            &mut all_results,
        )?;
    } else {
        return Err(format!(
            "Expected directory, got file: {}",
            options.target_path
        ));
    }

    println!("\n=== Scan Summary ===");
    println!("Files scanned: {}", total_files);
    println!("Total vulnerabilities: {}", total_vulnerabilities);
    println!("  Critical: {}", critical_count);
    println!("  High: {}", high_count);

    // Generate reports if vulnerabilities found
    if !all_results.is_empty() {
        println!("\nGenerating compliance reports...");

        // Create reports directory if it doesn't exist
        let reports_dir = PathBuf::from(&options.report_dir);
        fs::create_dir_all(&reports_dir)
            .map_err(|e| format!("Failed to create reports directory: {}", e))?;

        // Determine base name for reports
        let base_name = if let Some(name) = options.report_name {
            name
        } else {
            // Extract directory name from target path
            target
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("scan")
                .to_string()
        };

        // Use the first result for report generation (or combine them)
        if let Some(first_result) = all_results.first() {
            let sc13_report = generate_sc13_report(first_result, Some(&options.target_path));

            // Export SC-13 JSON
            match export_sc13_json(&sc13_report) {
                Ok(json) => {
                    let filename = format!("{}-sc13-compliance.json", base_name);
                    let output_file = reports_dir.join(filename);
                    fs::write(&output_file, json).map_err(|e| e.to_string())?;
                    println!("  ✓ SC-13 Report: {}", output_file.display());
                }
                Err(e) => eprintln!("  ✗ Failed to generate SC-13 report: {}", e),
            }

            // Export OSCAL JSON
            let oscal = generate_oscal_json(&sc13_report, Some(&options.target_path));
            match export_oscal_json(&oscal) {
                Ok(json) => {
                    let filename = format!("{}-oscal-assessment.json", base_name);
                    let output_file = reports_dir.join(filename);
                    fs::write(&output_file, json).map_err(|e| e.to_string())?;
                    println!("  ✓ OSCAL Report: {}", output_file.display());
                }
                Err(e) => eprintln!("  ✗ Failed to generate OSCAL report: {}", e),
            }
        }
    }

    // Cleanup cloned repository if requested
    if let Some(cloned) = cloned_path {
        if options.cleanup_after_scan {
            println!("\nCleaning up cloned repository...");
            if let Err(e) = fs::remove_dir_all(&cloned) {
                eprintln!("  ⚠ Warning: Failed to remove cloned directory: {}", e);
            } else {
                println!("  ✓ Removed: {}", cloned.display());
            }
        } else {
            println!("\nCloned repository kept at: {}", cloned.display());
        }
    }

    if critical_count > 0 || high_count > 0 {
        println!("\n⚠️  WARNING: Critical vulnerabilities found!");
        println!("Review the generated reports for detailed remediation steps.");
        process::exit(1);
    }

    Ok(())
}

fn clone_repository(url: &str) -> Result<PathBuf, String> {
    // Create a temporary directory for cloning
    let temp_dir = env::temp_dir().join(format!(
        "pdq-scanner-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));

    fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp directory: {}", e))?;

    // Extract repository name from URL
    let repo_name = extract_repo_name(url).unwrap_or_else(|| "repository".to_string());
    let clone_path = temp_dir.join(&repo_name);

    // Execute git clone command
    let output = process::Command::new("git")
        .args(["clone", "--depth", "1", url, clone_path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Failed to execute git clone: {}. Is git installed?", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git clone failed: {}", error_msg));
    }

    Ok(clone_path)
}

fn extract_repo_name(url: &str) -> Option<String> {
    // Extract repository name from various URL formats
    // https://github.com/user/repo.git -> repo
    // https://github.com/user/repo -> repo
    // git@github.com:user/repo.git -> repo
    // /path/to/repo -> repo

    let path_str = url.trim_end_matches('/').trim_end_matches(".git");

    let name = path_str
        .rsplit('/')
        .next()
        .or_else(|| path_str.rsplit(':').next())?;

    Some(name.to_string())
}

fn scan_dir_recursive(
    dir: &Path,
    total_files: &mut usize,
    total_vulns: &mut usize,
    critical: &mut usize,
    high: &mut usize,
    results: &mut Vec<pqc_scanner::AuditResult>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("Cannot read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            // Skip node_modules and common directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "node_modules" || name_str == ".git" || name_str == "target" {
                    continue;
                }
            }
            scan_dir_recursive(&path, total_files, total_vulns, critical, high, results)?;
        } else if path.is_file()
            && let Some(result) = scan_file(&path)?
        {
            *total_files += 1;
            *total_vulns += result.stats.total_vulnerabilities;
            *critical += result.stats.critical_count;
            *high += result.stats.high_count;

            if result.stats.total_vulnerabilities > 0 {
                println!("\n{}", path.display());
                println!("  Vulnerabilities: {}", result.stats.total_vulnerabilities);
                println!(
                    "  Critical: {}, High: {}",
                    result.stats.critical_count, result.stats.high_count
                );

                // Show first few vulnerabilities
                for (i, vuln) in result.vulnerabilities.iter().take(3).enumerate() {
                    println!(
                        "    {}. [{:?}] {} (line {})",
                        i + 1,
                        vuln.severity,
                        vuln.crypto_type,
                        vuln.line
                    );
                }

                if result.vulnerabilities.len() > 3 {
                    println!("    ... and {} more", result.vulnerabilities.len() - 3);
                }

                results.push(result);
            }
        }
    }

    Ok(())
}

fn scan_file(path: &Path) -> Result<Option<pqc_scanner::AuditResult>, String> {
    // Determine language from file extension
    let language = match path.extension().and_then(|s| s.to_str()) {
        Some("js") => Some(Language::JavaScript),
        Some("ts") => Some(Language::TypeScript),
        Some("py") => Some(Language::Python),
        Some("rs") => Some(Language::Rust),
        Some("java") => Some(Language::Java),
        Some("go") => Some(Language::Go),
        Some("cpp") | Some("cc") | Some("cxx") => Some(Language::Cpp),
        Some("cs") => Some(Language::Csharp),
        _ => None,
    };

    if let Some(lang) = language {
        // Read file content
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        // Analyze content
        let lang_str = match lang {
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Python => "python",
            Language::Rust => "rust",
            Language::Java => "java",
            Language::Go => "go",
            Language::Cpp => "cpp",
            Language::Csharp => "csharp",
        };

        match analyze(&content, lang_str) {
            Ok(result) => Ok(Some(result)),
            Err(e) => {
                eprintln!("Warning: Failed to analyze {}: {}", path.display(), e);
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}
