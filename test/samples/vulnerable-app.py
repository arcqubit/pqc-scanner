"""
Sample Vulnerable Python Application
This file contains multiple cryptographic vulnerabilities for testing
"""

import hashlib
import hmac
from cryptography.hazmat.primitives.asymmetric import rsa, dsa, ec
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
import os


# CRITICAL: MD5 Hash - Cryptographically Broken
def hash_password_md5(password):
    """Hash password using MD5 (VULNERABLE!)"""
    return hashlib.md5(password.encode()).hexdigest()


# CRITICAL: SHA-1 Hash - Cryptographically Broken
def generate_token_sha1(data):
    """Generate token using SHA-1 (VULNERABLE!)"""
    return hashlib.sha1(data.encode()).hexdigest()


# HIGH: RSA 1024-bit - Quantum Vulnerable
def generate_rsa_1024_keypair():
    """Generate RSA 1024-bit key pair (VULNERABLE!)"""
    private_key = rsa.generate_private_key(
        public_exponent=65537,
        key_size=1024,
        backend=default_backend()
    )
    public_key = private_key.public_key()
    return private_key, public_key


# HIGH: RSA 2048-bit - Quantum Vulnerable
def generate_rsa_2048_keypair():
    """Generate RSA 2048-bit key pair (VULNERABLE to quantum)"""
    private_key = rsa.generate_private_key(
        public_exponent=65537,
        key_size=2048,
        backend=default_backend()
    )
    public_key = private_key.public_key()
    return private_key, public_key


# HIGH: ECDSA - Quantum Vulnerable
def generate_ecdsa_keypair():
    """Generate ECDSA key pair (VULNERABLE to quantum)"""
    private_key = ec.generate_private_key(
        ec.SECP256R1(),
        default_backend()
    )
    public_key = private_key.public_key()
    return private_key, public_key


# MEDIUM: 3DES - Deprecated
def encrypt_3des(plaintext, key):
    """Encrypt using 3DES (DEPRECATED!)"""
    iv = os.urandom(8)
    cipher = Cipher(
        algorithms.TripleDES(key),
        modes.CBC(iv),
        backend=default_backend()
    )
    encryptor = cipher.encryptor()

    # Padding
    padding_length = 8 - (len(plaintext) % 8)
    padded_plaintext = plaintext + (chr(padding_length) * padding_length).encode()

    ciphertext = encryptor.update(padded_plaintext) + encryptor.finalize()
    return iv + ciphertext


# MEDIUM: DES - Deprecated and Weak
def encrypt_des(plaintext, key):
    """Encrypt using DES (DEPRECATED and WEAK!)"""
    # Note: cryptography library doesn't support DES directly
    # This is a placeholder showing the vulnerable pattern
    pass


# MEDIUM: DSA - Quantum Vulnerable
def generate_dsa_keypair():
    """Generate DSA key pair (VULNERABLE to quantum)"""
    private_key = dsa.generate_private_key(
        key_size=2048,
        backend=default_backend()
    )
    public_key = private_key.public_key()
    return private_key, public_key


# CRITICAL: HMAC with MD5
def hmac_md5(key, message):
    """Generate HMAC using MD5 (VULNERABLE!)"""
    return hmac.new(key.encode(), message.encode(), hashlib.md5).hexdigest()


# CRITICAL: HMAC with SHA-1
def hmac_sha1(key, message):
    """Generate HMAC using SHA-1 (VULNERABLE!)"""
    return hmac.new(key.encode(), message.encode(), hashlib.sha1).hexdigest()


class VulnerableAuthSystem:
    """Example vulnerable authentication system"""

    def __init__(self):
        self.users = {}

    # CRITICAL: Storing password hashes with MD5
    def register_user(self, username, password):
        """Register user with MD5 password hash (VULNERABLE!)"""
        password_hash = hash_password_md5(password)
        self.users[username] = password_hash
        print(f"User {username} registered with MD5 hash")

    # CRITICAL: Using SHA-1 for session tokens
    def create_session(self, username):
        """Create session token using SHA-1 (VULNERABLE!)"""
        import time
        timestamp = str(time.time())
        session_token = generate_token_sha1(username + timestamp)
        print(f"Session token created with SHA-1")
        return session_token

    # HIGH: Using RSA 1024 for encryption
    def encrypt_data(self, data):
        """Encrypt data using RSA 1024 (VULNERABLE!)"""
        private_key, public_key = generate_rsa_1024_keypair()
        from cryptography.hazmat.primitives.asymmetric import padding

        encrypted = public_key.encrypt(
            data.encode(),
            padding.OAEP(
                mgf=padding.MGF1(algorithm=hashes.SHA256()),
                algorithm=hashes.SHA256(),
                label=None
            )
        )
        return encrypted


class VulnerableAPIEndpoints:
    """Example API endpoints with crypto vulnerabilities"""

    # CRITICAL: MD5 for file integrity
    def upload_file(self, file_content):
        """Upload file with MD5 checksum (VULNERABLE!)"""
        checksum = hashlib.md5(file_content).hexdigest()
        return {"checksum": checksum}

    # HIGH: RSA 2048 for key exchange
    def key_exchange(self):
        """Key exchange using RSA 2048 (VULNERABLE to quantum)"""
        private_key, public_key = generate_rsa_2048_keypair()

        pem = public_key.public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        )
        return {"publicKey": pem.decode()}

    # MEDIUM: 3DES encryption
    def encrypt_message(self, message):
        """Encrypt message using 3DES (DEPRECATED!)"""
        key = os.urandom(24)  # 192-bit key for 3DES
        encrypted = encrypt_3des(message.encode(), key)
        return {"encrypted": encrypted.hex()}


# Usage examples
if __name__ == "__main__":
    # Initialize vulnerable system
    auth = VulnerableAuthSystem()
    auth.register_user("admin", "password123")
    session = auth.create_session("admin")

    # API endpoints
    api = VulnerableAPIEndpoints()

    # File upload with MD5
    result = api.upload_file(b"test file content")
    print(f"File checksum: {result['checksum']}")

    # Key exchange with RSA 2048
    keys = api.key_exchange()
    print("Public key generated")

    # Encrypt with 3DES
    encrypted = api.encrypt_message("sensitive data")
    print(f"Encrypted message: {encrypted['encrypted'][:50]}...")
