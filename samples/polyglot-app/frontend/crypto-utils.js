/**
 * Frontend Crypto Utilities (JavaScript)
 * WARNING: Intentionally vulnerable - DO NOT use in production
 */

const crypto = require('crypto');

/**
 * Generate RSA-1024 keys for client-side encryption
 * VULNERABLE: RSA-1024 is too weak
 */
function generateClientKeys() {
    // VULNERABLE: RSA-1024
    const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
        modulusLength: 1024
    });
    return { publicKey, privateKey };
}

/**
 * Hash data for cache keys
 * VULNERABLE: MD5 for caching
 */
function generateCacheKey(data) {
    // VULNERABLE: MD5 hash
    return crypto.createHash('md5').update(data).digest('hex');
}

/**
 * Generate random session ID
 * VULNERABLE: Math.random() for session IDs
 */
function generateSessionId() {
    // VULNERABLE: Weak random
    return Math.random().toString(36).substring(2);
}

module.exports = { generateClientKeys, generateCacheKey, generateSessionId };
