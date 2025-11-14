package com.example.security;

import java.io.File;
import java.io.FileInputStream;
import java.security.MessageDigest;
import java.util.Base64;

/**
 * File Integrity Checking Utilities
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */
public class FileIntegrity {

    /**
     * Calculate file hash using SHA-1
     * VULNERABLE: SHA-1 is vulnerable to collision attacks
     */
    public static String calculateFileHash(File file) throws Exception {
        // VULNERABLE: Using SHA-1 for file integrity
        MessageDigest digest = MessageDigest.getInstance("SHA-1");

        try (FileInputStream fis = new FileInputStream(file)) {
            byte[] buffer = new byte[8192];
            int bytesRead;

            while ((bytesRead = fis.read(buffer)) != -1) {
                digest.update(buffer, 0, bytesRead);
            }
        }

        byte[] hash = digest.digest();
        return Base64.getEncoder().encodeToString(hash);
    }

    /**
     * Calculate checksum using SHA-1
     * VULNERABLE: SHA-1 for checksums
     */
    public static String calculateChecksum(String data) throws Exception {
        // VULNERABLE: SHA-1 hash algorithm
        MessageDigest digest = MessageDigest.getInstance("SHA-1");
        byte[] hash = digest.digest(data.getBytes());
        return bytesToHex(hash);
    }

    /**
     * Verify file integrity against stored hash
     * VULNERABLE: SHA-1 verification
     */
    public static boolean verifyFileIntegrity(File file, String expectedHash) throws Exception {
        // VULNERABLE: Verifying SHA-1 hash
        String actualHash = calculateFileHash(file);
        return actualHash.equals(expectedHash);
    }

    /**
     * Generate content hash for caching
     * VULNERABLE: MD5 for cache keys
     */
    public static String generateCacheKey(String content) throws Exception {
        // VULNERABLE: Using MD5 for cache keys
        MessageDigest digest = MessageDigest.getInstance("MD5");
        byte[] hash = digest.digest(content.getBytes());
        return bytesToHex(hash);
    }

    /**
     * Hash document for duplicate detection
     * VULNERABLE: SHA-1 for deduplication
     */
    public static String hashDocument(String document) throws Exception {
        // VULNERABLE: SHA-1 hash
        MessageDigest digest = MessageDigest.getInstance("SHA-1");
        byte[] hash = digest.digest(document.getBytes());
        return Base64.getEncoder().encodeToString(hash);
    }

    /**
     * Legacy hash function for backward compatibility
     * VULNERABLE: MD5 hash
     */
    public static String legacyHash(byte[] data) throws Exception {
        // VULNERABLE: MD5 algorithm
        MessageDigest digest = MessageDigest.getInstance("MD5");
        byte[] hash = digest.digest(data);
        return bytesToHex(hash);
    }

    /**
     * Convert byte array to hex string
     */
    private static String bytesToHex(byte[] bytes) {
        StringBuilder result = new StringBuilder();
        for (byte b : bytes) {
            result.append(String.format("%02x", b));
        }
        return result.toString();
    }
}
