use flate2::{write::GzEncoder, Compression};
use std::io::{self, Write};

pub fn compress_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let capacity = std::cmp::max(data.len() / 3, 64);
    let mut encoder = GzEncoder::new(Vec::with_capacity(capacity), Compression::default());
    
    encoder.write_all(data)?;
    encoder.finish()
}