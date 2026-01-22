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
