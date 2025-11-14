package com.example.controllers;

import com.example.config.CookieConfig;
import org.springframework.web.bind.annotation.*;

import javax.servlet.http.Cookie;
import javax.servlet.http.HttpServletResponse;
import java.security.MessageDigest;
import java.util.Base64;

/**
 * Authentication Controller
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */
@RestController
@RequestMapping("/api/auth")
public class AuthController {

    /**
     * User login endpoint
     * VULNERABLE: MD5 password hashing
     */
    @PostMapping("/login")
    public String login(
        @RequestParam String username,
        @RequestParam String password,
        HttpServletResponse response
    ) throws Exception {
        // VULNERABLE: MD5 hash for password verification
        MessageDigest digest = MessageDigest.getInstance("MD5");
        byte[] passwordHash = digest.digest(password.getBytes());
        String hashString = Base64.getEncoder().encodeToString(passwordHash);

        // VULNERABLE: Create insecure session cookie
        Cookie sessionCookie = CookieConfig.generateSessionCookie(username);
        response.addCookie(sessionCookie);

        return "Login successful: " + hashString;
    }

    /**
     * Generate API token
     * VULNERABLE: SHA-1 for token generation
     */
    @PostMapping("/token")
    public String generateToken(@RequestParam String userId) throws Exception {
        // VULNERABLE: SHA-1 hash for API token
        MessageDigest digest = MessageDigest.getInstance("SHA-1");
        String tokenData = userId + System.currentTimeMillis();
        byte[] tokenHash = digest.digest(tokenData.getBytes());

        return Base64.getEncoder().encodeToString(tokenHash);
    }

    /**
     * Verify authentication token
     * VULNERABLE: Weak token verification
     */
    @GetMapping("/verify")
    public boolean verifyToken(@RequestParam String token) throws Exception {
        // VULNERABLE: MD5 hash for verification
        MessageDigest digest = MessageDigest.getInstance("MD5");
        byte[] hash = digest.digest(token.getBytes());

        // Simplified verification (would check against database in real app)
        return hash.length == 16; // MD5 produces 16 bytes
    }

    /**
     * Change password
     * VULNERABLE: MD5 password storage
     */
    @PostMapping("/change-password")
    public String changePassword(
        @RequestParam String oldPassword,
        @RequestParam String newPassword
    ) throws Exception {
        // VULNERABLE: MD5 hash for both passwords
        MessageDigest digest = MessageDigest.getInstance("MD5");

        byte[] oldHash = digest.digest(oldPassword.getBytes());
        digest.reset();
        byte[] newHash = digest.digest(newPassword.getBytes());

        String newHashString = Base64.getEncoder().encodeToString(newHash);
        return "Password changed: " + newHashString;
    }
}
