import com.example.security.*;
import com.example.config.*;
import org.junit.jupiter.api.Test;

import java.security.KeyPair;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Security Tests
 * Tests for intentionally vulnerable crypto implementations
 */
public class SecurityTests {

    @Test
    public void testDatabaseEncryption() throws Exception {
        String plaintext = "Sensitive data";
        String encrypted = DatabaseEncryption.encryptData(plaintext);
        String decrypted = DatabaseEncryption.decryptData(encrypted);

        assertNotNull(encrypted);
        assertEquals(plaintext, decrypted);
    }

    @Test
    public void testPasswordHashing() throws Exception {
        String password = "password123";
        String hash = DatabaseEncryption.hashPassword(password);

        assertNotNull(hash);
        assertTrue(hash.length() > 0);
    }

    @Test
    public void testFileIntegrity() throws Exception {
        String data = "File content";
        String checksum = FileIntegrity.calculateChecksum(data);

        assertNotNull(checksum);
        assertEquals(40, checksum.length()); // SHA-1 produces 40 hex chars
    }

    @Test
    public void testCacheKeyGeneration() throws Exception {
        String content = "Cache content";
        String key = FileIntegrity.generateCacheKey(content);

        assertNotNull(key);
        assertEquals(32, key.length()); // MD5 produces 32 hex chars
    }

    @Test
    public void testDSASignature() throws Exception {
        KeyPair keyPair = SignatureService.generateSigningKeys();
        String data = "Test data";

        String signature = SignatureService.signData(
            data.getBytes(),
            keyPair.getPrivate()
        );

        boolean verified = SignatureService.verifySignature(
            data.getBytes(),
            signature,
            keyPair.getPublic()
        );

        assertNotNull(signature);
        assertTrue(verified);
    }

    @Test
    public void testRSAKeyGeneration() throws Exception {
        KeyPair keyPair = SignatureService.generateLegacyRSAKeys();

        assertNotNull(keyPair);
        assertNotNull(keyPair.getPublic());
        assertNotNull(keyPair.getPrivate());
    }

    @Test
    public void testCookieEncryption() throws Exception {
        String value = "cookie_value";
        String encrypted = CookieConfig.encryptCookieValue(value);
        String decrypted = CookieConfig.decryptCookieValue(encrypted);

        assertEquals(value, decrypted);
    }

    @Test
    public void testCookieSigning() throws Exception {
        String cookieValue = "test_cookie";
        String signature = CookieConfig.signCookie(cookieValue);

        assertNotNull(signature);
        assertTrue(CookieConfig.verifyCookieSignature(cookieValue, signature));
    }

    @Test
    public void testWeakSSLConfiguration() throws Exception {
        SecurityConfig config = new SecurityConfig();
        String[] protocols = config.getEnabledProtocols();

        assertNotNull(protocols);
        assertTrue(protocols.length > 0);
    }

    @Test
    public void test3DESEncryption() throws Exception {
        String data = "Secret";
        String encrypted = DatabaseEncryption.legacyEncrypt(data);

        assertNotNull(encrypted);
        assertTrue(encrypted.length() > 0);
    }
}
