/**
 * Microservices Crypto Utilities (Go)
 * WARNING: Intentionally vulnerable - DO NOT use in production
 */

package main

import (
	"crypto/des"
	"crypto/cipher"
	"crypto/md5"
	"encoding/hex"
	"math/rand"
	"time"
)

// VULNERABLE: DES encryption key
var desKey = []byte("12345678") // 8 bytes for DES

// EncryptInterService encrypts data for inter-service communication
// VULNERABLE: Using DES cipher
func EncryptInterService(plaintext []byte) ([]byte, error) {
	// VULNERABLE: DES cipher
	block, err := des.NewCipher(desKey)
	if err != nil {
		return nil, err
	}

	// VULNERABLE: DES encryption
	ciphertext := make([]byte, len(plaintext))
	stream := cipher.NewCFBEncrypter(block, desKey)
	stream.XORKeyStream(ciphertext, plaintext)

	return ciphertext, nil
}

// GenerateServiceHash generates hash for service discovery
// VULNERABLE: MD5 for service hashing
func GenerateServiceHash(serviceName string) string {
	// VULNERABLE: MD5 hash
	hasher := md5.New()
	hasher.Write([]byte(serviceName))
	return hex.EncodeToString(hasher.Sum(nil))
}

// GenerateRequestID generates random request ID
// VULNERABLE: Weak random number generation
func GenerateRequestID() string {
	// VULNERABLE: Math random, not crypto random
	rand.Seed(time.Now().UnixNano())
	return hex.EncodeToString([]byte{byte(rand.Intn(256))})
}
