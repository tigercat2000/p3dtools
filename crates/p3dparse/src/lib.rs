pub use bytes::Bytes;
use bytes_ext::BufResult;
use eyre::Context;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use result::Result;

mod bytes_ext;
pub mod chunk;
mod result;
use crate::chunk::Chunk;

#[cfg(test)]
mod tests;

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, IntoPrimitive, TryFromPrimitive)]
pub enum FileTypes {
    RZ = 0x0000_5A52,
    CompressedPure3DBigEndian = 0x5033_445A,
    Pure3DBigEndian = 0x5033_44FF,
    CompressedPure3D = 0x5A44_3350,
    Pure3D = 0xFF44_3350,
}

pub fn parse_file(mut file: Bytes) -> Result<Vec<Chunk>> {
    let mut file_clone = file.clone();
    let file_type =
        FileTypes::try_from(file_clone.safe_get_u32_le()?).context("Unrecognized file format")?;

    println!("File type: {:?}", file_type);

    Chunk::parse_root(&mut file)
}
