use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn encode(data: &[u8]) -> String {
    STANDARD.encode(data)
}

pub fn decode(data: &str) -> Result<Vec<u8>> {
    let bytes = STANDARD
        .decode(data)
        .map_err(|_| anyhow::anyhow!("Invalid Base64 data"))?;
    Ok(bytes)
}
