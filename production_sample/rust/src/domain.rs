#[derive(Debug)]
pub enum DomainError {
    InvalidInput,
    Cancelled,
    Internal,
}

pub fn process(bytes: Vec<u8>, is_cancelled: impl Fn() -> bool) -> Result<Vec<u8>, DomainError> {
    if bytes.is_empty() {
        return Err(DomainError::InvalidInput);
    }

    let mut output = Vec::with_capacity(bytes.len());
    for chunk in bytes.chunks(4096) {
        if is_cancelled() {
            return Err(DomainError::Cancelled);
        }
        output.extend_from_slice(chunk);
    }
    Ok(output)
}
