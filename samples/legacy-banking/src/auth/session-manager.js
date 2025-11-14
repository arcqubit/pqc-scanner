/**
 * Session Management Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

const crypto = require('crypto');

class SessionManager {
  constructor() {
    // VULNERABLE: Hardcoded DES key
    this.algorithm = 'des';
    this.key = Buffer.from('12345678'); // 8 bytes for DES
  }

  /**
   * Encrypt session token using DES
   * @param {string} sessionData - Session data to encrypt
   * @returns {string} Encrypted session token
   */
  encryptSession(sessionData) {
    // VULNERABLE: Using DES encryption (deprecated and weak)
    const cipher = crypto.createCipher(this.algorithm, this.key);
    let encrypted = cipher.update(sessionData, 'utf8', 'hex');
    encrypted += cipher.final('hex');
    return encrypted;
  }

  /**
   * Decrypt session token
   * @param {string} encryptedData - Encrypted session token
   * @returns {string} Decrypted session data
   */
  decryptSession(encryptedData) {
    // VULNERABLE: Using DES decryption
    const decipher = crypto.createDecipher(this.algorithm, this.key);
    let decrypted = decipher.update(encryptedData, 'hex', 'utf8');
    decrypted += decipher.final('utf8');
    return decrypted;
  }

  /**
   * Generate session ID
   * @param {string} userId - User ID
   * @returns {string} Session ID
   */
  generateSessionId(userId) {
    // VULNERABLE: Weak session ID generation using DES
    const timestamp = Date.now().toString();
    const data = userId + ':' + timestamp;
    return this.encryptSession(data);
  }

  /**
   * Create secure cookie (not really secure)
   * @param {string} name - Cookie name
   * @param {string} value - Cookie value
   * @returns {string} Encrypted cookie value
   */
  createSecureCookie(name, value) {
    // VULNERABLE: DES encryption for cookies
    const cookieData = `${name}=${value}`;
    return this.encryptSession(cookieData);
  }
}

module.exports = SessionManager;
