use std::fs;

use bytes::Bytes;
use p3dparse::parse_file;

pub fn main() {
    let bytes = Bytes::from_static(include_bytes!("../test_data/l1r1.p3d"));
    match parse_file(bytes) {
        Ok(root) => {
            let json = serde_json::to_string_pretty(&root).unwrap();
            fs::write("./out.json", json).expect("Failed to write output");
        }
        Err(e) => {
            eprintln!("Failed to parse l1r1.p3d");
            eprintln!("{:?}", e)
        }
    }
}
