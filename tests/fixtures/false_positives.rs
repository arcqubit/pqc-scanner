// This file contains comments and strings that mention crypto
// but should not trigger actual vulnerabilities

fn main() {
    // We used to use RSA but migrated to quantum-safe algorithms
    let comment = "This mentions ECDSA in a string";
    println!("MD5 is mentioned here but not used");
}
