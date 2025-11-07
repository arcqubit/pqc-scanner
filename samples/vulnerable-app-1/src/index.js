/**
 * VULNERABLE APPLICATION - MAIN ENTRY POINT
 * This is an intentionally vulnerable application for security testing.
 * DO NOT USE IN PRODUCTION!
 */

const express = require('express');
const bodyParser = require('body-parser');
const auth = require('./auth');
const crypto = require('./crypto');
const storage = require('./storage');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(bodyParser.json());

// Initialize session store
const sessionStore = new storage.SessionStore();

// VULNERABILITY: No rate limiting on authentication endpoints
app.post('/api/register', (req, res) => {
  try {
    const { username, password, email } = req.body;
    const result = auth.registerUser(username, password, email);

    res.json({
      success: true,
      userId: result.userId,
      token: result.token
    });
  } catch (error) {
    res.status(400).json({ success: false, error: error.message });
  }
});

app.post('/api/login', (req, res) => {
  try {
    const { username, password } = req.body;
    const result = auth.loginUser(username, password);

    // VULNERABILITY: Logging sensitive data
    storage.logUserActivity(result.userId, 'login', { username, password });

    res.json({
      success: true,
      token: result.token,
      sessionToken: result.sessionToken
    });
  } catch (error) {
    res.status(401).json({ success: false, error: error.message });
  }
});

// VULNERABILITY: Exposing cryptographic operations via API
app.post('/api/encrypt', (req, res) => {
  const { method, data } = req.body;

  let encrypted;
  switch (method) {
    case 'des':
      encrypted = crypto.encryptWithDES(data);
      break;
    case 'rc4':
      encrypted = crypto.encryptWithRC4(data);
      break;
    case 'ecb':
      encrypted = crypto.encryptWithECB(data);
      break;
    case '3des':
      encrypted = crypto.encryptWith3DES(data);
      break;
    default:
      return res.status(400).json({ error: 'Unknown method' });
  }

  res.json({ encrypted });
});

// VULNERABILITY: Endpoint that generates weak RSA keys
app.get('/api/generate-keys', (req, res) => {
  const keys = crypto.generateWeakRSAKeys();

  // VULNERABILITY: Returning private key over HTTP!
  res.json({
    publicKey: keys.publicKey,
    privateKey: keys.privateKey // NEVER DO THIS!
  });
});

// VULNERABILITY: File integrity check using MD5
app.post('/api/verify-file', (req, res) => {
  const { content, expectedHash } = req.body;
  const actualHash = crypto.calculateFileHash(content);

  res.json({
    valid: actualHash === expectedHash,
    hash: actualHash,
    algorithm: 'md5' // Weak!
  });
});

// VULNERABILITY: Storing credit card with weak encryption
app.post('/api/payment', (req, res) => {
  const { cardNumber, cvv, expiryDate } = req.body;
  const stored = storage.storeCreditCard(cardNumber, cvv, expiryDate);

  res.json({
    success: true,
    cardId: stored.cardHash,
    lastFour: stored.lastFour
  });
});

// VULNERABILITY: Debug endpoint exposing sensitive configuration
app.get('/api/debug/config', (req, res) => {
  res.json({
    database: storage.DB_CONFIG, // Exposing credentials!
    jwtSecret: auth.JWT_SECRET,
    masterKey: crypto.MASTER_KEY.toString('hex')
  });
});

// VULNERABILITY: Endpoint that creates insecure backups
app.post('/api/backup', (req, res) => {
  const { data } = req.body;
  const backupPath = `/tmp/backup-${Date.now()}.json`;
  const backup = storage.createBackup(data, backupPath);

  res.json({
    success: true,
    backupPath: backupPath,
    metadata: backup.metadata
  });
});

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({
    status: 'running',
    version: '1.0.0',
    vulnerabilities: 'YES - This is a test application'
  });
});

// Start server
if (require.main === module) {
  app.listen(PORT, () => {
    console.log(`Vulnerable app running on port ${PORT}`);
    console.log('WARNING: This application contains intentional security vulnerabilities!');
    console.log('DO NOT use in production or expose to the internet!');
  });
}

module.exports = app;
