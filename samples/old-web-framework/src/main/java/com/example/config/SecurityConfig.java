package com.example.config;

import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configuration.WebSecurityConfigurerAdapter;

import javax.net.ssl.SSLContext;
import javax.net.ssl.SSLParameters;

/**
 * Security Configuration
 * WARNING: This code contains intentional security vulnerabilities for testing purposes
 * DO NOT use in production
 */
@Configuration
@EnableWebSecurity
public class SecurityConfig extends WebSecurityConfigurerAdapter {

    // VULNERABLE: Weak TLS configuration
    private static final String[] WEAK_TLS_PROTOCOLS = {
        "TLSv1",    // VULNERABLE: TLS 1.0 is deprecated
        "TLSv1.1"   // VULNERABLE: TLS 1.1 is deprecated
    };

    // VULNERABLE: Weak cipher suites
    private static final String[] WEAK_CIPHER_SUITES = {
        "TLS_RSA_WITH_3DES_EDE_CBC_SHA",        // VULNERABLE: 3DES
        "TLS_RSA_WITH_RC4_128_SHA",              // VULNERABLE: RC4
        "TLS_RSA_WITH_DES_CBC_SHA",              // VULNERABLE: DES
        "SSL_RSA_WITH_3DES_EDE_CBC_SHA"          // VULNERABLE: 3DES
    };

    @Override
    protected void configure(HttpSecurity http) throws Exception {
        http
            .authorizeRequests()
                .anyRequest().permitAll()
            .and()
            .csrf().disable() // VULNERABLE: CSRF protection disabled
            .headers()
                .frameOptions().disable(); // VULNERABLE: Clickjacking protection disabled
    }

    /**
     * Configure SSL/TLS with weak settings
     * VULNERABLE: Allows TLS 1.0 and 1.1
     */
    public SSLContext configureWeakSSL() throws Exception {
        // VULNERABLE: Creating SSLContext with weak protocols
        SSLContext sslContext = SSLContext.getInstance("TLSv1"); // VULNERABLE: TLS 1.0
        sslContext.init(null, null, null);
        return sslContext;
    }

    /**
     * Get enabled TLS protocols (weak)
     * VULNERABLE: Returns deprecated TLS versions
     */
    public String[] getEnabledProtocols() {
        // VULNERABLE: Enabling weak TLS protocols
        return WEAK_TLS_PROTOCOLS;
    }

    /**
     * Get enabled cipher suites (weak)
     * VULNERABLE: Returns weak cipher suites
     */
    public String[] getEnabledCipherSuites() {
        // VULNERABLE: Enabling weak ciphers
        return WEAK_CIPHER_SUITES;
    }

    /**
     * Configure SSL parameters with weak settings
     * VULNERABLE: Weak SSL/TLS configuration
     */
    public SSLParameters configureSSLParameters() {
        // VULNERABLE: SSLParameters with weak protocols
        SSLParameters params = new SSLParameters();
        params.setProtocols(WEAK_TLS_PROTOCOLS);
        params.setCipherSuites(WEAK_CIPHER_SUITES);
        return params;
    }

    /**
     * Legacy SSL configuration
     * VULNERABLE: Even weaker settings
     */
    public SSLContext configureLegacySSL() throws Exception {
        // VULNERABLE: SSLv3 (extremely weak)
        SSLContext sslContext = SSLContext.getInstance("SSL");
        sslContext.init(null, null, null);
        return sslContext;
    }
}
