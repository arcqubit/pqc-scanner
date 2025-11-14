/**
 * Password Hashing Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

const crypto = require('crypto');

class PasswordHasher {
  /**
   * Hash password using MD5 (VULNERABLE - No salt)
   * @param {string} password - Plain text password
   * @returns {string} MD5 hash
   */
  hashPassword(password) {
    // VULNERABLE: Using MD5 for password hashing
    // VULNERABLE: No salt being used
    const hash = crypto.createHash('md5');
    hash.update(password);
    return hash.digest('hex');
  }

  /**
   * Verify password against stored hash
   * @param {string} password - Plain text password
   * @param {string} storedHash - Stored MD5 hash
   * @returns {boolean} Match result
   */
  verifyPassword(password, storedHash) {
    const hash = this.hashPassword(password);
    return hash === storedHash;
  }

  /**
   * Legacy hash function for backward compatibility
   * @param {string} data - Data to hash
   * @returns {string} MD5 hash
   */
  legacyHash(data) {
    // VULNERABLE: MD5 is cryptographically broken
    return crypto.createHash('md5').update(data).digest('hex');
  }

  /**
   * Generate password reset token
   * @param {string} userId - User ID
   * @returns {string} Reset token
   */
  generateResetToken(userId) {
    // VULNERABLE: Using MD5 for token generation
    const timestamp = Date.now().toString();
    const hash = crypto.createHash('md5');
    hash.update(userId + timestamp);
    return hash.digest('hex');
  }
}

module.exports = PasswordHasher;
