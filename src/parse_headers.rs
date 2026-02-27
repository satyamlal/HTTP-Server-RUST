use std::{
    collections::HashMap,
    io::{BufRead, BufReader, ErrorKind, Read},
};

// to prevet DDOS attacks
const MAX_HEADER_LINE_LEN: u64 = 8192; //8KB max per line
const MAX_HEADERS: usize = 100; // Limiting total headers parsed

pub fn parse_headers(buf_reader: &mut impl BufRead) -> Result<HashMap<String, String>, Error> {
    let mut headers = HashMap::new();

    for _ in 0..MAX_HEADERS {
        let mut line = String::new();

        let bytes_read = buf_readers.by_ref().take(MAX_HEADER_LINE_LEN).read_line(&mut line)?; // Guard against OOM

        if bytes_read == 0 {
            break;
        }

        if bytes_read as u64 == MAX_HEADER_LINE_LEN && !line.ends_with("\n") {
            return Err(Error::new(ErrorKind::InvalidData, "Header line exceeded maximum length!"));
        }

        let trimmed = line.trim();

        if trimmed.is_empty() {
            return Ok(headers);
        }

        if let Some((key, value)) = line.split_once(":") {
            let header_key = key.trim().to_lowercase();
            let header_value = value.trim().to_string();

            // handling duplicate headers
            headers.entry(header_key)
                .and_modify(|existing_val| {
                    existing_val.push_str(", ");
                    existing_val.push_str(&header_value);
                })
                .or_insert(header_value);
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Malformed HTTP Header"));
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Too Many headers in request!"));
}