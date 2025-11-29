pub fn xor_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

pub fn xor_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    // XOR encrypt = XOR decrypt
    xor_encrypt(key, data)
}
