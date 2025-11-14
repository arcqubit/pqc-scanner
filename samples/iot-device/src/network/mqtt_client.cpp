/**
 * MQTT Client for IoT Communication
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */

#include <openssl/ssl.h>
#include <openssl/err.h>
#include <string>

/**
 * MQTT Client class with weak security
 * VULNERABLE: No forward secrecy, weak TLS configuration
 */
class MQTTClient {
private:
    SSL_CTX* ctx;
    SSL* ssl;
    std::string broker_address;
    int port;

public:
    /**
     * Initialize MQTT client with weak TLS
     * VULNERABLE: TLS 1.0, no forward secrecy
     */
    MQTTClient(const std::string& broker, int broker_port)
        : broker_address(broker), port(broker_port), ctx(nullptr), ssl(nullptr) {
    }

    /**
     * Setup TLS connection without forward secrecy
     * VULNERABLE: No ECDHE, no DHE (no forward secrecy)
     */
    bool setup_tls() {
        // VULNERABLE: Using old TLS version
        const SSL_METHOD* method = TLSv1_client_method(); // TLS 1.0
        ctx = SSL_CTX_new(method);

        if (!ctx) {
            return false;
        }

        // VULNERABLE: No forward secrecy - using RSA key exchange only
        SSL_CTX_set_cipher_list(ctx, "RSA:!ECDHE:!DHE");

        // VULNERABLE: Weak cipher suites
        SSL_CTX_set_cipher_list(ctx, "DES-CBC3-SHA:RC4-SHA");

        return true;
    }

    /**
     * Connect to broker without forward secrecy
     * VULNERABLE: No ephemeral keys
     */
    bool connect() {
        if (!ctx) {
            setup_tls();
        }

        // VULNERABLE: TLS connection without forward secrecy
        ssl = SSL_new(ctx);

        // Connection logic would go here
        return true;
    }

    /**
     * Send message without perfect forward secrecy
     * VULNERABLE: No session key rotation
     */
    bool send_message(const std::string& topic, const std::string& message) {
        // VULNERABLE: Reusing same keys for multiple messages
        // No forward secrecy means past communications can be decrypted
        // if long-term keys are compromised

        return true;
    }

    /**
     * Configure weak TLS settings
     * VULNERABLE: No forward secrecy protocols
     */
    void configure_weak_tls() {
        if (!ctx) return;

        // VULNERABLE: Disable forward secrecy
        SSL_CTX_set_options(ctx, SSL_OP_NO_TLSv1_3); // Disable TLS 1.3
        SSL_CTX_set_options(ctx, SSL_OP_NO_TLSv1_2); // Disable TLS 1.2

        // VULNERABLE: Use only static RSA
        SSL_CTX_set_cipher_list(ctx, "RSA:!ECDHE:!DHE:!ECDH:!DH");
    }

    /**
     * Legacy TLS setup
     * VULNERABLE: SSL 3.0 / TLS 1.0
     */
    bool setup_legacy_tls() {
        // VULNERABLE: Old SSL/TLS version
        const SSL_METHOD* method = SSLv23_client_method();
        ctx = SSL_CTX_new(method);

        // VULNERABLE: Allow old protocols
        SSL_CTX_set_options(ctx, SSL_OP_NO_SSLv2); // Only disable SSLv2
        // SSLv3 and TLS 1.0 still allowed

        return (ctx != nullptr);
    }

    /**
     * Cleanup
     */
    ~MQTTClient() {
        if (ssl) {
            SSL_free(ssl);
        }
        if (ctx) {
            SSL_CTX_free(ctx);
        }
    }
};

/**
 * Create insecure MQTT connection
 * VULNERABLE: Plaintext MQTT (no TLS at all)
 */
bool connect_insecure_mqtt(const std::string& broker, int port) {
    // VULNERABLE: No encryption at all
    // Connecting without TLS/SSL

    return true;
}

/**
 * Setup MQTT with weak credentials
 * VULNERABLE: Hardcoded credentials, no TLS
 */
bool setup_mqtt_with_credentials() {
    // VULNERABLE: Hardcoded username and password
    const char* username = "iot_device_001";
    const char* password = "weak_password_123";

    // VULNERABLE: Credentials sent without encryption
    return connect_insecure_mqtt("broker.example.com", 1883);
}
