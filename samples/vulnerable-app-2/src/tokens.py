"""
Token Generation Module - Contains ECDSA signatures and weak RNG
VULNERABILITIES:
- ECDSA (quantum-vulnerable to Shor's algorithm)
- Weak random number generation
"""

import hashlib
import random
import time
from ecdsa import SigningKey, VerifyingKey, SECP256k1, BadSignatureError
from typing import Tuple, Optional
import secrets


class ECDSATokenSigner:
    """
    Token signing using ECDSA (Elliptic Curve Digital Signature Algorithm).

    SECURITY ISSUES:
    - ECDSA is completely broken by Shor's algorithm on quantum computers
    - Provides 128-bit classical security, but 0 bits quantum security
    - Should be replaced with post-quantum signature schemes

    Should use: Dilithium, Falcon, or SPHINCS+ (NIST PQC standards)
    """

    def __init__(self):
        """
        Initialize ECDSA signing.

        VULNERABLE: ECDSA offers no quantum resistance.
        """
        # VULNERABILITY: ECDSA key generation (secp256k1 curve)
        self.private_key = SigningKey.generate(curve=SECP256k1)
        self.public_key = self.private_key.get_verifying_key()

    def sign_token(self, token_data: str) -> Tuple[str, bytes]:
        """
        Sign token using ECDSA.

        VULNERABLE: Shor's algorithm can recover private key from
        public key in polynomial time on quantum computer.
        """
        # Hash the token data
        token_hash = hashlib.sha256(token_data.encode()).digest()

        # VULNERABILITY: ECDSA signature
        signature = self.private_key.sign(token_hash)

        return token_data, signature

    def verify_token(self, token_data: str, signature: bytes) -> bool:
        """
        Verify ECDSA signature.

        VULNERABLE: Signature scheme broken by quantum computers.
        """
        try:
            token_hash = hashlib.sha256(token_data.encode()).digest()

            # VULNERABILITY: ECDSA verification
            self.public_key.verify(signature, token_hash)
            return True

        except BadSignatureError:
            return False

    def export_public_key(self) -> bytes:
        """Export public key for verification."""
        return self.public_key.to_string()

    @staticmethod
    def verify_with_public_key(token_data: str, signature: bytes,
                               public_key_bytes: bytes) -> bool:
        """
        Verify signature with provided public key.

        VULNERABLE: ECDSA verification is quantum-vulnerable.
        """
        try:
            public_key = VerifyingKey.from_string(public_key_bytes, curve=SECP256k1)
            token_hash = hashlib.sha256(token_data.encode()).digest()

            # VULNERABILITY: ECDSA signature verification
            public_key.verify(signature, token_hash)
            return True

        except (BadSignatureError, Exception):
            return False


class WeakRandomGenerator:
    """
    Demonstrates weak random number generation.

    SECURITY ISSUES:
    - Uses predictable PRNG (Mersenne Twister)
    - Seeded with time (predictable)
    - Not cryptographically secure

    Should use: secrets module or os.urandom()
    """

    def __init__(self, seed: Optional[int] = None):
        """
        Initialize weak RNG.

        VULNERABLE: Mersenne Twister is not cryptographically secure.
        """
        if seed is None:
            # VULNERABILITY: Predictable seed based on time
            seed = int(time.time())

        # VULNERABILITY: Using random module instead of secrets
        random.seed(seed)

    def generate_token(self, length: int = 32) -> str:
        """
        Generate token using weak RNG.

        VULNERABLE: Tokens are predictable if seed is known.
        """
        chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"

        # VULNERABILITY: Using random.choice instead of secrets.choice
        token = ''.join(random.choice(chars) for _ in range(length))

        return token

    def generate_session_key(self) -> int:
        """
        Generate session key using weak RNG.

        VULNERABLE: Session keys should be cryptographically random.
        """
        # VULNERABILITY: random.randint is predictable
        return random.randint(1000000, 9999999)

    def generate_nonce(self) -> str:
        """
        Generate nonce using weak RNG.

        VULNERABLE: Nonces must be unpredictable for security.
        """
        # VULNERABILITY: Weak random nonce generation
        return str(random.randint(0, 2**32 - 1))


class TokenManager:
    """
    Manages authentication tokens with multiple vulnerabilities.
    """

    def __init__(self):
        self.signer = ECDSATokenSigner()
        self.rng = WeakRandomGenerator()

    def create_auth_token(self, user_id: str) -> dict:
        """
        Create authentication token with ECDSA signature.

        VULNERABLE: Combines ECDSA (quantum-vulnerable) with
        weak random number generation.
        """
        # VULNERABILITY: Weak random token generation
        random_part = self.rng.generate_token(16)

        # Create token data
        timestamp = str(int(time.time()))
        token_data = f"{user_id}:{timestamp}:{random_part}"

        # VULNERABILITY: ECDSA signature
        token, signature = self.signer.sign_token(token_data)

        return {
            "token": token,
            "signature": signature.hex(),
            "public_key": self.signer.export_public_key().hex()
        }

    def verify_auth_token(self, token_data: dict) -> bool:
        """
        Verify authentication token.

        VULNERABLE: Relies on quantum-vulnerable ECDSA.
        """
        signature = bytes.fromhex(token_data["signature"])
        public_key = bytes.fromhex(token_data["public_key"])

        # VULNERABILITY: ECDSA verification
        return ECDSATokenSigner.verify_with_public_key(
            token_data["token"],
            signature,
            public_key
        )

    def generate_api_key(self) -> str:
        """
        Generate API key using weak RNG.

        VULNERABLE: API keys should be cryptographically random.
        """
        # VULNERABILITY: Weak random API key
        prefix = "sk_live_"
        random_part = self.rng.generate_token(32)

        return f"{prefix}{random_part}"

    def generate_reset_token(self, email: str) -> str:
        """
        Generate password reset token.

        VULNERABLE: Reset tokens must be unpredictable.
        """
        # VULNERABILITY: Weak random reset token
        timestamp = str(int(time.time()))
        random_part = self.rng.generate_token(24)

        # VULNERABILITY: Weak hash for token
        data = f"{email}:{timestamp}:{random_part}"
        return hashlib.md5(data.encode()).hexdigest()


def generate_cryptographic_key(bit_length: int = 128) -> bytes:
    """
    Generate cryptographic key using weak RNG.

    VULNERABLE: Should use secrets.token_bytes() instead.
    """
    # VULNERABILITY: Using random.getrandbits instead of secrets
    key_int = random.getrandbits(bit_length)
    key_bytes = key_int.to_bytes(bit_length // 8, byteorder='big')

    return key_bytes


if __name__ == "__main__":
    # Demonstrate vulnerabilities
    print("=== ECDSA Token Signing (Quantum Vulnerable) ===")
    manager = TokenManager()

    # Create token with ECDSA signature
    auth_token = manager.create_auth_token("user456")
    print(f"Token: {auth_token['token']}")
    print(f"Signature: {auth_token['signature'][:32]}...")

    # Verify token
    is_valid = manager.verify_auth_token(auth_token)
    print(f"Token valid: {is_valid}")

    print("\n=== Weak Random Generation ===")
    weak_rng = WeakRandomGenerator(seed=12345)  # Predictable seed!

    # Generate weak tokens
    weak_token = weak_rng.generate_token()
    print(f"Weak token: {weak_token}")

    session_key = weak_rng.generate_session_key()
    print(f"Weak session key: {session_key}")

    # Generate API key with weak randomness
    api_key = manager.generate_api_key()
    print(f"Weak API key: {api_key}")
