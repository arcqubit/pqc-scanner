/**
 * Cryptographic Key Generation Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

const crypto = require('crypto');

class KeyGenerator {
  /**
   * Generate RSA key pair for customer data encryption
   * @returns {Object} RSA key pair
   */
  generateCustomerEncryptionKeys() {
    // VULNERABLE: Using RSA-1024 (too small, quantum-vulnerable)
    const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
      modulusLength: 1024,
      publicKeyEncoding: {
        type: 'spki',
        format: 'pem'
      },
      privateKeyEncoding: {
        type: 'pkcs8',
        format: 'pem'
      }
    });

    return { publicKey, privateKey };
  }

  /**
   * Generate RSA key pair for transaction signing
   * @returns {Object} RSA key pair
   */
  generateTransactionKeys() {
    // VULNERABLE: RSA-1024 is too weak
    const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
      modulusLength: 1024,
      publicKeyEncoding: {
        type: 'spki',
        format: 'pem'
      },
      privateKeyEncoding: {
        type: 'pkcs8',
        format: 'pem',
        cipher: 'aes-256-cbc',
        passphrase: 'weak-passphrase'
      }
    });

    return { publicKey, privateKey };
  }

  /**
   * Generate encryption key for database
   * @returns {Buffer} Encryption key
   */
  generateDatabaseKey() {
    // VULNERABLE: Using RSA-1024 for key wrapping
    const { publicKey } = crypto.generateKeyPairSync('rsa', {
      modulusLength: 1024
    });

    return publicKey;
  }

  /**
   * Legacy key generation for backward compatibility
   * @returns {Object} RSA-512 key pair (extremely weak)
   */
  generateLegacyKeys() {
    // VULNERABLE: RSA-512 is completely broken
    const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
      modulusLength: 512,
      publicKeyEncoding: {
        type: 'spki',
        format: 'pem'
      },
      privateKeyEncoding: {
        type: 'pkcs8',
        format: 'pem'
      }
    });

    return { publicKey, privateKey };
  }
}

module.exports = KeyGenerator;
