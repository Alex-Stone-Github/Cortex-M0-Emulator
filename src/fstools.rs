/// Now that I think about it, this function is kind of useless
pub fn read_file_buffer(path: &str) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
    let buffer = std::fs::read(path)?;
    Ok(buffer.into_boxed_slice())
}
