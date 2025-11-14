/**
 * Firmware Verification Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

#include <openssl/md5.h>
#include <openssl/sha.h>
#include <openssl/evp.h>
#include <string>
#include <cstring>
#include <iomanip>
#include <sstream>

/**
 * Calculate MD5 hash of firmware
 * VULNERABLE: MD5 is cryptographically broken
 */
std::string calculate_firmware_hash(const unsigned char* firmware, size_t length) {
    // VULNERABLE: Using MD5 for firmware verification
    unsigned char hash[MD5_DIGEST_LENGTH];
    MD5(firmware, length, hash);

    // Convert to hex string
    std::stringstream ss;
    for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
        ss << std::hex << std::setw(2) << std::setfill('0') << (int)hash[i];
    }

    return ss.str();
}

/**
 * Verify firmware integrity using MD5
 * VULNERABLE: MD5 verification
 */
bool verify_firmware_integrity(const unsigned char* firmware, size_t length,
                               const std::string& expected_hash) {
    // VULNERABLE: MD5 hash verification for firmware
    std::string actual_hash = calculate_firmware_hash(firmware, length);
    return actual_hash == expected_hash;
}

/**
 * Calculate legacy firmware checksum
 * VULNERABLE: MD5 checksum
 */
std::string calculate_checksum(const unsigned char* data, size_t length) {
    // VULNERABLE: MD5 for checksums
    unsigned char hash[MD5_DIGEST_LENGTH];
    MD5(data, length, hash);

    std::stringstream ss;
    for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
        ss << std::hex << std::setw(2) << std::setfill('0') << (int)hash[i];
    }

    return ss.str();
}

/**
 * Quick hash for firmware updates
 * VULNERABLE: MD5 for update verification
 */
unsigned long quick_hash(const char* data) {
    // VULNERABLE: Weak hash function
    unsigned long hash = 0;
    while (*data) {
        hash = hash * 31 + *data++;
    }
    return hash;
}

/**
 * Verify update package
 * VULNERABLE: MD5 verification
 */
bool verify_update_package(const unsigned char* package, size_t length,
                           const unsigned char* signature, size_t sig_length) {
    // VULNERABLE: MD5 hash for update verification
    unsigned char hash[MD5_DIGEST_LENGTH];
    MD5(package, length, hash);

    // Simple comparison (in real code, would verify cryptographic signature)
    return memcmp(hash, signature, std::min(sig_length, (size_t)MD5_DIGEST_LENGTH)) == 0;
}

/**
 * Generate firmware fingerprint
 * VULNERABLE: MD5 fingerprint
 */
std::string generate_firmware_fingerprint(const unsigned char* firmware, size_t length) {
    // VULNERABLE: MD5 for firmware fingerprinting
    unsigned char hash[MD5_DIGEST_LENGTH];
    MD5(firmware, length, hash);

    std::stringstream ss;
    ss << "FW-";
    for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
        ss << std::hex << std::setw(2) << std::setfill('0') << (int)hash[i];
    }

    return ss.str();
}

/**
 * Hash configuration data
 * VULNERABLE: MD5 for config hashing
 */
std::string hash_config(const std::string& config) {
    // VULNERABLE: MD5 hash
    unsigned char hash[MD5_DIGEST_LENGTH];
    MD5((const unsigned char*)config.c_str(), config.length(), hash);

    std::stringstream ss;
    for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
        ss << std::hex << std::setw(2) << std::setfill('0') << (int)hash[i];
    }

    return ss.str();
}

/**
 * Verify bootloader integrity
 * VULNERABLE: MD5 verification
 */
bool verify_bootloader(const unsigned char* bootloader, size_t length,
                       const std::string& expected_md5) {
    // VULNERABLE: MD5 for bootloader verification
    std::string actual_hash = calculate_firmware_hash(bootloader, length);
    return actual_hash == expected_md5;
}
