use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use anyhow::{Result, anyhow};

pub struct EncryptionManager {
    cipher: Aes256Gcm,
}

impl EncryptionManager {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn from_env() -> Result<Self> {
        let key_str = std::env::var("ENCRYPTION_KEY")
            .map_err(|_| anyhow!("ENCRYPTION_KEY environment variable not set"))?;
        
        let key_bytes = general_purpose::STANDARD.decode(&key_str)
            .map_err(|_| anyhow!("Invalid base64 encoding for ENCRYPTION_KEY"))?;
        
        if key_bytes.len() != 32 {
            return Err(anyhow!("ENCRYPTION_KEY must be 32 bytes when decoded"));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes);
        Ok(Self::new(&key))
    }

    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let mut combined = Vec::new();
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(&combined))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        let combined = general_purpose::STANDARD.decode(encrypted_data)
            .map_err(|e| anyhow!("Base64 decode failed: {}", e))?;

        if combined.len() < 12 {
            return Err(anyhow!("Invalid encrypted data: too short"));
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext)
            .map_err(|e| anyhow!("Invalid UTF-8 in decrypted data: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = EncryptionManager::generate_key();
        let manager = EncryptionManager::new(&key);
        
        let original = "This is a secret message";
        let encrypted = manager.encrypt(original).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_different_nonces() {
        let key = EncryptionManager::generate_key();
        let manager = EncryptionManager::new(&key);
        
        let message = "Same message";
        let encrypted1 = manager.encrypt(message).unwrap();
        let encrypted2 = manager.encrypt(message).unwrap();
        
        // Same message should produce different ciphertext due to different nonces
        assert_ne!(encrypted1, encrypted2);
        
        // But both should decrypt to the same message
        assert_eq!(manager.decrypt(&encrypted1).unwrap(), message);
        assert_eq!(manager.decrypt(&encrypted2).unwrap(), message);
    }
}