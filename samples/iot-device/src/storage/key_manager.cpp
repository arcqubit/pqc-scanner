/**
 * Key Storage and Management Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

#include <fstream>
#include <string>
#include <cstring>

/**
 * Key Manager class with weak storage
 * VULNERABLE: Plaintext key storage
 */
class KeyManager {
private:
    std::string key_file_path;

    // VULNERABLE: Hardcoded encryption keys
    static constexpr const char* MASTER_KEY = "hardcoded_master_key_insecure";
    static constexpr const char* DEVICE_KEY = "device_key_plaintext_storage";

public:
    KeyManager(const std::string& path) : key_file_path(path) {}

    /**
     * Store key in plaintext file
     * VULNERABLE: Plaintext key storage
     */
    bool store_key(const std::string& key_id, const unsigned char* key, size_t key_length) {
        // VULNERABLE: Writing keys to plaintext file
        std::ofstream file(key_file_path + "/" + key_id + ".key", std::ios::binary);

        if (!file.is_open()) {
            return false;
        }

        // VULNERABLE: No encryption, just writing raw key
        file.write((const char*)key, key_length);
        file.close();

        return true;
    }

    /**
     * Load key from plaintext file
     * VULNERABLE: Reading plaintext keys
     */
    bool load_key(const std::string& key_id, unsigned char* key, size_t& key_length) {
        // VULNERABLE: Reading keys from plaintext file
        std::ifstream file(key_file_path + "/" + key_id + ".key", std::ios::binary);

        if (!file.is_open()) {
            return false;
        }

        file.read((char*)key, key_length);
        key_length = file.gcount();
        file.close();

        return true;
    }

    /**
     * Get hardcoded master key
     * VULNERABLE: Hardcoded key in source code
     */
    const char* get_master_key() {
        // VULNERABLE: Returning hardcoded key
        return MASTER_KEY;
    }

    /**
     * Get device key (hardcoded)
     * VULNERABLE: Hardcoded device key
     */
    const char* get_device_key() {
        // VULNERABLE: Hardcoded device key
        return DEVICE_KEY;
    }

    /**
     * Store key with weak obfuscation
     * VULNERABLE: Weak obfuscation, not encryption
     */
    bool store_obfuscated_key(const std::string& key_id,
                             const unsigned char* key, size_t key_length) {
        // VULNERABLE: XOR "obfuscation" is not encryption
        unsigned char obfuscated[256];
        const unsigned char obf_key = 0xAA; // VULNERABLE: Single-byte XOR

        for (size_t i = 0; i < key_length && i < 256; i++) {
            obfuscated[i] = key[i] ^ obf_key; // VULNERABLE: Trivial to reverse
        }

        std::ofstream file(key_file_path + "/" + key_id + ".obf", std::ios::binary);
        if (!file.is_open()) {
            return false;
        }

        file.write((const char*)obfuscated, key_length);
        file.close();

        return true;
    }

    /**
     * Load obfuscated key
     * VULNERABLE: Weak deobfuscation
     */
    bool load_obfuscated_key(const std::string& key_id,
                            unsigned char* key, size_t& key_length) {
        // VULNERABLE: Reading obfuscated (not encrypted) key
        unsigned char obfuscated[256];

        std::ifstream file(key_file_path + "/" + key_id + ".obf", std::ios::binary);
        if (!file.is_open()) {
            return false;
        }

        file.read((char*)obfuscated, 256);
        key_length = file.gcount();

        const unsigned char obf_key = 0xAA;
        for (size_t i = 0; i < key_length; i++) {
            key[i] = obfuscated[i] ^ obf_key; // VULNERABLE: Trivial XOR
        }

        file.close();
        return true;
    }

    /**
     * Export key in plaintext
     * VULNERABLE: Exporting keys without protection
     */
    std::string export_key_plaintext(const unsigned char* key, size_t key_length) {
        // VULNERABLE: Exporting key as plaintext string
        std::string key_str;
        for (size_t i = 0; i < key_length; i++) {
            char hex[3];
            sprintf(hex, "%02x", key[i]);
            key_str += hex;
        }
        return key_str;
    }

    /**
     * Store credentials in plaintext config
     * VULNERABLE: Plaintext credential storage
     */
    bool store_credentials(const std::string& username, const std::string& password) {
        // VULNERABLE: Storing credentials in plaintext file
        std::ofstream file(key_file_path + "/credentials.txt");

        if (!file.is_open()) {
            return false;
        }

        // VULNERABLE: Plaintext username and password
        file << "username=" << username << std::endl;
        file << "password=" << password << std::endl;
        file.close();

        return true;
    }
};

/**
 * Get hardcoded API key
 * VULNERABLE: Hardcoded API key in source
 */
const char* get_api_key() {
    // VULNERABLE: Hardcoded API key
    return "api_key_hardcoded_in_source_12345";
}

/**
 * Get hardcoded encryption key
 * VULNERABLE: Hardcoded encryption key
 */
const char* get_encryption_key() {
    // VULNERABLE: Hardcoded encryption key
    return "encryption_key_not_in_hsm";
}
