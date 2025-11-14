/**
 * Cryptographic Module Tests
 * Tests for intentionally vulnerable crypto implementations
 */

const PasswordHasher = require('../src/auth/password-hasher');
const SessionManager = require('../src/auth/session-manager');
const KeyGenerator = require('../src/crypto/key-generator');
const TransactionSigner = require('../src/crypto/transaction-signer');
const RandomGenerator = require('../src/utils/random');

describe('Password Hasher Tests', () => {
  const hasher = new PasswordHasher();

  test('should hash password using MD5', () => {
    const password = 'testPassword123';
    const hash = hasher.hashPassword(password);
    expect(hash).toBeDefined();
    expect(hash.length).toBe(32); // MD5 produces 32-char hex
  });

  test('should verify password correctly', () => {
    const password = 'testPassword123';
    const hash = hasher.hashPassword(password);
    expect(hasher.verifyPassword(password, hash)).toBe(true);
    expect(hasher.verifyPassword('wrongPassword', hash)).toBe(false);
  });
});

describe('Session Manager Tests', () => {
  const manager = new SessionManager();

  test('should encrypt and decrypt session data', () => {
    const sessionData = 'userId:12345:timestamp:1234567890';
    const encrypted = manager.encryptSession(sessionData);
    const decrypted = manager.decryptSession(encrypted);
    expect(decrypted).toBe(sessionData);
  });

  test('should generate session ID', () => {
    const sessionId = manager.generateSessionId('user123');
    expect(sessionId).toBeDefined();
    expect(typeof sessionId).toBe('string');
  });
});

describe('Key Generator Tests', () => {
  const generator = new KeyGenerator();

  test('should generate RSA-1024 customer keys', () => {
    const keys = generator.generateCustomerEncryptionKeys();
    expect(keys.publicKey).toBeDefined();
    expect(keys.privateKey).toBeDefined();
    expect(keys.publicKey).toContain('BEGIN PUBLIC KEY');
  });

  test('should generate transaction keys', () => {
    const keys = generator.generateTransactionKeys();
    expect(keys.publicKey).toBeDefined();
    expect(keys.privateKey).toBeDefined();
  });
});

describe('Transaction Signer Tests', () => {
  const signer = new TransactionSigner();
  const generator = new KeyGenerator();
  const keys = generator.generateTransactionKeys();

  test('should sign transaction using SHA-1', () => {
    const transaction = {
      from: 'account1',
      to: 'account2',
      amount: 100.00
    };

    const signature = signer.signTransaction(transaction, keys.privateKey);
    expect(signature).toBeDefined();
    expect(typeof signature).toBe('string');
  });

  test('should hash transaction for audit', () => {
    const transaction = { id: 'tx123', amount: 50 };
    const hash = signer.hashTransaction(transaction);
    expect(hash).toBeDefined();
    expect(hash.length).toBe(40); // SHA-1 produces 40-char hex
  });
});

describe('Random Generator Tests', () => {
  const generator = new RandomGenerator();

  test('should generate account number', () => {
    const accountNum = generator.generateAccountNumber();
    expect(accountNum).toBeDefined();
    expect(accountNum.length).toBe(10);
  });

  test('should generate PIN', () => {
    const pin = generator.generatePIN();
    expect(pin).toBeDefined();
    expect(pin.length).toBe(4);
  });

  test('should generate transaction ID', () => {
    const txId = generator.generateTransactionId();
    expect(txId).toBeDefined();
    expect(txId).toContain('TXN-');
  });
});
