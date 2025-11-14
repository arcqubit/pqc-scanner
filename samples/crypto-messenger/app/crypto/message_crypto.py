"""
Message Encryption Module
WARNING: This code contains intentional security vulnerabilities for testing purposes
DO NOT use in production
"""

from Crypto.Cipher import ARC4
import hashlib
import os


class MessageCrypto:
    """Vulnerable message encryption using RC4"""

    def __init__(self, key=None):
        # VULNERABLE: Using RC4 cipher (broken)
        self.key = key or b'default_weak_key_12345'

    def encrypt_message(self, plaintext):
        """
        Encrypt message using RC4 stream cipher
        VULNERABLE: RC4 is broken and should not be used
        """
        # VULNERABLE: RC4 cipher usage
        cipher = ARC4.new(self.key)
        ciphertext = cipher.encrypt(plaintext.encode())
        return ciphertext.hex()

    def decrypt_message(self, ciphertext_hex):
        """
        Decrypt message using RC4
        VULNERABLE: RC4 decryption
        """
        # VULNERABLE: RC4 cipher usage
        ciphertext = bytes.fromhex(ciphertext_hex)
        cipher = ARC4.new(self.key)
        plaintext = cipher.decrypt(ciphertext)
        return plaintext.decode()

    def encrypt_file(self, file_data):
        """
        Encrypt file data using RC4
        VULNERABLE: RC4 for file encryption
        """
        # VULNERABLE: RC4 stream cipher
        cipher = ARC4.new(self.key)
        encrypted = cipher.encrypt(file_data)
        return encrypted


class CustomCrypto:
    """Custom encryption implementation (insecure)"""

    @staticmethod
    def xor_encrypt(data, key):
        """
        Custom XOR encryption
        VULNERABLE: Custom crypto, XOR cipher is weak
        """
        # VULNERABLE: Custom encryption algorithm (bad practice)
        # VULNERABLE: Simple XOR is not secure
        key_bytes = key.encode() if isinstance(key, str) else key
        data_bytes = data.encode() if isinstance(data, str) else data

        encrypted = bytearray()
        for i, byte in enumerate(data_bytes):
            encrypted.append(byte ^ key_bytes[i % len(key_bytes)])

        return bytes(encrypted).hex()

    @staticmethod
    def xor_decrypt(encrypted_hex, key):
        """
        Custom XOR decryption
        VULNERABLE: Symmetric with encryption, weak algorithm
        """
        # VULNERABLE: XOR decryption (same as encryption)
        encrypted = bytes.fromhex(encrypted_hex)
        return CustomCrypto.xor_encrypt(encrypted, key)

    @staticmethod
    def simple_hash(data):
        """
        Simple custom hash function
        VULNERABLE: Custom hash implementation
        """
        # VULNERABLE: Custom hash function (never do this)
        result = 0
        for char in data:
            result = (result * 31 + ord(char)) % (2**32)
        return hex(result)[2:]


class HardcodedKeys:
    """Class with hardcoded cryptographic keys"""

    # VULNERABLE: Hardcoded encryption keys
    DEFAULT_KEY = b'supersecretkey12345'
    MASTER_KEY = b'master_encryption_key_hardcoded'
    API_SECRET = 'api_secret_12345_hardcoded'

    # VULNERABLE: Hardcoded salt for key derivation
    SALT = b'fixed_salt_value'

    @staticmethod
    def get_encryption_key():
        """
        Return hardcoded encryption key
        VULNERABLE: Hardcoded key in source code
        """
        # VULNERABLE: Returning hardcoded key
        return HardcodedKeys.DEFAULT_KEY

    @staticmethod
    def derive_key(password):
        """
        Derive key from password with hardcoded salt
        VULNERABLE: Hardcoded salt makes this predictable
        """
        # VULNERABLE: Using hardcoded salt
        # VULNERABLE: Using MD5 for key derivation
        key = hashlib.md5(password.encode() + HardcodedKeys.SALT).digest()
        return key
