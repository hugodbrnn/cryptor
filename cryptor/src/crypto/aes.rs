use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, KeyInit};
use rand::rngs::OsRng;
use rand::RngCore;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use anyhow::{Result, anyhow};

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32; // AES-256
const PBKDF2_ITERS: u32 = 100_000;

fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        salt,
        PBKDF2_ITERS,
        &mut key,
    );
    key
}

pub fn encrypt(password: &str, data: &[u8]) -> Result<Vec<u8>> {
    // 1) Génération du salt
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    // 2) Dérivation de la clé
    let key_bytes = derive_key(password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // 3) Génération du nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4) AES-GCM
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|_| anyhow!("AES-GCM encryption failed"))?;

    // 5) Format final = salt || nonce || ciphertext
    let mut result = Vec::new();
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

pub fn decrypt(password: &str, data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < SALT_LEN + NONCE_LEN {
        return Err(anyhow!("Invalid encrypted data"));
    }

    // 1) Extraction
    let salt = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    // 2) Dérivation de la clé
    let key_bytes = derive_key(password, salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // 3) AES-GCM
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Invalid password or corrupted data"))?;

    Ok(plaintext)
}
