use crate::{
    bytes_ext::BufResult,
    chunk::{data::parse_trait::Parse, types::ChunkType},
    Result,
};
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BreakableObject {
    pub typ: u32,
    pub count: u32,
}

impl Parse for BreakableObject {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(BreakableObject {
            typ: bytes.safe_get_u32_le()?,
            count: bytes.safe_get_u32_le()?,
        })
    }
}
