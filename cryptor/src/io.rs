use std::fs;

pub fn read_file(path: &str) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(path)?)
}

pub fn write_file(path: &str, data: &[u8]) -> anyhow::Result<()> {
    Ok(fs::write(path, data)?)
}