/**
 * Transaction Signing Module
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

const crypto = require('crypto');

class TransactionSigner {
  /**
   * Sign transaction using SHA-1
   * @param {Object} transaction - Transaction data
   * @param {string} privateKey - Private key for signing
   * @returns {string} Transaction signature
   */
  signTransaction(transaction, privateKey) {
    const transactionData = JSON.stringify(transaction);

    // VULNERABLE: Using SHA-1 for digital signatures (deprecated)
    const sign = crypto.createSign('SHA1');
    sign.update(transactionData);
    sign.end();

    const signature = sign.sign(privateKey, 'hex');
    return signature;
  }

  /**
   * Verify transaction signature
   * @param {Object} transaction - Transaction data
   * @param {string} signature - Transaction signature
   * @param {string} publicKey - Public key for verification
   * @returns {boolean} Verification result
   */
  verifyTransaction(transaction, signature, publicKey) {
    const transactionData = JSON.stringify(transaction);

    // VULNERABLE: Using SHA-1 for signature verification
    const verify = crypto.createVerify('SHA1');
    verify.update(transactionData);
    verify.end();

    return verify.verify(publicKey, signature, 'hex');
  }

  /**
   * Generate transaction hash for audit trail
   * @param {Object} transaction - Transaction data
   * @returns {string} Transaction hash
   */
  hashTransaction(transaction) {
    const transactionData = JSON.stringify(transaction);

    // VULNERABLE: SHA-1 hash for integrity checking
    const hash = crypto.createHash('sha1');
    hash.update(transactionData);
    return hash.digest('hex');
  }

  /**
   * Create transaction fingerprint
   * @param {string} accountId - Account ID
   * @param {number} amount - Transaction amount
   * @returns {string} Transaction fingerprint
   */
  createFingerprint(accountId, amount) {
    const data = `${accountId}:${amount}`;

    // VULNERABLE: SHA-1 for fingerprinting
    return crypto.createHash('sha1').update(data).digest('hex');
  }

  /**
   * Sign batch of transactions
   * @param {Array} transactions - Array of transactions
   * @param {string} privateKey - Private key
   * @returns {Array} Signed transactions
   */
  signBatch(transactions, privateKey) {
    return transactions.map(tx => {
      // VULNERABLE: SHA-1 signature for each transaction
      const signature = this.signTransaction(tx, privateKey);
      return { ...tx, signature };
    });
  }
}

module.exports = TransactionSigner;
