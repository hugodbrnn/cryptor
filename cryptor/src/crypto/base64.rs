use anyhow::Result;

pub fn encode(data: &[u8]) -> String {
    base64::encode(data)
}

pub fn decode(data: &str) -> Result<Vec<u8>> {
    let bytes = base64::decode(data)?;
    Ok(bytes)
}
