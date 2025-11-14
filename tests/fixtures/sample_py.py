import hashlib
from Crypto.PublicKey import RSA, DSA

md5_hash = hashlib.md5(data).hexdigest()
sha1_hash = hashlib.sha1(data).hexdigest()

rsa_key = RSA.generate(2048)
dsa_key = DSA.generate(2048)
