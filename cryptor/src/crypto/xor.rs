pub fn xor_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());

    for (i, &byte) in data.iter().enumerate() {
        let key_byte = key[i % key.len()];
        out.push(byte ^ key_byte);
    }

    out
}

pub fn xor_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    // XOR est symétrique : decrypt = encrypt
    xor_encrypt(key, data)
}
