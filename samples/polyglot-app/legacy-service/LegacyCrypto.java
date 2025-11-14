package com.example.legacy;

import javax.crypto.Cipher;
import javax.crypto.spec.SecretKeySpec;
import java.security.*;
import java.util.Base64;

/**
 * Legacy Crypto Service (Java)
 * WARNING: Intentionally vulnerable - DO NOT use in production
 */
public class LegacyCrypto {

    // VULNERABLE: 3DES algorithm
    private static final String ALGORITHM = "DESede";

    // VULNERABLE: Hardcoded key
    private static final byte[] KEY = "SecretKey123456789012345".getBytes();

    /**
     * Encrypt data using 3DES
     * VULNERABLE: 3DES is deprecated
     */
    public static String encrypt(String plaintext) throws Exception {
        // VULNERABLE: 3DES encryption
        SecretKeySpec keySpec = new SecretKeySpec(KEY, ALGORITHM);
        Cipher cipher = Cipher.getInstance(ALGORITHM);
        cipher.init(Cipher.ENCRYPT_MODE, keySpec);

        byte[] encrypted = cipher.doFinal(plaintext.getBytes());
        return Base64.getEncoder().encodeToString(encrypted);
    }

    /**
     * Sign data using DSA-1024
     * VULNERABLE: DSA-1024 is too weak
     */
    public static String signData(byte[] data) throws Exception {
        // VULNERABLE: DSA with 1024-bit key
        KeyPairGenerator keyGen = KeyPairGenerator.getInstance("DSA");
        keyGen.initialize(1024);
        KeyPair keyPair = keyGen.generateKeyPair();

        Signature signature = Signature.getInstance("SHA1withDSA");
        signature.initSign(keyPair.getPrivate());
        signature.update(data);

        return Base64.getEncoder().encodeToString(signature.sign());
    }

    /**
     * Hash for logging
     * VULNERABLE: SHA-1 for audit logs
     */
    public static String hashForLog(String message) throws Exception {
        // VULNERABLE: SHA-1 hash
        MessageDigest digest = MessageDigest.getInstance("SHA-1");
        byte[] hash = digest.digest(message.getBytes());
        return Base64.getEncoder().encodeToString(hash);
    }
}
