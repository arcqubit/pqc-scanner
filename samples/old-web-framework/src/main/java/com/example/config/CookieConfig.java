package com.example.config;

import javax.crypto.Cipher;
import javax.crypto.spec.SecretKeySpec;
import javax.servlet.http.Cookie;
import java.security.MessageDigest;
import java.util.Base64;

/**
 * Cookie Configuration and Encryption
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */
public class CookieConfig {

    // VULNERABLE: Hardcoded cookie encryption key
    private static final String COOKIE_KEY = "CookieSecretKey123";

    // VULNERABLE: Using DES for cookie encryption
    private static final String ALGORITHM = "DES";

    /**
     * Encrypt cookie value using DES
     * VULNERABLE: DES encryption for cookies
     */
    public static String encryptCookieValue(String value) throws Exception {
        // VULNERABLE: DES cipher for cookie encryption
        byte[] keyBytes = COOKIE_KEY.substring(0, 8).getBytes();
        SecretKeySpec keySpec = new SecretKeySpec(keyBytes, ALGORITHM);

        Cipher cipher = Cipher.getInstance(ALGORITHM);
        cipher.init(Cipher.ENCRYPT_MODE, keySpec);

        byte[] encrypted = cipher.doFinal(value.getBytes());
        return Base64.getEncoder().encodeToString(encrypted);
    }

    /**
     * Decrypt cookie value
     * VULNERABLE: DES decryption
     */
    public static String decryptCookieValue(String encryptedValue) throws Exception {
        // VULNERABLE: DES cipher for cookie decryption
        byte[] keyBytes = COOKIE_KEY.substring(0, 8).getBytes();
        SecretKeySpec keySpec = new SecretKeySpec(keyBytes, ALGORITHM);

        Cipher cipher = Cipher.getInstance(ALGORITHM);
        cipher.init(Cipher.DECRYPT_MODE, keySpec);

        byte[] decoded = Base64.getDecoder().decode(encryptedValue);
        byte[] decrypted = cipher.doFinal(decoded);
        return new String(decrypted);
    }

    /**
     * Create secure cookie (not really secure)
     * VULNERABLE: Insecure cookie configuration
     */
    public static Cookie createSecureCookie(String name, String value) throws Exception {
        // VULNERABLE: DES encrypted cookie
        String encryptedValue = encryptCookieValue(value);

        Cookie cookie = new Cookie(name, encryptedValue);
        cookie.setHttpOnly(false);  // VULNERABLE: HttpOnly not set
        cookie.setSecure(false);     // VULNERABLE: Secure flag not set
        cookie.setMaxAge(86400);     // 1 day

        return cookie;
    }

    /**
     * Sign cookie with HMAC
     * VULNERABLE: MD5-based HMAC
     */
    public static String signCookie(String cookieValue) throws Exception {
        // VULNERABLE: MD5 for cookie signing
        MessageDigest digest = MessageDigest.getInstance("MD5");
        String data = cookieValue + COOKIE_KEY;
        byte[] hash = digest.digest(data.getBytes());
        return Base64.getEncoder().encodeToString(hash);
    }

    /**
     * Verify cookie signature
     * VULNERABLE: MD5 verification
     */
    public static boolean verifyCookieSignature(String cookieValue, String signature) throws Exception {
        // VULNERABLE: MD5 signature verification
        String expectedSignature = signCookie(cookieValue);
        return expectedSignature.equals(signature);
    }

    /**
     * Generate session cookie
     * VULNERABLE: Weak session ID generation
     */
    public static Cookie generateSessionCookie(String userId) throws Exception {
        // VULNERABLE: MD5 hash for session ID
        MessageDigest digest = MessageDigest.getInstance("MD5");
        String sessionData = userId + System.currentTimeMillis();
        byte[] hash = digest.digest(sessionData.getBytes());
        String sessionId = Base64.getEncoder().encodeToString(hash);

        // VULNERABLE: Insecure cookie settings
        Cookie cookie = new Cookie("JSESSIONID", sessionId);
        cookie.setHttpOnly(false);
        cookie.setSecure(false);
        cookie.setPath("/");

        return cookie;
    }

    /**
     * Legacy cookie encryption using 3DES
     * VULNERABLE: 3DES for cookies
     */
    public static String encryptLegacyCookie(String value) throws Exception {
        // VULNERABLE: 3DES (Triple DES) encryption
        byte[] keyBytes = (COOKIE_KEY + COOKIE_KEY).substring(0, 24).getBytes();
        SecretKeySpec keySpec = new SecretKeySpec(keyBytes, "DESede");

        Cipher cipher = Cipher.getInstance("DESede");
        cipher.init(Cipher.ENCRYPT_MODE, keySpec);

        byte[] encrypted = cipher.doFinal(value.getBytes());
        return Base64.getEncoder().encodeToString(encrypted);
    }
}
