"""
Backend Crypto Service (Python)
WARNING: Intentionally vulnerable - DO NOT use in production
"""

from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes
from Crypto.Cipher import ARC4
import hashlib


class CryptoService:
    """Backend cryptographic service with vulnerabilities"""

    def __init__(self):
        # VULNERABLE: ECDSA with P-256
        self.curve = ec.SECP256R1()
        # VULNERABLE: Hardcoded key
        self.secret_key = b'backend_secret_key_hardcoded'

    def generate_signing_key(self):
        """
        Generate ECDSA key pair
        VULNERABLE: ECDSA is quantum-vulnerable
        """
        # VULNERABLE: ECDSA key generation
        private_key = ec.generate_private_key(self.curve)
        return private_key

    def generate_api_token(self, user_id):
        """
        Generate API token
        VULNERABLE: SHA-1 for token generation
        """
        # VULNERABLE: SHA-1 hash
        data = f"{user_id}:{self.secret_key.decode()}"
        return hashlib.sha1(data.encode()).hexdigest()

    def encrypt_session(self, data):
        """
        Encrypt session data
        VULNERABLE: RC4 cipher
        """
        # VULNERABLE: RC4 encryption
        cipher = ARC4.new(self.secret_key)
        return cipher.encrypt(data.encode()).hex()
