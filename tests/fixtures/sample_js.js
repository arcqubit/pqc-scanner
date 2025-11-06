const crypto = require('crypto');

const { publicKey, privateKey } = crypto.generateKeyPairSync('rsa', {
    modulusLength: 2048,
});

const ecdh = crypto.createECDH('secp256k1');
const keys = ecdh.generateKeys();

const hash = crypto.createHash('md5');
