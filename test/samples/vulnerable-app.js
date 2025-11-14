/**
 * Sample Vulnerable JavaScript Application
 * This file contains multiple cryptographic vulnerabilities for testing
 */

const crypto = require('crypto');
const fs = require('fs');

// CRITICAL: MD5 Hash - Cryptographically Broken
function hashPasswordMD5(password) {
    const hash = crypto.createHash('md5');
    hash.update(password);
    return hash.digest('hex');
}

// CRITICAL: SHA-1 Hash - Cryptographically Broken
function generateTokenSHA1(data) {
    const hash = crypto.createHash('sha1');
    hash.update(data);
    return hash.digest('hex');
}

// HIGH: RSA 1024-bit - Quantum Vulnerable
function generateRSA1024KeyPair() {
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

// HIGH: RSA 2048-bit - Quantum Vulnerable (better but still vulnerable)
function generateRSA2048KeyPair() {
    return crypto.generateKeyPairSync('rsa', {
        modulusLength: 2048,
        publicKeyEncoding: { type: 'spki', format: 'pem' },
        privateKeyEncoding: { type: 'pkcs8', format: 'pem' }
    });
}

// HIGH: ECDSA - Quantum Vulnerable
function signWithECDSA(data, privateKey) {
    const sign = crypto.createSign('ecdsa-with-SHA256');
    sign.update(data);
    return sign.sign(privateKey, 'hex');
}

// MEDIUM: 3DES - Deprecated
function encrypt3DES(text, key, iv) {
    const cipher = crypto.createCipheriv('des-ede3', key, iv);
    let encrypted = cipher.update(text, 'utf8', 'hex');
    encrypted += cipher.final('hex');
    return encrypted;
}

// MEDIUM: DES - Deprecated and Weak
function encryptDES(text, key, iv) {
    const cipher = crypto.createCipheriv('des', key, iv);
    let encrypted = cipher.update(text, 'utf8', 'hex');
    encrypted += cipher.final('hex');
    return encrypted;
}

// MEDIUM: DSA - Quantum Vulnerable
function generateDSAKeyPair() {
    return crypto.generateKeyPairSync('dsa', {
        modulusLength: 2048,
        divisorLength: 256,
        publicKeyEncoding: { type: 'spki', format: 'pem' },
        privateKeyEncoding: { type: 'pkcs8', format: 'pem' }
    });
}

// HIGH: Diffie-Hellman - Quantum Vulnerable
function createDHKeyExchange() {
    const dh = crypto.createDiffieHellman(2048);
    dh.generateKeys();
    return {
        publicKey: dh.getPublicKey('hex'),
        privateKey: dh.getPrivateKey('hex')
    };
}

// Application usage examples
class VulnerableAuthSystem {
    constructor() {
        this.users = new Map();
    }

    // CRITICAL: Storing password hashes with MD5
    registerUser(username, password) {
        const passwordHash = hashPasswordMD5(password);
        this.users.set(username, passwordHash);
        console.log(`User ${username} registered with MD5 hash`);
    }

    // CRITICAL: Using SHA-1 for session tokens
    createSession(username) {
        const timestamp = Date.now().toString();
        const sessionToken = generateTokenSHA1(username + timestamp);
        console.log(`Session token created with SHA-1`);
        return sessionToken;
    }

    // HIGH: Using RSA 1024 for encryption
    encryptData(data) {
        const { publicKey } = generateRSA1024KeyPair();
        const encrypted = crypto.publicEncrypt(publicKey, Buffer.from(data));
        return encrypted.toString('base64');
    }
}

// Example API endpoints (Express-style)
const routes = {
    // CRITICAL: MD5 for file integrity
    '/upload': function(req, res) {
        const fileHash = crypto.createHash('md5')
            .update(req.body.file)
            .digest('hex');
        res.json({ checksum: fileHash });
    },

    // HIGH: RSA 2048 for key exchange
    '/key-exchange': function(req, res) {
        const keys = generateRSA2048KeyPair();
        res.json({ publicKey: keys.publicKey });
    },

    // MEDIUM: 3DES encryption
    '/encrypt': function(req, res) {
        const key = Buffer.alloc(24);
        const iv = Buffer.alloc(8);
        const encrypted = encrypt3DES(req.body.data, key, iv);
        res.json({ encrypted });
    }
};

// Initialize vulnerable system
const auth = new VulnerableAuthSystem();
auth.registerUser('admin', 'password123');
const session = auth.createSession('admin');

// Export for testing
module.exports = {
    hashPasswordMD5,
    generateTokenSHA1,
    generateRSA1024KeyPair,
    generateRSA2048KeyPair,
    signWithECDSA,
    encrypt3DES,
    encryptDES,
    generateDSAKeyPair,
    createDHKeyExchange,
    VulnerableAuthSystem
};
