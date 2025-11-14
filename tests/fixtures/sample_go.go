package main

import (
    "crypto/rsa"
    "crypto/ecdsa"
    "crypto/dh"
)

func main() {
    privateKey, _ := rsa.GenerateKey(rand.Reader, 2048)
    ecdsaKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
    params := dh.GenerateParameters(2048)
}
