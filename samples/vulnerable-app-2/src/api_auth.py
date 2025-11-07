"""
API Authentication Module - Contains MD5-based token generation
VULNERABILITY: Uses MD5 for API token generation (quantum-vulnerable)
"""

import hashlib
import time
from typing import Optional


class APIAuthenticator:
    """
    Handles API authentication using MD5 hashes.

    SECURITY ISSUE: MD5 is cryptographically broken and vulnerable to:
    - Collision attacks
    - Rainbow table attacks
    - Quantum attacks (Grover's algorithm)

    Should use: HMAC-SHA256 or better yet, quantum-safe alternatives
    """

    def __init__(self, secret_key: str):
        self.secret_key = secret_key

    def generate_api_token(self, user_id: str) -> str:
        """
        Generate API token using MD5.

        VULNERABLE: MD5 is not collision-resistant and can be broken
        by quantum computers using Grover's algorithm in O(2^64) operations.
        """
        timestamp = str(int(time.time()))
        payload = f"{user_id}:{timestamp}:{self.secret_key}"

        # VULNERABILITY: MD5 hash function
        token_hash = hashlib.md5(payload.encode()).hexdigest()

        return f"{user_id}:{timestamp}:{token_hash}"

    def validate_token(self, token: str) -> bool:
        """
        Validate API token using MD5.

        VULNERABLE: Token validation relies on broken MD5 algorithm.
        """
        try:
            user_id, timestamp, provided_hash = token.split(':')

            # Check if token is expired (24 hour validity)
            if int(time.time()) - int(timestamp) > 86400:
                return False

            # Recreate hash and compare
            payload = f"{user_id}:{timestamp}:{self.secret_key}"
            expected_hash = hashlib.md5(payload.encode()).hexdigest()

            # VULNERABILITY: Comparing MD5 hashes
            return provided_hash == expected_hash

        except (ValueError, IndexError):
            return False

    def hash_password(self, password: str) -> str:
        """
        Hash password using MD5.

        EXTREMELY VULNERABLE: Never use MD5 for password hashing!
        """
        # VULNERABILITY: MD5 for password storage
        return hashlib.md5(password.encode()).hexdigest()

    def verify_password(self, password: str, password_hash: str) -> bool:
        """Verify password against MD5 hash."""
        return self.hash_password(password) == password_hash


class SessionManager:
    """
    Manages user sessions with weak hashing.

    VULNERABILITY: Uses MD5 for session token generation
    """

    @staticmethod
    def create_session_id(username: str, ip_address: str) -> str:
        """
        Create session ID using MD5.

        VULNERABLE: Predictable session IDs using weak hash function.
        """
        timestamp = str(time.time())
        data = f"{username}:{ip_address}:{timestamp}"

        # VULNERABILITY: MD5 for session ID
        session_id = hashlib.md5(data.encode()).hexdigest()

        return session_id

    @staticmethod
    def create_csrf_token(session_id: str) -> str:
        """
        Create CSRF token using MD5.

        VULNERABLE: CSRF tokens should be unpredictable.
        """
        # VULNERABILITY: MD5 for CSRF token
        return hashlib.md5(session_id.encode()).hexdigest()


def create_api_signature(data: str, secret: str) -> str:
    """
    Create API request signature using MD5.

    VULNERABLE: Should use HMAC-SHA256 or quantum-safe alternative.
    """
    message = f"{data}:{secret}"

    # VULNERABILITY: MD5 for message authentication
    return hashlib.md5(message.encode()).hexdigest()


if __name__ == "__main__":
    # Example usage demonstrating vulnerabilities
    auth = APIAuthenticator("super_secret_key")

    # Generate vulnerable API token
    token = auth.generate_api_token("user123")
    print(f"Generated MD5-based token: {token}")

    # Hash password with MD5 (extremely bad practice)
    password_hash = auth.hash_password("MyPassword123!")
    print(f"MD5 password hash: {password_hash}")

    # Create vulnerable session
    session_id = SessionManager.create_session_id("alice", "192.168.1.1")
    print(f"MD5 session ID: {session_id}")
