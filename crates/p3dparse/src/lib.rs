use bytes::{Buf, Bytes};
use eyre::Context;
use num_enum::{IntoPrimitive, TryFromPrimitive};

mod chunk;
mod result;
use result::Result;

use crate::chunk::Chunk;

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, IntoPrimitive, TryFromPrimitive)]
pub enum FileTypes {
    RZ = 0x0000_5A52,
    CompressedPure3DBigEndian = 0x5033_445A,
    Pure3DBigEndian = 0x5033_44FF,
    CompressedPure3D = 0x5A44_3350,
    Pure3D = 0xFF44_3350,
}

pub fn parse_file(mut file: Bytes) -> Result<()> {
    let mut file_clone = file.clone();
    let file_type =
        FileTypes::try_from(file_clone.get_u32_le()).context("Unrecognized file format")?;

    println!("File type: {:?}", file_type);

    let root_chunk = Chunk::parse(&mut file, None)?;

    println!("Root Chunk: {:?}", root_chunk);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_types() {
        match FileTypes::try_from(0x00005A52) {
            Ok(filetype) => assert_eq!(filetype, FileTypes::RZ),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_l1r1() {
        let bytes = Bytes::from_static(include_bytes!("../test_data/l1r1.p3d"));
        parse_file(bytes).unwrap();
    }
}
