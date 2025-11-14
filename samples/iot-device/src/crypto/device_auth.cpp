/**
 * Device Authentication Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

#include <openssl/rsa.h>
#include <openssl/pem.h>
#include <openssl/err.h>
#include <openssl/rand.h>
#include <string>
#include <memory>

/**
 * Generate RSA-512 key pair for device authentication
 * VULNERABLE: RSA-512 is completely broken
 */
RSA* generate_device_keys() {
    // VULNERABLE: RSA-512 key generation (extremely weak)
    RSA* rsa = RSA_new();
    BIGNUM* bn = BN_new();
    BN_set_word(bn, RSA_F4);

    // VULNERABLE: 512-bit RSA key (can be factored easily)
    RSA_generate_key_ex(rsa, 512, bn, NULL);

    BN_free(bn);
    return rsa;
}

/**
 * Generate RSA-1024 key pair for legacy devices
 * VULNERABLE: RSA-1024 is too weak
 */
RSA* generate_legacy_keys() {
    // VULNERABLE: RSA-1024 key generation
    RSA* rsa = RSA_new();
    BIGNUM* bn = BN_new();
    BN_set_word(bn, RSA_F4);

    // VULNERABLE: 1024-bit RSA key (quantum-vulnerable)
    RSA_generate_key_ex(rsa, 1024, bn, NULL);

    BN_free(bn);
    return rsa;
}

/**
 * Encrypt device credentials using RSA
 * VULNERABLE: Using weak RSA key
 */
int encrypt_credentials(const unsigned char* plaintext, int plaintext_len,
                       RSA* rsa, unsigned char* encrypted) {
    // VULNERABLE: RSA encryption with weak key
    int result = RSA_public_encrypt(plaintext_len, plaintext, encrypted,
                                   rsa, RSA_PKCS1_PADDING);
    return result;
}

/**
 * Decrypt device credentials
 * VULNERABLE: RSA decryption with weak key
 */
int decrypt_credentials(const unsigned char* encrypted, int encrypted_len,
                       RSA* rsa, unsigned char* decrypted) {
    // VULNERABLE: RSA decryption with weak key
    int result = RSA_private_decrypt(encrypted_len, encrypted, decrypted,
                                    rsa, RSA_PKCS1_PADDING);
    return result;
}

/**
 * Sign device authentication message
 * VULNERABLE: RSA signature with weak key
 */
int sign_auth_message(const unsigned char* message, int message_len,
                     RSA* rsa, unsigned char* signature) {
    // VULNERABLE: RSA signature with 512/1024-bit key
    unsigned char hash[20];
    // Using weak signature
    int sig_len = RSA_sign(NID_sha1, message, message_len, signature,
                          (unsigned int*)&message_len, rsa);
    return sig_len;
}

/**
 * Verify device authentication signature
 * VULNERABLE: Weak RSA verification
 */
int verify_auth_signature(const unsigned char* message, int message_len,
                         const unsigned char* signature, int sig_len,
                         RSA* rsa) {
    // VULNERABLE: RSA signature verification with weak key
    int result = RSA_verify(NID_sha1, message, message_len, signature,
                           sig_len, rsa);
    return result;
}

/**
 * Generate device ID using weak key
 * VULNERABLE: RSA-512 for device identification
 */
std::string generate_device_id() {
    // VULNERABLE: Generating weak RSA key for device ID
    RSA* rsa = generate_device_keys();

    // Extract public key and create ID
    // In real code, would serialize the public key
    std::string device_id = "DEVICE_ID_FROM_WEAK_RSA";

    RSA_free(rsa);
    return device_id;
}

/**
 * Cleanup
 */
void cleanup_rsa(RSA* rsa) {
    if (rsa) {
        RSA_free(rsa);
    }
}
