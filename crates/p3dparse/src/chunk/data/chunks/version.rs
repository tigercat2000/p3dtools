use super::super::parse_trait::Parse;
use crate::{chunk::types::ChunkType, Result};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionData {
    pub version: u32,
}

impl Parse for VersionData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(VersionData {
            version: bytes.get_u32_le(),
        })
    }
}
