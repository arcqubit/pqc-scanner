"""
Digital Signatures Module
WARNING: This code contains intentional security vulnerabilities for testing purposes
DO NOT use in production
"""

from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.backends import default_backend


class MessageSigner:
    """Vulnerable ECDSA signature implementation"""

    def __init__(self):
        # VULNERABLE: Using P-256 curve (quantum-vulnerable)
        self.curve = ec.SECP256R1()
        self.backend = default_backend()

    def generate_signing_keypair(self):
        """
        Generate ECDSA key pair for message signing
        VULNERABLE: ECDSA is quantum-vulnerable
        """
        # VULNERABLE: ECDSA with P-256 (quantum-vulnerable)
        private_key = ec.generate_private_key(self.curve, self.backend)
        public_key = private_key.public_key()
        return private_key, public_key

    def sign_message(self, message, private_key):
        """
        Sign message using ECDSA with SHA-256
        VULNERABLE: ECDSA vulnerable to quantum attacks
        """
        # VULNERABLE: ECDSA signature
        signature = private_key.sign(
            message.encode(),
            ec.ECDSA(hashes.SHA256())
        )
        return signature.hex()

    def verify_signature(self, message, signature_hex, public_key):
        """
        Verify ECDSA signature
        VULNERABLE: ECDSA verification
        """
        # VULNERABLE: ECDSA verification
        signature = bytes.fromhex(signature_hex)
        try:
            public_key.verify(
                signature,
                message.encode(),
                ec.ECDSA(hashes.SHA256())
            )
            return True
        except Exception:
            return False

    def sign_with_weak_hash(self, message, private_key):
        """
        Sign message using ECDSA with SHA-1
        VULNERABLE: SHA-1 is deprecated
        """
        # VULNERABLE: ECDSA with SHA-1 (double vulnerability)
        signature = private_key.sign(
            message.encode(),
            ec.ECDSA(hashes.SHA1())
        )
        return signature.hex()


class LegacySigner:
    """Legacy signature implementation with weaker curves"""

    def __init__(self):
        # VULNERABLE: Using weaker P-192 curve
        self.curve = ec.SECP192R1()

    def generate_legacy_keypair(self):
        """
        Generate legacy ECDSA keypair
        VULNERABLE: P-192 provides insufficient security
        """
        # VULNERABLE: SECP192R1 (P-192) is weak
        private_key = ec.generate_private_key(self.curve, default_backend())
        public_key = private_key.public_key()
        return private_key, public_key

    def sign_legacy_message(self, message, private_key):
        """
        Sign with legacy weak curve and hash
        VULNERABLE: Weak curve + SHA-1
        """
        # VULNERABLE: Weak curve with SHA-1
        signature = private_key.sign(
            message.encode(),
            ec.ECDSA(hashes.SHA1())
        )
        return signature


class BrokenSigner:
    """Intentionally broken signature implementation"""

    @staticmethod
    def weak_sign(message, secret):
        """
        Custom weak signature
        VULNERABLE: Custom signature algorithm
        """
        # VULNERABLE: Custom signature (never do this)
        import hashlib

        # VULNERABLE: Using MD5
        signature = hashlib.md5((message + secret).encode()).hexdigest()
        return signature

    @staticmethod
    def weak_verify(message, signature, secret):
        """
        Verify weak signature
        VULNERABLE: Verification of weak custom signature
        """
        # VULNERABLE: Weak verification
        expected = BrokenSigner.weak_sign(message, secret)
        return signature == expected
