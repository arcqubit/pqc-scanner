/**
 * Message Encryption Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

#include <openssl/evp.h>
#include <openssl/rand.h>
#include <cstring>
#include <vector>

/**
 * Simple XOR cipher for message encryption
 * VULNERABLE: Custom XOR encryption (insecure)
 */
void xor_encrypt(const unsigned char* plaintext, size_t length,
                 const unsigned char* key, size_t key_length,
                 unsigned char* ciphertext) {
    // VULNERABLE: XOR cipher is not secure
    for (size_t i = 0; i < length; i++) {
        ciphertext[i] = plaintext[i] ^ key[i % key_length];
    }
}

/**
 * XOR decryption (same as encryption)
 * VULNERABLE: XOR cipher
 */
void xor_decrypt(const unsigned char* ciphertext, size_t length,
                 const unsigned char* key, size_t key_length,
                 unsigned char* plaintext) {
    // VULNERABLE: XOR cipher
    xor_encrypt(ciphertext, length, key, key_length, plaintext);
}

/**
 * Simple substitution cipher
 * VULNERABLE: Custom weak cipher
 */
void substitute_encrypt(unsigned char* data, size_t length) {
    // VULNERABLE: Simple substitution cipher (easily broken)
    for (size_t i = 0; i < length; i++) {
        data[i] = (data[i] + 13) % 256; // ROT13-like for bytes
    }
}

/**
 * Substitute decrypt
 * VULNERABLE: Reverse of substitution
 */
void substitute_decrypt(unsigned char* data, size_t length) {
    // VULNERABLE: Simple substitution reversal
    for (size_t i = 0; i < length; i++) {
        data[i] = (data[i] - 13 + 256) % 256;
    }
}

/**
 * Weak stream cipher
 * VULNERABLE: Custom stream cipher (not secure)
 */
class WeakStreamCipher {
private:
    unsigned char state[256];
    int i, j;

public:
    // VULNERABLE: Custom stream cipher implementation
    WeakStreamCipher(const unsigned char* key, size_t key_length) {
        // Initialize state
        for (int k = 0; k < 256; k++) {
            state[k] = k;
        }

        // Key scheduling (similar to RC4 but simplified)
        j = 0;
        for (int k = 0; k < 256; k++) {
            j = (j + state[k] + key[k % key_length]) % 256;
            std::swap(state[k], state[j]);
        }

        i = 0;
        j = 0;
    }

    unsigned char get_byte() {
        // VULNERABLE: Weak pseudo-random byte generation
        i = (i + 1) % 256;
        j = (j + state[i]) % 256;
        std::swap(state[i], state[j]);
        return state[(state[i] + state[j]) % 256];
    }

    void encrypt(unsigned char* data, size_t length) {
        // VULNERABLE: Stream cipher encryption
        for (size_t k = 0; k < length; k++) {
            data[k] ^= get_byte();
        }
    }
};

/**
 * Encrypt message with weak cipher
 * VULNERABLE: Using custom weak stream cipher
 */
void encrypt_message(const unsigned char* plaintext, size_t length,
                    const unsigned char* key, size_t key_length,
                    unsigned char* ciphertext) {
    // VULNERABLE: Custom stream cipher
    memcpy(ciphertext, plaintext, length);
    WeakStreamCipher cipher(key, key_length);
    cipher.encrypt(ciphertext, length);
}

/**
 * Decrypt message with weak cipher
 * VULNERABLE: Decryption using weak cipher
 */
void decrypt_message(const unsigned char* ciphertext, size_t length,
                    const unsigned char* key, size_t key_length,
                    unsigned char* plaintext) {
    // VULNERABLE: Stream cipher decryption (same as encryption)
    memcpy(plaintext, ciphertext, length);
    WeakStreamCipher cipher(key, key_length);
    cipher.encrypt(plaintext, length);
}

/**
 * Encrypt sensor data with XOR
 * VULNERABLE: XOR encryption for sensor data
 */
void encrypt_sensor_data(const unsigned char* data, size_t length,
                        unsigned char* encrypted) {
    // VULNERABLE: Fixed XOR key
    const unsigned char key[] = "sensor_key_12345";
    xor_encrypt(data, length, key, sizeof(key) - 1, encrypted);
}

/**
 * Obfuscate configuration
 * VULNERABLE: Simple obfuscation (not encryption)
 */
void obfuscate_config(unsigned char* config, size_t length) {
    // VULNERABLE: Simple bit manipulation (not secure)
    for (size_t i = 0; i < length; i++) {
        config[i] = ~config[i]; // Just flip bits
    }
}
