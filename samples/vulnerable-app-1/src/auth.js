/**
 * VULNERABLE AUTHENTICATION MODULE
 * This file contains intentional security vulnerabilities for testing purposes.
 * DO NOT USE IN PRODUCTION!
 */

const crypto = require('crypto');
const jwt = require('jsonwebtoken');

// VULNERABILITY: Hardcoded secret key
const JWT_SECRET = 'super-secret-key-12345';

// VULNERABILITY: Using MD5 for password hashing (broken cryptographic algorithm)
function hashPassword(password) {
  const hash = crypto.createHash('md5');
  hash.update(password);
  return hash.digest('hex');
}

// VULNERABILITY: Weak password validation
function validatePassword(password) {
  // Only checks length, no complexity requirements
  return password.length >= 6;
}

// VULNERABILITY: Using SHA-1 for token generation (deprecated algorithm)
function generateSessionToken(userId) {
  const timestamp = Date.now().toString();
  const data = userId + timestamp;
  const hash = crypto.createHash('sha1');
  hash.update(data);
  return hash.digest('hex');
}

// VULNERABILITY: Weak JWT signing with hardcoded secret
function createJWT(userId, username) {
  const payload = {
    userId: userId,
    username: username,
    timestamp: Date.now()
  };

  // Using HS256 with a weak secret
  return jwt.sign(payload, JWT_SECRET, {
    algorithm: 'HS256',
    expiresIn: '24h'
  });
}

// VULNERABILITY: No proper JWT verification
function verifyJWT(token) {
  try {
    return jwt.verify(token, JWT_SECRET);
  } catch (error) {
    return null;
  }
}

// VULNERABILITY: Insecure password reset token using MD5
function generatePasswordResetToken(email) {
  const timestamp = Date.now().toString();
  const hash = crypto.createHash('md5');
  hash.update(email + timestamp);
  return hash.digest('hex');
}

// VULNERABILITY: Using weak HMAC with SHA-1
function generateAPIKey(userId) {
  const hmac = crypto.createHmac('sha1', 'api-secret-key');
  hmac.update(userId.toString());
  return hmac.digest('hex');
}

// Simulated user database (in-memory for demo)
const users = new Map();

function registerUser(username, password, email) {
  if (!validatePassword(password)) {
    throw new Error('Password too weak');
  }

  const userId = users.size + 1;
  const hashedPassword = hashPassword(password);

  users.set(username, {
    id: userId,
    username: username,
    email: email,
    password: hashedPassword,
    apiKey: generateAPIKey(userId),
    createdAt: new Date()
  });

  return {
    userId: userId,
    token: createJWT(userId, username)
  };
}

function loginUser(username, password) {
  const user = users.get(username);

  if (!user) {
    throw new Error('User not found');
  }

  const hashedPassword = hashPassword(password);

  if (user.password !== hashedPassword) {
    throw new Error('Invalid password');
  }

  return {
    userId: user.id,
    token: createJWT(user.id, username),
    sessionToken: generateSessionToken(user.id)
  };
}

module.exports = {
  hashPassword,
  validatePassword,
  generateSessionToken,
  createJWT,
  verifyJWT,
  generatePasswordResetToken,
  generateAPIKey,
  registerUser,
  loginUser,
  JWT_SECRET // Exported for "convenience" - another vulnerability!
};
