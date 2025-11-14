"""
Message Routes
WARNING: This code contains intentional security vulnerabilities for testing purposes
DO NOT use in production
"""

from flask import Flask, request, jsonify
import hashlib


app = Flask(__name__)

# VULNERABLE: Hardcoded secret key
app.secret_key = 'flask_secret_key_hardcoded_123456'


@app.route('/api/message/send', methods=['POST'])
def send_message():
    """
    Send encrypted message
    VULNERABLE: Uses weak encryption
    """
    data = request.json
    message = data.get('message')

    # VULNERABLE: MD5 hash for message ID
    message_id = hashlib.md5(message.encode()).hexdigest()

    # Encrypt message (using vulnerable crypto from other modules)
    # Implementation would use RC4 or custom crypto

    return jsonify({
        'message_id': message_id,
        'status': 'sent'
    })


@app.route('/api/message/verify', methods=['POST'])
def verify_message():
    """
    Verify message signature
    VULNERABLE: Uses weak signature verification
    """
    data = request.json
    message = data.get('message')

    # VULNERABLE: SHA-1 hash for verification
    message_hash = hashlib.sha1(message.encode()).hexdigest()

    return jsonify({
        'hash': message_hash,
        'verified': True
    })


@app.route('/api/auth/login', methods=['POST'])
def login():
    """
    User login endpoint
    VULNERABLE: Weak password hashing
    """
    data = request.json
    username = data.get('username')
    password = data.get('password')

    # VULNERABLE: MD5 password hash
    password_hash = hashlib.md5(password.encode()).hexdigest()

    # Check against stored hash (simplified)
    # In real app would query database

    return jsonify({
        'token': password_hash,  # Using hash as token (bad practice)
        'status': 'logged_in'
    })


if __name__ == '__main__':
    # VULNERABLE: Debug mode in production code
    app.run(debug=True)
