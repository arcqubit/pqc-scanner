import java.security.*;

public class CryptoExample {
    public static void main(String[] args) throws Exception {
        KeyPairGenerator keyGen = KeyPairGenerator.getInstance("RSA");
        keyGen.initialize(2048);
        KeyPair pair = keyGen.generateKeyPair();

        MessageDigest md = MessageDigest.getInstance("SHA-256");
    }
}
