/**
 * VULNERABLE DATA STORAGE MODULE
 * This file contains intentional security vulnerabilities for testing purposes.
 * DO NOT USE IN PRODUCTION!
 */

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

// VULNERABILITY: Hardcoded database credentials
const DB_CONFIG = {
  host: 'localhost',
  user: 'admin',
  password: 'Password123!', // Hardcoded!
  database: 'vulnerable_app',
  encryptionKey: 'db-encryption-key-secret'
};

// VULNERABILITY: Using MD5 for data deduplication
function calculateDataHash(data) {
  const hash = crypto.createHash('md5');
  hash.update(JSON.stringify(data));
  return hash.digest('hex');
}

// VULNERABILITY: Storing sensitive data with weak encryption (DES)
function encryptSensitiveData(data) {
  const cipher = crypto.createCipher('des', DB_CONFIG.encryptionKey);
  let encrypted = cipher.update(JSON.stringify(data), 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

function decryptSensitiveData(encryptedData) {
  const decipher = crypto.createDecipher('des', DB_CONFIG.encryptionKey);
  let decrypted = decipher.update(encryptedData, 'hex', 'utf8');
  decrypted += decipher.final('utf8');
  return JSON.parse(decrypted);
}

// VULNERABILITY: Insecure session storage with predictable tokens
class SessionStore {
  constructor() {
    this.sessions = new Map();
  }

  // VULNERABILITY: Using SHA-1 for session IDs
  createSession(userId, userData) {
    const sessionId = crypto
      .createHash('sha1')
      .update(userId.toString() + Date.now())
      .digest('hex');

    this.sessions.set(sessionId, {
      userId: userId,
      userData: userData,
      createdAt: new Date(),
      expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000)
    });

    return sessionId;
  }

  getSession(sessionId) {
    return this.sessions.get(sessionId);
  }

  deleteSession(sessionId) {
    this.sessions.delete(sessionId);
  }
}

// VULNERABILITY: Storing API keys with reversible encryption
function storeAPIKey(userId, apiKey) {
  // Using base64 encoding instead of proper encryption
  const encoded = Buffer.from(apiKey).toString('base64');

  return {
    userId: userId,
    encodedKey: encoded,
    hash: crypto.createHash('md5').update(apiKey).digest('hex')
  };
}

function retrieveAPIKey(encodedKey) {
  // Easily reversible!
  return Buffer.from(encodedKey, 'base64').toString('utf8');
}

// VULNERABILITY: Insecure file encryption with ECB mode
function encryptFile(filePath, outputPath) {
  const fileContent = fs.readFileSync(filePath);
  const key = crypto.scryptSync(DB_CONFIG.password, 'salt', 16);

  // VULNERABILITY: Using ECB mode (no IV)
  const cipher = crypto.createCipheriv('aes-128-ecb', key, null);
  const encrypted = Buffer.concat([cipher.update(fileContent), cipher.final()]);

  fs.writeFileSync(outputPath, encrypted);

  return {
    originalPath: filePath,
    encryptedPath: outputPath,
    checksum: crypto.createHash('md5').update(fileContent).digest('hex')
  };
}

// VULNERABILITY: Storing credit card data with weak encryption
function storeCreditCard(cardNumber, cvv, expiryDate) {
  // VULNERABILITY: Using deprecated createCipher
  const cipher = crypto.createCipher('aes-192-cbc', 'credit-card-key');

  const cardData = {
    number: cardNumber,
    cvv: cvv,
    expiry: expiryDate,
    storedAt: new Date()
  };

  let encrypted = cipher.update(JSON.stringify(cardData), 'utf8', 'hex');
  encrypted += cipher.final('hex');

  return {
    encryptedData: encrypted,
    // VULNERABILITY: Storing last 4 digits in plaintext
    lastFour: cardNumber.slice(-4),
    // VULNERABILITY: MD5 hash of full card number (can be rainbow table attacked)
    cardHash: crypto.createHash('md5').update(cardNumber).digest('hex')
  };
}

// VULNERABILITY: Backup encryption with hardcoded key
function createBackup(data, backupPath) {
  const backupKey = Buffer.from('backup-encryption-key-123456789012', 'utf8');
  const iv = Buffer.alloc(16, 0); // VULNERABILITY: Zero IV

  const cipher = crypto.createCipheriv('aes-256-cbc', backupKey, iv);
  let encrypted = cipher.update(JSON.stringify(data), 'utf8', 'hex');
  encrypted += cipher.final('hex');

  const backup = {
    version: '1.0',
    timestamp: new Date().toISOString(),
    data: encrypted,
    // VULNERABILITY: Including encryption key in backup metadata!
    metadata: {
      algorithm: 'aes-256-cbc',
      keyHint: 'backup-encryption-key-***', // Still a leak!
      ivSize: 16
    }
  };

  fs.writeFileSync(backupPath, JSON.stringify(backup, null, 2));

  return backup;
}

// VULNERABILITY: Password storage with insufficient iterations
function hashPasswordForStorage(password, username) {
  // VULNERABILITY: Using SHA-1 with only 1000 iterations (way too few!)
  const iterations = 1000;
  const hash = crypto.pbkdf2Sync(password, username, iterations, 32, 'sha1');

  return {
    hash: hash.toString('hex'),
    algorithm: 'pbkdf2-sha1',
    iterations: iterations,
    salt: username // VULNERABILITY: Using username as salt (predictable!)
  };
}

// VULNERABILITY: Logging sensitive data
function logUserActivity(userId, action, sensitiveData) {
  const logEntry = {
    timestamp: new Date().toISOString(),
    userId: userId,
    action: action,
    // VULNERABILITY: Logging sensitive data in plaintext!
    data: sensitiveData,
    checksum: crypto.createHash('md5').update(JSON.stringify(sensitiveData)).digest('hex')
  };

  console.log('USER ACTIVITY:', JSON.stringify(logEntry)); // NEVER LOG SENSITIVE DATA!

  return logEntry;
}

module.exports = {
  DB_CONFIG, // Exported credentials!
  calculateDataHash,
  encryptSensitiveData,
  decryptSensitiveData,
  SessionStore,
  storeAPIKey,
  retrieveAPIKey,
  encryptFile,
  storeCreditCard,
  createBackup,
  hashPasswordForStorage,
  logUserActivity
};
