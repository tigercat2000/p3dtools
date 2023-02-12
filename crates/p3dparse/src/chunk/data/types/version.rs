use crate::{
    chunk::{data::parse_trait::Parse, types::ChunkType},
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub version: u32,
}

impl Parse for Version {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Version {
            version: bytes.get_u32_le(),
        })
    }
}
