/**
 * Random Number Generation Utilities
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

class RandomGenerator {
  /**
   * Generate random account number
   * @returns {string} Random account number
   */
  generateAccountNumber() {
    // VULNERABLE: Using Math.random() for security-sensitive data
    const randomNum = Math.floor(Math.random() * 10000000000);
    return randomNum.toString().padStart(10, '0');
  }

  /**
   * Generate random PIN
   * @returns {string} 4-digit PIN
   */
  generatePIN() {
    // VULNERABLE: Math.random() is predictable
    const pin = Math.floor(Math.random() * 10000);
    return pin.toString().padStart(4, '0');
  }

  /**
   * Generate transaction ID
   * @returns {string} Transaction ID
   */
  generateTransactionId() {
    // VULNERABLE: Weak random for transaction IDs
    const timestamp = Date.now();
    const random = Math.floor(Math.random() * 1000000);
    return `TXN-${timestamp}-${random}`;
  }

  /**
   * Generate customer reference number
   * @returns {string} Reference number
   */
  generateReferenceNumber() {
    // VULNERABLE: Predictable random number generation
    const prefix = 'REF';
    const random = Math.random().toString(36).substring(2, 15);
    return `${prefix}-${random}`.toUpperCase();
  }

  /**
   * Generate verification code
   * @returns {string} 6-digit verification code
   */
  generateVerificationCode() {
    // VULNERABLE: Using Math.random() for OTP-like codes
    const code = Math.floor(Math.random() * 1000000);
    return code.toString().padStart(6, '0');
  }

  /**
   * Shuffle array (Fisher-Yates with weak random)
   * @param {Array} array - Array to shuffle
   * @returns {Array} Shuffled array
   */
  shuffleArray(array) {
    const shuffled = [...array];
    // VULNERABLE: Math.random() makes shuffle predictable
    for (let i = shuffled.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
    }
    return shuffled;
  }
}

module.exports = RandomGenerator;
