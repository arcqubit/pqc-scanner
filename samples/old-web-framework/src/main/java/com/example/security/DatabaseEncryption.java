package com.example.security;

import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.spec.SecretKeySpec;
import java.security.MessageDigest;
import java.util.Base64;

/**
 * Database Encryption Utilities
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */
public class DatabaseEncryption {

    // VULNERABLE: Using 3DES algorithm (deprecated)
    private static final String ALGORITHM = "DESede";

    // VULNERABLE: Hardcoded encryption key
    private static final byte[] HARDCODED_KEY = "SecretKey123456789012345".getBytes();

    /**
     * Encrypt data for database storage using 3DES
     * VULNERABLE: 3DES is deprecated and vulnerable
     */
    public static String encryptData(String plaintext) throws Exception {
        // VULNERABLE: Using 3DES (Triple DES) algorithm
        SecretKeySpec keySpec = new SecretKeySpec(HARDCODED_KEY, ALGORITHM);
        Cipher cipher = Cipher.getInstance(ALGORITHM);
        cipher.init(Cipher.ENCRYPT_MODE, keySpec);

        byte[] encrypted = cipher.doFinal(plaintext.getBytes());
        return Base64.getEncoder().encodeToString(encrypted);
    }

    /**
     * Decrypt data from database using 3DES
     * VULNERABLE: 3DES decryption
     */
    public static String decryptData(String ciphertext) throws Exception {
        // VULNERABLE: Using 3DES algorithm
        SecretKeySpec keySpec = new SecretKeySpec(HARDCODED_KEY, ALGORITHM);
        Cipher cipher = Cipher.getInstance(ALGORITHM);
        cipher.init(Cipher.DECRYPT_MODE, keySpec);

        byte[] decoded = Base64.getDecoder().decode(ciphertext);
        byte[] decrypted = cipher.doFinal(decoded);
        return new String(decrypted);
    }

    /**
     * Generate 3DES key for database encryption
     * VULNERABLE: Generates 3DES keys
     */
    public static SecretKey generateDatabaseKey() throws Exception {
        // VULNERABLE: Generating 3DES key
        KeyGenerator keyGen = KeyGenerator.getInstance(ALGORITHM);
        keyGen.init(168); // 3DES key size
        return keyGen.generateKey();
    }

    /**
     * Encrypt sensitive field using DES (even weaker)
     * VULNERABLE: Using single DES
     */
    public static String encryptSensitiveField(String data) throws Exception {
        // VULNERABLE: Single DES is extremely weak
        SecretKeySpec keySpec = new SecretKeySpec(
            HARDCODED_KEY, 0, 8, "DES"
        );
        Cipher cipher = Cipher.getInstance("DES");
        cipher.init(Cipher.ENCRYPT_MODE, keySpec);

        byte[] encrypted = cipher.doFinal(data.getBytes());
        return Base64.getEncoder().encodeToString(encrypted);
    }

    /**
     * Legacy encryption method for backward compatibility
     * VULNERABLE: Multiple encryption weaknesses
     */
    public static String legacyEncrypt(String plaintext) throws Exception {
        // VULNERABLE: 3DES with hardcoded key
        return encryptData(plaintext);
    }

    /**
     * Hash password for storage
     * VULNERABLE: Using MD5 for password hashing
     */
    public static String hashPassword(String password) throws Exception {
        // VULNERABLE: MD5 hash for passwords (no salt)
        MessageDigest md = MessageDigest.getInstance("MD5");
        byte[] hash = md.digest(password.getBytes());
        return Base64.getEncoder().encodeToString(hash);
    }
}
