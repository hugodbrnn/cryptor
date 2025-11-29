mod cli;
mod io;
mod crypto;

use crypto::aes;
use crypto::chacha;
use crypto::xor;
use crypto::base64;

fn main() -> anyhow::Result<()> {
    // === CLI officiel ===
    let args = cli::parse_args();
    cli::run(args)?;

    // === TESTS LOCAUX (facultatifs) ===
    // Commenter ou supprimer si tu veux utiliser uniquement la CLI.

    // --- TEST AES ---
    let encrypted_aes = aes::encrypt("hello", b"secret message")?;
    let decrypted_aes = aes::decrypt("hello", &encrypted_aes)?;
    println!("AES decrypted = {}", String::from_utf8_lossy(&decrypted_aes));

    // --- TEST CHACHA ---
    let encrypted_chacha = chacha::encrypt("password", b"hello chacha")?;
    let decrypted_chacha = chacha::decrypt("password", &encrypted_chacha)?;
    println!("ChaCha decrypted = {}", String::from_utf8_lossy(&decrypted_chacha));

    // --- TEST XOR ---
    let key = b"KEY";
    let encrypted_xor = xor::xor_encrypt(key, b"hello xor");
    let decrypted_xor = xor::xor_decrypt(key, &encrypted_xor);
    println!("XOR decrypted = {}", String::from_utf8_lossy(&decrypted_xor));

    // --- TEST BASE64 ---
    let encoded_b64 = base64::encode(b"hello base64");
    let decoded_b64 = base64::decode(&encoded_b64)?;
    println!("Base64 decoded = {}", String::from_utf8_lossy(&decoded_b64));

    Ok(())
}
