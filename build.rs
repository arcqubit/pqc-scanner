// Build script to validate JSON database files at compile time
// This prevents runtime panics from corrupted database files

use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=data/cccs_algorithms.json");
    println!("cargo:rerun-if-changed=data/cmvp_certificates.json");

    // Validate CCCS algorithms database
    let cccs_path = Path::new("data/cccs_algorithms.json");
    if !cccs_path.exists() {
        panic!("CCCS algorithms database not found at data/cccs_algorithms.json");
    }

    let cccs_data = fs::read_to_string(cccs_path)
        .expect("Failed to read CCCS algorithms database");

    // Validate JSON structure
    match serde_json::from_str::<serde_json::Value>(&cccs_data) {
        Ok(_) => println!("cargo:warning=CCCS algorithms database validated successfully"),
        Err(e) => panic!(
            "CCCS algorithms database contains invalid JSON: {}",
            e
        ),
    }

    // Validate CMVP certificates database
    let cmvp_path = Path::new("data/cmvp_certificates.json");
    if !cmvp_path.exists() {
        panic!("CMVP certificates database not found at data/cmvp_certificates.json");
    }

    let cmvp_data = fs::read_to_string(cmvp_path)
        .expect("Failed to read CMVP certificates database");

    // Validate JSON structure
    match serde_json::from_str::<serde_json::Value>(&cmvp_data) {
        Ok(_) => println!("cargo:warning=CMVP certificates database validated successfully"),
        Err(e) => panic!(
            "CMVP certificates database contains invalid JSON: {}",
            e
        ),
    }

    println!("cargo:warning=All database files validated at build time");
}
