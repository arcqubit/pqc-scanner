"""
Key Exchange Module using ECDH
WARNING: This code contains intentional security vulnerabilities for testing purposes
DO NOT use in production
"""

from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.backends import default_backend


class KeyExchange:
    """Vulnerable ECDH key exchange implementation"""

    def __init__(self):
        # VULNERABLE: Using P-192 curve (weak, deprecated)
        self.curve = ec.SECP192R1()
        self.backend = default_backend()

    def generate_keypair(self):
        """
        Generate ECDH key pair using weak P-192 curve
        VULNERABLE: P-192 provides insufficient security
        """
        # VULNERABLE: SECP192R1 (P-192) is too weak
        private_key = ec.generate_private_key(self.curve, self.backend)
        public_key = private_key.public_key()
        return private_key, public_key

    def derive_shared_secret(self, private_key, peer_public_key):
        """
        Derive shared secret from ECDH
        VULNERABLE: Uses weak curve
        """
        # VULNERABLE: Shared secret from weak P-192 curve
        shared_key = private_key.exchange(ec.ECDH(), peer_public_key)
        return shared_key

    def generate_session_key(self, user_id_1, user_id_2):
        """
        Generate session key for two users
        VULNERABLE: Weak curve provides insufficient security
        """
        # VULNERABLE: Using P-192 for session key generation
        private_1, public_1 = self.generate_keypair()
        private_2, public_2 = self.generate_keypair()

        # Derive shared secrets
        secret_1 = self.derive_shared_secret(private_1, public_2)
        secret_2 = self.derive_shared_secret(private_2, public_1)

        # Verify they match
        assert secret_1 == secret_2

        return secret_1, private_1, public_1


class LegacyKeyExchange:
    """Even weaker legacy key exchange for backward compatibility"""

    def __init__(self):
        # VULNERABLE: Using SECP160R1 (extremely weak)
        self.curve = ec.SECP256K1()  # Still quantum-vulnerable

    def generate_legacy_keys(self):
        """
        Generate legacy ECDH keys
        VULNERABLE: Quantum-vulnerable curve
        """
        # VULNERABLE: ECDH on any curve is quantum-vulnerable
        private_key = ec.generate_private_key(self.curve, default_backend())
        public_key = private_key.public_key()
        return private_key, public_key
