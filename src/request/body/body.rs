pub(crate) fn parse(
    body: &mut Vec<u8>,
    data: &[u8],
    content_length: usize,
) -> Result<(bool, usize), std::io::Error> {
    let remaining = content_length.saturating_sub(body.len());
    if remaining == 0 {
        return Ok((true, 0));
    }

    let read = std::cmp::min(remaining, data.len());

    body.extend_from_slice(&data[..read]);

    let done = body.len() >= content_length;

    Ok((done, read))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_body() {
        let mut body: Vec<u8> = Vec::new();
        let data = b"Hello, World!";
        let content_length = data.len();

        let result = parse(&mut body, data, content_length);
        assert!(result.is_ok());

        let (done, consumed) = result.unwrap();
        assert!(done);
        assert_eq!(consumed, content_length);

        assert_eq!(body, data);
    }

    #[test]
    fn test_body_shorter_than_reported_content_length() {
        let mut body: Vec<u8> = Vec::new();
        let data = b"Hello, World!";
        let content_length = data.len() * 2;

        let result = parse(&mut body, data, content_length);
        assert!(result.is_ok());

        let (done, consumed) = result.unwrap();
        assert!(!done);
        assert_eq!(consumed, data.len());

        assert_eq!(body, data);
    }
}
