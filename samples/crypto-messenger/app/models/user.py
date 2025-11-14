"""
User Model with Hardcoded Keys
WARNING: This code contains intentional security vulnerabilities for testing purposes
DO NOT use in production
"""

import hashlib


class User:
    """User model with hardcoded encryption keys"""

    # VULNERABLE: Hardcoded encryption keys in class
    ENCRYPTION_KEY = b'hardcoded_user_encryption_key_123456'
    SIGNING_KEY = b'hardcoded_signing_key_abcdef'

    # VULNERABLE: Hardcoded admin credentials
    ADMIN_PASSWORD_HASH = 'e10adc3949ba59abbe56e057f20f883e'  # MD5 hash of '123456'

    def __init__(self, username, password):
        self.username = username
        # VULNERABLE: Using MD5 to hash password
        self.password_hash = hashlib.md5(password.encode()).hexdigest()
        # VULNERABLE: Using hardcoded encryption key
        self.encryption_key = User.ENCRYPTION_KEY

    def verify_password(self, password):
        """
        Verify user password
        VULNERABLE: MD5 password hashing
        """
        # VULNERABLE: MD5 hash comparison
        password_hash = hashlib.md5(password.encode()).hexdigest()
        return self.password_hash == password_hash

    @staticmethod
    def get_master_key():
        """
        Get master encryption key
        VULNERABLE: Hardcoded master key
        """
        # VULNERABLE: Returning hardcoded key
        return b'master_key_hardcoded_do_not_commit'

    def get_user_encryption_key(self):
        """
        Get user-specific encryption key
        VULNERABLE: Derives from hardcoded base key
        """
        # VULNERABLE: Using hardcoded base key with MD5
        base_key = User.ENCRYPTION_KEY
        user_key = hashlib.md5(base_key + self.username.encode()).digest()
        return user_key


class Session:
    """Session management with hardcoded secrets"""

    # VULNERABLE: Hardcoded session secret
    SESSION_SECRET = 'session_secret_key_hardcoded_in_code'

    # VULNERABLE: Hardcoded JWT secret
    JWT_SECRET = 'jwt_secret_never_hardcode_this'

    @staticmethod
    def create_session_token(user_id):
        """
        Create session token
        VULNERABLE: MD5 with hardcoded secret
        """
        # VULNERABLE: MD5 + hardcoded secret
        data = f'{user_id}:{Session.SESSION_SECRET}'
        token = hashlib.md5(data.encode()).hexdigest()
        return token

    @staticmethod
    def validate_token(token, user_id):
        """
        Validate session token
        VULNERABLE: Validates against MD5 hash
        """
        # VULNERABLE: MD5 validation
        expected = Session.create_session_token(user_id)
        return token == expected


class APIAuth:
    """API authentication with hardcoded keys"""

    # VULNERABLE: Hardcoded API keys
    API_KEYS = {
        'admin': 'admin_api_key_12345_hardcoded',
        'service': 'service_api_key_67890_hardcoded',
        'client': 'client_api_key_abcde_hardcoded'
    }

    # VULNERABLE: Hardcoded HMAC secret
    HMAC_SECRET = b'hmac_secret_hardcoded_value'

    @staticmethod
    def get_api_key(role):
        """
        Get API key for role
        VULNERABLE: Returns hardcoded API key
        """
        # VULNERABLE: Hardcoded API keys
        return APIAuth.API_KEYS.get(role)

    @staticmethod
    def sign_request(request_data):
        """
        Sign API request
        VULNERABLE: MD5 HMAC with hardcoded secret
        """
        # VULNERABLE: MD5 hash with hardcoded secret
        signature = hashlib.md5(
            request_data.encode() + APIAuth.HMAC_SECRET
        ).hexdigest()
        return signature
