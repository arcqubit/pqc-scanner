/**
 * VULNERABLE CRYPTOGRAPHIC OPERATIONS MODULE
 * This file contains intentional security vulnerabilities for testing purposes.
 * DO NOT USE IN PRODUCTION!
 */

const crypto = require('crypto');

// VULNERABILITY: Hardcoded encryption key
const MASTER_KEY = Buffer.from('this-is-a-secret-key-32-bytes!', 'utf8');

// VULNERABILITY: Using DES encryption (obsolete and insecure)
function encryptWithDES(plaintext) {
  const cipher = crypto.createCipher('des', 'weak-password');
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

function decryptWithDES(ciphertext) {
  const decipher = crypto.createDecipher('des', 'weak-password');
  let decrypted = decipher.update(ciphertext, 'hex', 'utf8');
  decrypted += decipher.final('utf8');
  return decrypted;
}

// VULNERABILITY: Using RC4 stream cipher (broken)
function encryptWithRC4(plaintext) {
  const cipher = crypto.createCipher('rc4', 'another-weak-key');
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

// VULNERABILITY: RSA with 1024-bit key (too small, vulnerable to factorization)
function generateWeakRSAKeys() {
  const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
    modulusLength: 1024, // WEAK! Should be at least 2048, preferably 4096
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

// VULNERABILITY: Using ECB mode (no IV, deterministic encryption)
function encryptWithECB(plaintext) {
  const cipher = crypto.createCipheriv('aes-128-ecb', MASTER_KEY.slice(0, 16), null);
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

function decryptWithECB(ciphertext) {
  const decipher = crypto.createDecipheriv('aes-128-ecb', MASTER_KEY.slice(0, 16), null);
  let decrypted = decipher.update(ciphertext, 'hex', 'utf8');
  decrypted += decipher.final('utf8');
  return decrypted;
}

// VULNERABILITY: Using MD5 for file integrity (collision-prone)
function calculateFileHash(fileContent) {
  const hash = crypto.createHash('md5');
  hash.update(fileContent);
  return hash.digest('hex');
}

// VULNERABILITY: Using SHA-1 for digital signatures (deprecated)
function signData(data, privateKey) {
  const sign = crypto.createSign('RSA-SHA1');
  sign.update(data);
  sign.end();
  return sign.sign(privateKey, 'hex');
}

function verifySignature(data, signature, publicKey) {
  const verify = crypto.createVerify('RSA-SHA1');
  verify.update(data);
  verify.end();
  return verify.verify(publicKey, signature, 'hex');
}

// VULNERABILITY: Weak random number generation
function generateWeakRandomToken() {
  // Using Math.random() instead of crypto.randomBytes
  return Math.random().toString(36).substring(2, 15);
}

// VULNERABILITY: Fixed IV (Initialization Vector) - should be random!
const FIXED_IV = Buffer.from('1234567890123456', 'utf8');

function encryptWithFixedIV(plaintext) {
  const cipher = crypto.createCipheriv('aes-256-cbc', MASTER_KEY, FIXED_IV);
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

// VULNERABILITY: Using deprecated crypto.createCipher (derives key from password insecurely)
function legacyEncrypt(plaintext, password) {
  const cipher = crypto.createCipher('aes-192-cbc', password);
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

// VULNERABILITY: Exposing private keys in comments/logs
function debugKeys() {
  const keys = generateWeakRSAKeys();
  console.log('DEBUG: Private Key:', keys.privateKey); // NEVER DO THIS!
  return keys;
}

// VULNERABILITY: Using Triple DES (3DES) - deprecated
function encryptWith3DES(plaintext) {
  const key = crypto.scryptSync('password', 'salt', 24);
  const cipher = crypto.createCipheriv('des-ede3', key, Buffer.alloc(8));
  let encrypted = cipher.update(plaintext, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

module.exports = {
  encryptWithDES,
  decryptWithDES,
  encryptWithRC4,
  generateWeakRSAKeys,
  encryptWithECB,
  decryptWithECB,
  calculateFileHash,
  signData,
  verifySignature,
  generateWeakRandomToken,
  encryptWithFixedIV,
  legacyEncrypt,
  debugKeys,
  encryptWith3DES,
  MASTER_KEY // Exported! Another vulnerability!
};
