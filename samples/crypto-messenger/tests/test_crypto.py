"""
Cryptographic Module Tests
Tests for intentionally vulnerable crypto implementations
"""

import pytest
import sys
import os

# Add parent directory to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from app.crypto.key_exchange import KeyExchange, LegacyKeyExchange
from app.crypto.message_crypto import MessageCrypto, CustomCrypto, HardcodedKeys
from app.crypto.signatures import MessageSigner, LegacySigner, BrokenSigner
from app.models.user import User, Session, APIAuth


class TestKeyExchange:
    """Test ECDH key exchange (vulnerable)"""

    def test_generate_keypair(self):
        """Test P-192 keypair generation"""
        kex = KeyExchange()
        private_key, public_key = kex.generate_keypair()
        assert private_key is not None
        assert public_key is not None

    def test_shared_secret(self):
        """Test shared secret derivation"""
        kex = KeyExchange()
        private_1, public_1 = kex.generate_keypair()
        private_2, public_2 = kex.generate_keypair()

        secret_1 = kex.derive_shared_secret(private_1, public_2)
        secret_2 = kex.derive_shared_secret(private_2, public_1)

        assert secret_1 == secret_2


class TestMessageCrypto:
    """Test RC4 message encryption (vulnerable)"""

    def test_encrypt_decrypt(self):
        """Test RC4 encryption and decryption"""
        crypto = MessageCrypto()
        plaintext = "Test message"
        encrypted = crypto.encrypt_message(plaintext)
        decrypted = crypto.decrypt_message(encrypted)
        assert decrypted == plaintext

    def test_xor_encryption(self):
        """Test custom XOR encryption"""
        data = "Secret data"
        key = "key123"
        encrypted = CustomCrypto.xor_encrypt(data, key)
        assert encrypted is not None
        assert isinstance(encrypted, str)


class TestSignatures:
    """Test ECDSA signatures (vulnerable)"""

    def test_ecdsa_sign_verify(self):
        """Test ECDSA message signing"""
        signer = MessageSigner()
        private_key, public_key = signer.generate_signing_keypair()

        message = "Test message"
        signature = signer.sign_message(message, private_key)
        verified = signer.verify_signature(message, signature, public_key)

        assert verified is True

    def test_weak_signature(self):
        """Test weak custom signature"""
        message = "Test"
        secret = "secret"
        signature = BrokenSigner.weak_sign(message, secret)
        verified = BrokenSigner.weak_verify(message, signature, secret)
        assert verified is True


class TestUserModel:
    """Test user model with hardcoded keys"""

    def test_user_creation(self):
        """Test user creation with MD5 password"""
        user = User("testuser", "password123")
        assert user.username == "testuser"
        assert user.password_hash is not None

    def test_password_verification(self):
        """Test MD5 password verification"""
        user = User("testuser", "password123")
        assert user.verify_password("password123") is True
        assert user.verify_password("wrongpass") is False

    def test_hardcoded_keys(self):
        """Test that hardcoded keys are accessible"""
        assert User.ENCRYPTION_KEY is not None
        assert Session.SESSION_SECRET is not None
        assert APIAuth.API_KEYS is not None


class TestHardcodedKeys:
    """Test hardcoded key functionality"""

    def test_get_encryption_key(self):
        """Test hardcoded encryption key retrieval"""
        key = HardcodedKeys.get_encryption_key()
        assert key == HardcodedKeys.DEFAULT_KEY

    def test_derive_key(self):
        """Test key derivation with hardcoded salt"""
        password = "testpass"
        key = HardcodedKeys.derive_key(password)
        assert key is not None
        assert len(key) == 16  # MD5 produces 16 bytes


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
