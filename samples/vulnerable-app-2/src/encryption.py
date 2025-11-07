"""
Encryption Module - Contains 3DES and RC4 implementations
VULNERABILITIES:
- 3DES (quantum-vulnerable, deprecated)
- RC4 stream cipher (broken)
"""

from Crypto.Cipher import DES3, ARC4
from Crypto.Random import get_random_bytes
from Crypto.Util.Padding import pad, unpad
import base64
from typing import Tuple


class TripleDESEncryption:
    """
    3DES encryption implementation.

    SECURITY ISSUES:
    - 3DES has effective key length of only 112 bits (due to meet-in-the-middle)
    - Vulnerable to quantum attacks (Grover's algorithm reduces to 56-bit security)
    - Deprecated by NIST, should not be used in new applications
    - Block size of only 64 bits makes it vulnerable to birthday attacks

    Should use: AES-256 or post-quantum alternatives like Kyber
    """

    def __init__(self, key: bytes):
        """
        Initialize 3DES cipher.

        VULNERABLE: 3DES is deprecated and quantum-vulnerable.
        """
        # Ensure key is 24 bytes for 3DES
        if len(key) != 24:
            raise ValueError("3DES key must be 24 bytes")
        self.key = key

    def encrypt(self, plaintext: str) -> Tuple[str, str]:
        """
        Encrypt data using 3DES in CBC mode.

        VULNERABLE: 3DES provides only 112-bit security, reduced to
        56 bits against quantum computers.
        """
        # VULNERABILITY: Using 3DES cipher
        cipher = DES3.new(self.key, DES3.MODE_CBC)
        iv = cipher.iv

        # Pad plaintext to block size (8 bytes for DES3)
        padded_data = pad(plaintext.encode(), DES3.block_size)

        # Encrypt
        ciphertext = cipher.encrypt(padded_data)

        # Return base64 encoded ciphertext and IV
        return (
            base64.b64encode(ciphertext).decode('utf-8'),
            base64.b64encode(iv).decode('utf-8')
        )

    def decrypt(self, ciphertext: str, iv: str) -> str:
        """
        Decrypt data using 3DES.

        VULNERABLE: Relies on deprecated 3DES algorithm.
        """
        # Decode from base64
        ciphertext_bytes = base64.b64decode(ciphertext)
        iv_bytes = base64.b64decode(iv)

        # VULNERABILITY: 3DES decryption
        cipher = DES3.new(self.key, DES3.MODE_CBC, iv=iv_bytes)

        # Decrypt and unpad
        padded_plaintext = cipher.decrypt(ciphertext_bytes)
        plaintext = unpad(padded_plaintext, DES3.block_size)

        return plaintext.decode('utf-8')


class RC4Cipher:
    """
    RC4 stream cipher implementation.

    SECURITY ISSUES:
    - RC4 has known biases in its keystream
    - Vulnerable to various attacks (BEAST, CRIME, etc.)
    - Prohibited in TLS since 2015 (RFC 7465)
    - Not quantum-resistant

    Should use: ChaCha20 or AES-GCM
    """

    def __init__(self, key: bytes):
        """
        Initialize RC4 cipher.

        VULNERABLE: RC4 is completely broken and should never be used.
        """
        self.key = key

    def encrypt(self, plaintext: str) -> str:
        """
        Encrypt using RC4 stream cipher.

        EXTREMELY VULNERABLE: RC4 has statistical biases that can
        be exploited to recover plaintext.
        """
        # VULNERABILITY: RC4 cipher usage
        cipher = ARC4.new(self.key)
        ciphertext = cipher.encrypt(plaintext.encode())

        return base64.b64encode(ciphertext).decode('utf-8')

    def decrypt(self, ciphertext: str) -> str:
        """
        Decrypt using RC4 stream cipher.

        VULNERABLE: RC4 is cryptographically broken.
        """
        ciphertext_bytes = base64.b64decode(ciphertext)

        # VULNERABILITY: RC4 cipher usage
        cipher = ARC4.new(self.key)
        plaintext = cipher.decrypt(ciphertext_bytes)

        return plaintext.decode('utf-8')


class DataEncryptor:
    """
    Example data encryption service using vulnerable algorithms.
    """

    def __init__(self):
        # Generate weak keys (demonstrating bad practice)
        self.des3_key = get_random_bytes(24)  # 3DES key
        self.rc4_key = get_random_bytes(16)   # RC4 key

        self.des3_cipher = TripleDESEncryption(self.des3_key)
        self.rc4_cipher = RC4Cipher(self.rc4_key)

    def encrypt_sensitive_data(self, data: str, method: str = "3des") -> dict:
        """
        Encrypt sensitive data using specified method.

        VULNERABLE: Both methods are cryptographically weak.
        """
        if method == "3des":
            # VULNERABILITY: 3DES encryption
            ciphertext, iv = self.des3_cipher.encrypt(data)
            return {
                "method": "3DES-CBC",
                "ciphertext": ciphertext,
                "iv": iv
            }
        elif method == "rc4":
            # VULNERABILITY: RC4 encryption
            ciphertext = self.rc4_cipher.encrypt(data)
            return {
                "method": "RC4",
                "ciphertext": ciphertext
            }
        else:
            raise ValueError("Invalid encryption method")

    def decrypt_sensitive_data(self, encrypted_data: dict) -> str:
        """
        Decrypt sensitive data.

        VULNERABLE: Uses weak decryption algorithms.
        """
        method = encrypted_data["method"]

        if method == "3DES-CBC":
            return self.des3_cipher.decrypt(
                encrypted_data["ciphertext"],
                encrypted_data["iv"]
            )
        elif method == "RC4":
            return self.rc4_cipher.decrypt(encrypted_data["ciphertext"])
        else:
            raise ValueError("Unknown encryption method")


def encrypt_database_field(value: str, key: bytes) -> str:
    """
    Encrypt database field using 3DES.

    VULNERABLE: Database encryption should use modern algorithms.
    """
    # VULNERABILITY: 3DES for database encryption
    encryptor = TripleDESEncryption(key)
    ciphertext, iv = encryptor.encrypt(value)

    # Combine IV and ciphertext
    return f"{iv}:{ciphertext}"


if __name__ == "__main__":
    # Demonstrate vulnerable encryption methods
    encryptor = DataEncryptor()

    sensitive_data = "Credit Card: 4532-1234-5678-9010"

    # 3DES encryption (vulnerable)
    des3_encrypted = encryptor.encrypt_sensitive_data(sensitive_data, "3des")
    print(f"3DES Encrypted: {des3_encrypted}")

    # RC4 encryption (extremely vulnerable)
    rc4_encrypted = encryptor.encrypt_sensitive_data(sensitive_data, "rc4")
    print(f"RC4 Encrypted: {rc4_encrypted}")

    # Decrypt
    decrypted = encryptor.decrypt_sensitive_data(des3_encrypted)
    print(f"Decrypted: {decrypted}")
