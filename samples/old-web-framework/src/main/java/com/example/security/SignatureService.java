package com.example.security;

import java.security.*;
import java.util.Base64;

/**
 * Digital Signature Service
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */
public class SignatureService {

    // VULNERABLE: Using DSA algorithm (quantum-vulnerable, deprecated)
    private static final String SIGNATURE_ALGORITHM = "DSA";
    private static final int DSA_KEY_SIZE = 1024; // VULNERABLE: Too small

    /**
     * Generate DSA key pair for signing
     * VULNERABLE: DSA-1024 is too weak
     */
    public static KeyPair generateSigningKeys() throws Exception {
        // VULNERABLE: DSA with 1024-bit key size
        KeyPairGenerator keyGen = KeyPairGenerator.getInstance(SIGNATURE_ALGORITHM);
        keyGen.initialize(DSA_KEY_SIZE);
        return keyGen.generateKeyPair();
    }

    /**
     * Sign data using DSA
     * VULNERABLE: DSA signature algorithm
     */
    public static String signData(byte[] data, PrivateKey privateKey) throws Exception {
        // VULNERABLE: DSA signature with SHA-1
        Signature signature = Signature.getInstance("SHA1withDSA");
        signature.initSign(privateKey);
        signature.update(data);

        byte[] signatureBytes = signature.sign();
        return Base64.getEncoder().encodeToString(signatureBytes);
    }

    /**
     * Verify DSA signature
     * VULNERABLE: DSA signature verification
     */
    public static boolean verifySignature(
        byte[] data,
        String signatureStr,
        PublicKey publicKey
    ) throws Exception {
        // VULNERABLE: DSA verification with SHA-1
        Signature signature = Signature.getInstance("SHA1withDSA");
        signature.initVerify(publicKey);
        signature.update(data);

        byte[] signatureBytes = Base64.getDecoder().decode(signatureStr);
        return signature.verify(signatureBytes);
    }

    /**
     * Sign transaction using DSA
     * VULNERABLE: DSA for transaction signatures
     */
    public static String signTransaction(String transaction, PrivateKey key) throws Exception {
        // VULNERABLE: DSA signature
        return signData(transaction.getBytes(), key);
    }

    /**
     * Generate weak RSA key pair for legacy support
     * VULNERABLE: RSA-1024 is too small
     */
    public static KeyPair generateLegacyRSAKeys() throws Exception {
        // VULNERABLE: RSA-1024 key generation
        KeyPairGenerator keyGen = KeyPairGenerator.getInstance("RSA");
        keyGen.initialize(1024); // VULNERABLE: Too small
        return keyGen.generateKeyPair();
    }

    /**
     * Sign with legacy RSA-SHA1
     * VULNERABLE: RSA with SHA-1
     */
    public static String signWithRSA(byte[] data, PrivateKey privateKey) throws Exception {
        // VULNERABLE: RSA signature with SHA-1
        Signature signature = Signature.getInstance("SHA1withRSA");
        signature.initSign(privateKey);
        signature.update(data);

        byte[] signatureBytes = signature.sign();
        return Base64.getEncoder().encodeToString(signatureBytes);
    }

    /**
     * Hash data for signing
     * VULNERABLE: SHA-1 hash
     */
    public static byte[] hashForSigning(String data) throws Exception {
        // VULNERABLE: SHA-1 hash algorithm
        MessageDigest digest = MessageDigest.getInstance("SHA-1");
        return digest.digest(data.getBytes());
    }
}
