use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce
};
use chacha20poly1305::aead::{Aead, KeyInit};

use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use anyhow::{Result, anyhow};
use rand::RngCore;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const PBKDF2_ITERS: u32 = 100_000;

// ---------------------------
// Key derivation (PBKDF2-HMAC-SHA256)
// ---------------------------
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

// ---------------------------
// Encrypt with ChaCha20-Poly1305
// Output format:  salt || nonce || ciphertext
// ---------------------------
pub fn encrypt(password: &str, data: &[u8]) -> Result<Vec<u8>> {
    // Salt random
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);

    // Key derivation
    let key_bytes = derive_key(password, &salt);
    let key = Key::from_slice(&key_bytes);

    // Nonce random
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Cipher
    let cipher = ChaCha20Poly1305::new(key);
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|_| anyhow!("ChaCha20 encryption failed"))?;

    // salt || nonce || ciphertext
    let mut output = Vec::new();
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

// ---------------------------
// Decrypt
// ---------------------------
pub fn decrypt(password: &str, data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < SALT_LEN + NONCE_LEN {
        return Err(anyhow!("Corrupted ciphertext"));
    }

    let salt = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let key_bytes = derive_key(password, salt);
    let key = Key::from_slice(&key_bytes);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = ChaCha20Poly1305::new(key);
    let decrypted = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Invalid password or corrupted data"))?;

    Ok(decrypted)
}
