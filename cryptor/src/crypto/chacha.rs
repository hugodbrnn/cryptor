use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;
use anyhow::{Result, anyhow};

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12; // 96-bit nonce, recommandé
const KEY_LEN: usize = 32;   // 256 bits
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
    // 1) Salt aléatoire
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);

    // 2) Dérivation de clé
    let key_bytes = derive_key(password, &salt);
    let key = Key::<ChaCha20Poly1305>::from_slice(&key_bytes);

    // 3) Nonce aléatoire
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 4) Chiffrement
    let cipher = ChaCha20Poly1305::new(key);
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|_| anyhow!("ChaCha20-Poly1305 encryption failed"))?;

    // 5) Format de sortie : salt || nonce || ciphertext
    let mut out = Vec::new();
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);

    Ok(out)
}

pub fn decrypt(password: &str, data: &[u8]) -> Result<Vec<u8>> {
    // Format attendu : [salt (16)] [nonce (12)] [ciphertext+tag]
    if data.len() < SALT_LEN + NONCE_LEN {
        return Err(anyhow!("Invalid encrypted data"));
    }

    let salt = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    // Dérive la clé
    let key_bytes = derive_key(password, salt);
    let key = Key::<ChaCha20Poly1305>::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Déchiffre
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Invalid password or corrupted data"))?;

    Ok(plaintext)
}
