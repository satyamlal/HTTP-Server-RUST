use flate2::{write::GzEncoder, Compression};
use std::io::Write;

pub fn compress_data(data: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}